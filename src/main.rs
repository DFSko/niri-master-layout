use std::io;

use niri_master_layout::app;
use niri_master_layout::app::AppCommand;
use niri_master_layout::ipc::SocketClient;

fn main() -> io::Result<()> {
    let command = parse_args()?;
    let mut client = SocketClient::connect()?;
    app::run(&mut client, command)
}

fn parse_args() -> io::Result<AppCommand> {
    parse_args_from(std::env::args().skip(1))
}

fn parse_args_from(args: impl IntoIterator<Item = String>) -> io::Result<AppCommand> {
    let mut args = args.into_iter();
    let Some(action) = args.next() else {
        return Ok(AppCommand::Toggle);
    };

    if args.next().is_some() {
        return Err(invalid_args());
    }

    match action.as_str() {
        "grow-master" => Ok(AppCommand::GrowMaster),
        "shrink-master" => Ok(AppCommand::ShrinkMaster),
        _ => Err(invalid_args()),
    }
}

fn invalid_args() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "usage: niri-master-layout [grow-master|shrink-master]",
    )
}
