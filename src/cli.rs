use std::ffi::OsString;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppCommand {
    Toggle,
    GrowMaster,
    ShrinkMaster,
}

pub fn parse_args() -> io::Result<AppCommand> {
    parse_args_from(std::env::args_os().skip(1))
}

pub fn parse_args_from(args: impl IntoIterator<Item = OsString>) -> io::Result<AppCommand> {
    let mut args = args.into_iter();
    let Some(action) = args.next() else {
        return Ok(AppCommand::Toggle);
    };

    if args.next().is_some() {
        return Err(invalid_args());
    }

    match action.to_str() {
        Some("grow-master") => Ok(AppCommand::GrowMaster),
        Some("shrink-master") => Ok(AppCommand::ShrinkMaster),
        _ => Err(invalid_args()),
    }
}

fn invalid_args() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "usage: niri-master-layout [grow-master|shrink-master]",
    )
}
