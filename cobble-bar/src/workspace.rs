#[cfg(feature = "hyprland")]
pub mod hyprland;
#[cfg(feature = "sway")]
pub mod sway;

#[derive(Debug)]
pub enum Event {
    Workspace(u32),
    CreateWorkspace(u32),
    DestroyWorkspace(u32),
}

#[derive(Clone, Debug)]
pub struct Workspaces {
    pub workspaces: Vec<u32>,
    pub active_workspace: u32,
}

impl Workspaces {
    /// Transition the state of the workspaces on an event.
    pub fn transition(self, event: Event) -> Self {
        match event {
            Event::Workspace(id) => self.transition_active_workspace(id),
            Event::CreateWorkspace(id) => self.create_workspace(id),
            Event::DestroyWorkspace(id) => self.destroy_workspace(id),
        }
    }

    fn transition_active_workspace(mut self, id: u32) -> Self {
        self.active_workspace = id;
        self
    }

    fn create_workspace(mut self, id: u32) -> Self {
        self.workspaces.push(id);
        self
    }

    fn destroy_workspace(mut self, id: u32) -> Self {
        let index = self
            .workspaces
            .iter()
            .enumerate()
            .find_map(|(index, workspace_id)| {
                if id == *workspace_id {
                    Some(index)
                } else {
                    None
                }
            })
            .unwrap();
        self.workspaces.remove(index);
        self
    }
}
