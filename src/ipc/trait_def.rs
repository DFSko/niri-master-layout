use std::io;

use niri_ipc::{Action, Window};

pub trait IpcClient {
    fn focused_window(&mut self) -> io::Result<Option<Window>>;
    fn windows(&mut self) -> io::Result<Vec<Window>>;
    fn run_action(&mut self, action: Action) -> io::Result<()>;
    fn run_action_best_effort(&mut self, action: Action) -> io::Result<()>;
}
