mod app;
mod ipc;
mod layout;
mod state_file;
mod window_utils;

#[cfg(test)]
mod test_support;

use std::io;

use crate::ipc::SocketClient;

fn main() -> io::Result<()> {
    let mut client = SocketClient::connect()?;
    app::run(&mut client)
}
