use std::{collections::VecDeque, env, path::PathBuf};

use futures::{stream, Stream};
use regex::Regex;
use tokio::{
    io::{AsyncReadExt as _, AsyncWriteExt as _},
    net::UnixSocket,
};

use crate::flatten;

use super::{Event, Workspaces};

lazy_static::lazy_static! {
    static ref WORKSPACE: Regex = Regex::new("^workspace ID ([0-9])").unwrap();
    static ref EVENT: Regex = Regex::new("^([a-z0-9]*)>>(.*)$").unwrap();
}

#[derive(Debug)]
pub enum HyprlandError {
    DoesNotExist,
    InvalidEvent,
}

impl std::error::Error for HyprlandError {}
impl std::fmt::Display for HyprlandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoesNotExist => {
                write!(f, "The Hyprland socket directory doesn't exist")
            }
            HyprlandError::InvalidEvent => {
                write!(f, "Received garbage on the Hyprland event socket")
            }
        }
    }
}

fn parse_workspace(text: &str) -> Option<(u32, String)> {
    let mut items = text.split(',');
    let id: u32 = items.next()?.parse().ok()?;
    let name = items.next()?.to_string();
    Some((id, name))
}

fn parse_event(text: &str) -> Option<Event> {
    let captures = EVENT.captures(text)?;
    let event_name = captures.get(1)?;
    let event_content = captures.get(2)?;
    let (id, _) = parse_workspace(event_content.as_str())?;
    let event = match event_name.as_str() {
        "createworkspacev2" => Event::CreateWorkspace(id),
        "workspacev2" => Event::Workspace(id),
        "destroyworkspacev2" => Event::DestroyWorkspace(id),
        _ => return None,
    };
    Some(event)
}

impl core::str::FromStr for Event {
    type Err = HyprlandError;

    fn from_str(event: &str) -> Result<Self, Self::Err> {
        parse_event(event).ok_or(HyprlandError::InvalidEvent)
    }
}

/// Return Some if the directory at the provided path exists.
fn directory_if_exists(path: String) -> Option<PathBuf> {
    let directory = PathBuf::from(path);
    match directory.exists() {
        true => Some(directory),
        false => None,
    }
}

/// Extract the workspace id from the response of a hyprctl-style query on the compositor.
fn workspace_id(line: &str) -> Option<u32> {
    let captures = WORKSPACE.captures(line)?;
    let id = captures.get(1)?;
    let id: u32 = id.as_str().parse().ok()?;
    Some(id)
}

/// Proxy class for interacting with the Hyprland server.
pub struct Hyprland {
    socket_directory: PathBuf,
}

impl Hyprland {
    /// Create a connection to the Hyprland server.
    pub async fn new_connection() -> anyhow::Result<Self> {
        let instance_signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")?;
        let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR")?;

        // NOTE: In a recent version of hyprland, sockets were moved from /tmp/hypr to $XDG_RUNTIME_DIR/hypr
        let recent_socket_path = format!("/{}/hypr/{}", xdg_runtime_dir, instance_signature);
        let legacy_socket_path = format!("/tmp/hypr/{}", instance_signature);
        let socket_directory = directory_if_exists(recent_socket_path)
            .or_else(|| directory_if_exists(legacy_socket_path))
            .ok_or(HyprlandError::DoesNotExist)?;

        Ok(Hyprland { socket_directory })
    }

    async fn request(&self, path: &[u8]) -> anyhow::Result<String> {
        let socket_path = self.socket_directory.join(".socket.sock");
        let socket = UnixSocket::new_stream()?;
        let mut control = socket.connect(socket_path).await?;

        control.write_all(path).await?;
        let mut response = String::new();
        control.read_to_string(&mut response).await?;

        Ok(response)
    }

    /// Query the state of the existing workspaces.
    pub async fn workspaces(&mut self) -> anyhow::Result<Workspaces> {
        let active_workspace = self.active_workspace().await?;
        let response = self.request(b"/workspaces\0".as_ref()).await?;
        let workspaces = response
            .lines()
            .filter_map(workspace_id)
            .collect::<Vec<u32>>();
        Ok(Workspaces {
            workspaces,
            active_workspace,
        })
    }

    async fn active_workspace(&mut self) -> anyhow::Result<u32> {
        let response = self.request(b"/activeworkspace\0".as_ref()).await?;
        Ok(response.lines().find_map(workspace_id).unwrap())
    }

    /// Consume the Hyprland proxy object and produce a stream that emits Hyprland events.
    pub async fn into_event_stream(self) -> anyhow::Result<impl Stream<Item = Event>> {
        let socket_path = self.socket_directory.join(".socket2.sock");
        let socket = UnixSocket::new_stream()?;
        let stream = socket.connect(socket_path).await?;

        Ok(flatten(stream::unfold(stream, |mut stream| async move {
            let mut events = VecDeque::new();
            while events.is_empty() {
                let mut buffer = [0u8; 128];
                let length = stream.read(&mut buffer).await.ok()?;
                let event = String::from_utf8_lossy(&buffer[..length]);
                events = event
                    .trim_matches(char::from(0))
                    // FIXME: I'm sure I'm being bitten by missing partial messages here.
                    // Switching very rapidly between workspace causes me to drop events, which
                    // makes it seem like I'm in the wrong workspace.
                    .lines()
                    .filter_map(|line| line.parse().ok())
                    .collect::<VecDeque<Event>>();
            }
            Some((events, stream))
        })))
    }
}
