mod common;

use std::fs;
use std::io;

use common::{remove_file_if_exists, unique_temp_state_path};
use niri_master_layout::state_file::load_layout_state;

#[test]
fn load_invalid_json_returns_invalid_data() {
    let path = unique_temp_state_path();
    fs::write(&path, "{not-json").expect("write should succeed");

    let error = load_layout_state(&path).expect_err("invalid json should fail");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);

    remove_file_if_exists(&path);
}
