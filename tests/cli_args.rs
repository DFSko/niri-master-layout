use std::ffi::OsString;

use niri_master_layout::cli::{AppCommand, parse_args_from};

#[test]
fn parse_args_defaults_to_toggle() {
    let command = parse_args_from(Vec::<OsString>::new()).expect("parse should succeed");

    assert_eq!(command, AppCommand::Toggle);
}

#[test]
fn parse_args_accepts_grow_master() {
    let command = parse_args_from([OsString::from("grow-master")]).expect("parse should succeed");

    assert_eq!(command, AppCommand::GrowMaster);
}

#[test]
fn parse_args_accepts_shrink_master() {
    let command = parse_args_from([OsString::from("shrink-master")]).expect("parse should succeed");

    assert_eq!(command, AppCommand::ShrinkMaster);
}

#[test]
fn parse_args_rejects_unknown_action() {
    let error = parse_args_from([OsString::from("unknown")]).expect_err("parse should fail");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn parse_args_rejects_multiple_actions() {
    let error = parse_args_from([OsString::from("grow-master"), OsString::from("extra")])
        .expect_err("parse should fail");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}
