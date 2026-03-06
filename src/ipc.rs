use std::io;

use niri_ipc::socket::Socket;
use niri_ipc::{Action, Request, Response, SizeChange, Window};

pub trait IpcClient {
    fn focused_window(&mut self) -> io::Result<Option<Window>>;
    fn windows(&mut self) -> io::Result<Vec<Window>>;
    fn run_action(&mut self, action: Action) -> io::Result<()>;
    fn run_action_best_effort(&mut self, action: Action) -> io::Result<()>;
}

pub struct SocketClient {
    socket: Socket,
}

impl SocketClient {
    pub fn connect() -> io::Result<Self> {
        Ok(Self {
            socket: Socket::connect()?,
        })
    }

    fn send_request(&mut self, request: Request) -> io::Result<Response> {
        self.socket.send(request)?.map_err(io::Error::other)
    }
}

impl IpcClient for SocketClient {
    fn focused_window(&mut self) -> io::Result<Option<Window>> {
        match self.send_request(Request::FocusedWindow)? {
            Response::FocusedWindow(window) => Ok(window),
            other => Err(unexpected_response("FocusedWindow", other)),
        }
    }

    fn windows(&mut self) -> io::Result<Vec<Window>> {
        match self.send_request(Request::Windows)? {
            Response::Windows(windows) => Ok(windows),
            other => Err(unexpected_response("Windows", other)),
        }
    }

    fn run_action(&mut self, action: Action) -> io::Result<()> {
        match self.send_request(Request::Action(action))? {
            Response::Handled => Ok(()),
            other => Err(unexpected_response("Action", other)),
        }
    }

    fn run_action_best_effort(&mut self, action: Action) -> io::Result<()> {
        // Best effort: keep I/O failures fatal, but ignore niri-side action errors.
        if self.socket.send(Request::Action(action))?.is_err() {
            // Intentionally ignored.
        }
        Ok(())
    }
}

pub fn set_window_width_percent(
    client: &mut impl IpcClient,
    window_id: u64,
    percent: f64,
) -> io::Result<()> {
    client.run_action(Action::SetWindowWidth {
        id: Some(window_id),
        change: SizeChange::SetProportion(percent),
    })
}

pub fn set_window_width_fixed_best_effort(
    client: &mut impl IpcClient,
    window_id: u64,
    width: i32,
) -> io::Result<()> {
    client.run_action_best_effort(Action::SetWindowWidth {
        id: Some(window_id),
        change: SizeChange::SetFixed(width.max(1)),
    })
}

pub fn set_window_height_fixed_best_effort(
    client: &mut impl IpcClient,
    window_id: u64,
    height: i32,
) -> io::Result<()> {
    client.run_action_best_effort(Action::SetWindowHeight {
        id: Some(window_id),
        change: SizeChange::SetFixed(height.max(1)),
    })
}

fn unexpected_response(kind: &str, response: Response) -> io::Error {
    io::Error::other(format!("unexpected response for {kind}: {response:?}"))
}

#[cfg(test)]
pub mod testing {
    use super::*;
    use std::collections::VecDeque;

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
            let windows = states.into_iter().map(Ok).collect();
            Self {
                windows,
                ..Self::default()
            }
        }
    }

    impl IpcClient for FakeClient {
        fn focused_window(&mut self) -> io::Result<Option<Window>> {
            self.focused.pop_front().unwrap_or(Ok(None))
        }

        fn windows(&mut self) -> io::Result<Vec<Window>> {
            self.windows.pop_front().unwrap_or_else(|| Ok(Vec::new()))
        }

        fn run_action(&mut self, action: Action) -> io::Result<()> {
            self.actions.push(action);
            if self.fail_run_action {
                return Err(io::Error::other("run_action failed"));
            }
            Ok(())
        }

        fn run_action_best_effort(&mut self, action: Action) -> io::Result<()> {
            self.best_effort_actions.push(action);
            Ok(())
        }
    }
}
