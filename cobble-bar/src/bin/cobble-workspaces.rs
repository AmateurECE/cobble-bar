use cobble_bar::workspace::{sway::Sway, Workspaces};
use futures::{pin_mut, StreamExt};

#[derive(serde::Serialize)]
struct State {
    content: String,
}

impl From<Workspaces> for State {
    fn from(value: Workspaces) -> Self {
        let max_workspace = value.workspaces.iter().max().unwrap();
        let content = (1..max_workspace + 1)
            .map(|i| {
                if i == value.active_workspace {
                    ACTIVE_WORKSPACE
                } else if value.workspaces.contains(&i) {
                    INACTIVE_WORKSPACE
                } else {
                    MISSING_WORKSPACE
                }
            })
            .collect::<Vec<&str>>()
            .join("  ");
        State { content }
    }
}

const ACTIVE_WORKSPACE: &str = "\u{ebb4}";
const INACTIVE_WORKSPACE: &str = "\u{eba5}";
const MISSING_WORKSPACE: &str = "\u{ebb5}";

fn print_state(workspaces: Workspaces) -> anyhow::Result<()> {
    let state = State::from(workspaces);
    println!("{}", serde_json::to_string(&state)?);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let mut window_manager = Sway::new_connection().await?;
    let mut state = window_manager.workspaces().await?;
    print_state(state.clone())?;

    let events = window_manager.into_event_stream().await?;
    pin_mut!(events);

    while let Some(event) = events.next().await {
        state = state.transition(event);
        print_state(state.clone())?;
    }
    Ok(())
}
