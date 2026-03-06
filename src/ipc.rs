use std::io;

use niri_ipc::socket::Socket;
use niri_ipc::{Action, Request, Response, SizeChange, Window};

pub fn focused_window(socket: &mut Socket) -> io::Result<Option<Window>> {
    match send_request(socket, Request::FocusedWindow)? {
        Response::FocusedWindow(window) => Ok(window),
        other => Err(io::Error::other(format!(
            "unexpected response for FocusedWindow: {other:?}"
        ))),
    }
}

pub fn windows(socket: &mut Socket) -> io::Result<Vec<Window>> {
    match send_request(socket, Request::Windows)? {
        Response::Windows(windows) => Ok(windows),
        other => Err(io::Error::other(format!(
            "unexpected response for Windows: {other:?}"
        ))),
    }
}

pub fn run_action(socket: &mut Socket, action: Action) -> io::Result<()> {
    match send_request(socket, Request::Action(action))? {
        Response::Handled => Ok(()),
        other => Err(io::Error::other(format!(
            "unexpected response for Action: {other:?}"
        ))),
    }
}

pub fn run_action_best_effort(socket: &mut Socket, action: Action) -> io::Result<()> {
    let _ = socket.send(Request::Action(action))?;
    Ok(())
}

pub fn set_window_width_percent(
    socket: &mut Socket,
    window_id: u64,
    percent: f64,
) -> io::Result<()> {
    run_action(
        socket,
        Action::SetWindowWidth {
            id: Some(window_id),
            change: SizeChange::SetProportion(percent),
        },
    )
}

pub fn set_window_width_fixed_best_effort(
    socket: &mut Socket,
    window_id: u64,
    width: i32,
) -> io::Result<()> {
    run_action_best_effort(
        socket,
        Action::SetWindowWidth {
            id: Some(window_id),
            change: SizeChange::SetFixed(width.max(1)),
        },
    )
}

pub fn set_window_height_fixed_best_effort(
    socket: &mut Socket,
    window_id: u64,
    height: i32,
) -> io::Result<()> {
    run_action_best_effort(
        socket,
        Action::SetWindowHeight {
            id: Some(window_id),
            change: SizeChange::SetFixed(height.max(1)),
        },
    )
}

fn send_request(socket: &mut Socket, request: Request) -> io::Result<Response> {
    match socket.send(request)? {
        Ok(response) => Ok(response),
        Err(message) => Err(io::Error::other(message)),
    }
}
