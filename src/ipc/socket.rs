use std::io;

use niri_ipc::socket::Socket;
use niri_ipc::{Action, Request, Response, Window};

use super::client::IpcClient;

pub struct SocketClient {
    socket: Socket,
}

impl SocketClient {
    pub fn connect() -> io::Result<Self> {
        Ok(Self {
            socket: Socket::connect()?,
        })
    }

    fn request(&mut self, request: Request) -> io::Result<Response> {
        self.socket.send(request)?.map_err(io::Error::other)
    }
}

impl IpcClient for SocketClient {
    fn focused(&mut self) -> io::Result<Option<Window>> {
        match self.request(Request::FocusedWindow)? {
            Response::FocusedWindow(window) => Ok(window),
            other => Err(unexpected("FocusedWindow", other)),
        }
    }

    fn windows(&mut self) -> io::Result<Vec<Window>> {
        match self.request(Request::Windows)? {
            Response::Windows(windows) => Ok(windows),
            other => Err(unexpected("Windows", other)),
        }
    }

    fn action(&mut self, action: Action) -> io::Result<()> {
        match self.request(Request::Action(action))? {
            Response::Handled => Ok(()),
            other => Err(unexpected("Action", other)),
        }
    }

    fn action_best_effort(&mut self, action: Action) -> io::Result<()> {
        drop(self.socket.send(Request::Action(action))?);
        Ok(())
    }
}

fn unexpected(kind: &str, response: Response) -> io::Error {
    io::Error::other(format!("unexpected response for {kind}: {response:?}"))
}
