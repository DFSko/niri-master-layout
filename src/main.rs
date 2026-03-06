use std::io;

use niri_master_layout::app;
use niri_master_layout::cli;
use niri_master_layout::ipc::SocketClient;

fn main() -> io::Result<()> {
    let command = cli::parse_args()?;
    let mut client = SocketClient::connect()?;
    app::run(&mut client, command)
}
