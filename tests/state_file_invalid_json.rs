mod common;

use std::fs;
use std::io;

use niri_master_layout::state_file::load_layout_state;

#[test]
fn load_invalid_json_returns_invalid_data() {
    let path = unique_temp_path();
    fs::write(&path, "{not-json").expect("write should succeed");

    let error = load_layout_state(&path).expect_err("invalid json should fail");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);

    let _ = fs::remove_file(path);
}

fn unique_temp_path() -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    std::env::temp_dir().join(format!("niri-master-layout-test-{nanos}.state"))
}
