use std::io;

use niri_master_layout::app;
use niri_master_layout::ipc::SocketClient;

fn main() -> io::Result<()> {
    let mut client = SocketClient::connect()?;
    app::run(&mut client)
}
