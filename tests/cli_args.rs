use niri_master_layout::app::AppCommand;

fn parse_args_from(args: impl IntoIterator<Item = String>) -> std::io::Result<AppCommand> {
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

fn invalid_args() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "usage: niri-master-layout [grow-master|shrink-master]",
    )
}

#[test]
fn parse_args_defaults_to_toggle() {
    let command = parse_args_from(Vec::<String>::new()).expect("parse should succeed");

    assert_eq!(command, AppCommand::Toggle);
}

#[test]
fn parse_args_accepts_grow_master() {
    let command = parse_args_from([String::from("grow-master")]).expect("parse should succeed");

    assert_eq!(command, AppCommand::GrowMaster);
}

#[test]
fn parse_args_accepts_shrink_master() {
    let command = parse_args_from([String::from("shrink-master")]).expect("parse should succeed");

    assert_eq!(command, AppCommand::ShrinkMaster);
}

#[test]
fn parse_args_rejects_unknown_action() {
    let error = parse_args_from([String::from("unknown")]).expect_err("parse should fail");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn parse_args_rejects_multiple_actions() {
    let error = parse_args_from([String::from("grow-master"), String::from("extra")])
        .expect_err("parse should fail");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}
