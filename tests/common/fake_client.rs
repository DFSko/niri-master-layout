use std::collections::VecDeque;
use std::io;

use niri_ipc::{Action, Window};
use niri_master_layout::ipc::IpcClient;

#[derive(Default)]
pub struct FakeClient {
    pub focused: VecDeque<io::Result<Option<Window>>>,
    pub windows: VecDeque<io::Result<Vec<Window>>>,
    pub actions: Vec<Action>,
    pub best_effort_actions: Vec<Action>,
    pub fail_run_action: bool,
}

impl FakeClient {
    pub fn with_windows(states: Vec<Vec<Window>>) -> Self {
        Self {
            windows: states.into_iter().map(Ok).collect(),
            ..Self::default()
        }
    }
}

impl IpcClient for FakeClient {
    fn focused(&mut self) -> io::Result<Option<Window>> {
        self.focused.pop_front().unwrap_or(Ok(None))
    }

    fn windows(&mut self) -> io::Result<Vec<Window>> {
        self.windows.pop_front().unwrap_or_else(|| Ok(Vec::new()))
    }

    fn action(&mut self, action: Action) -> io::Result<()> {
        self.actions.push(action);
        if self.fail_run_action {
            return Err(io::Error::other("action failed"));
        }
        Ok(())
    }

    fn action_best_effort(&mut self, action: Action) -> io::Result<()> {
        self.best_effort_actions.push(action);
        Ok(())
    }
}
