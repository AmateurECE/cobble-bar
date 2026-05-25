use std::{env, io};

use futures::{stream, Stream};

use super::{Event, Workspaces};

pub struct Sway {
    connection: swayipc::Connection,
}

#[derive(thiserror::Error, Debug)]
pub enum SwayError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Var(#[from] env::VarError),
    #[error("{0}")]
    Sway(#[from] swayipc::Error),
}

impl Sway {
    pub async fn new_connection() -> Result<Self, SwayError> {
        let connection = swayipc::Connection::new()?;

        Ok(Sway { connection })
    }

    pub async fn workspaces(&mut self) -> Result<Workspaces, SwayError> {
        let workspaces = self.connection.get_workspaces()?;

        let active_workspace = workspaces
            .iter()
            .find(|w| w.visible)
            .unwrap()
            .num
            .try_into()
            .unwrap();
        let workspaces = workspaces
            .into_iter()
            .map(|w| w.num.try_into().unwrap())
            .collect();
        Ok(Workspaces {
            workspaces,
            active_workspace,
        })
    }

    pub async fn into_event_stream(self) -> Result<impl Stream<Item = Event>, SwayError> {
        let stream = self.connection.subscribe([swayipc::EventType::Workspace])?;
        Ok(stream::unfold(stream, |mut stream| async move {
            let Ok(event) = stream.next()? else {
                return None;
            };
            let swayipc::Event::Workspace(event) = event else {
                return None;
            };
            match event.change {
                swayipc::WorkspaceChange::Empty => Some((
                    Event::DestroyWorkspace(
                        event.current.unwrap().num.unwrap().try_into().unwrap(),
                    ),
                    stream,
                )),
                swayipc::WorkspaceChange::Focus => Some((
                    Event::Workspace(event.current.unwrap().num.unwrap().try_into().unwrap()),
                    stream,
                )),
                swayipc::WorkspaceChange::Init => Some((
                    Event::CreateWorkspace(event.current.unwrap().num.unwrap().try_into().unwrap()),
                    stream,
                )),
                _ => todo!(),
            }
        }))
    }
}
