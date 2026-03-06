mod common;

use std::fs;

use niri_master_layout::state_file::{load_layout_state, save_layout_state};

use common::{make_floating_window, make_tiled_window};

#[test]
fn save_and_load_roundtrip_with_workspace() {
    let path = unique_temp_path();
    let windows = vec![
        make_tiled_window(10, 42, 2, 1, 1200, 800),
        make_tiled_window(20, 42, 1, 1, 800, 600),
        make_floating_window(30, 42, 500, 300),
    ];

    save_layout_state(&path, 10, 42, &windows).expect("save should succeed");
    let state = load_layout_state(&path).expect("load should succeed");

    assert_eq!(state.workspace_id, 42);
    assert_eq!(state.master_id, 10);
    assert_eq!(state.windows.len(), 2);
    assert_eq!(state.windows[0].id, 20);
    assert_eq!(state.windows[1].id, 10);

    let _ = fs::remove_file(path);
}

fn unique_temp_path() -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    std::env::temp_dir().join(format!("niri-master-layout-test-{nanos}.state"))
}
