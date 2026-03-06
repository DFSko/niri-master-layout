mod common;

use niri_master_layout::state_file::{load_layout_state, save_layout_state};

use common::windows::make_tiled_window;
use common::{remove_file_if_exists, unique_temp_state_path};

#[test]
fn save_layout_state_ignores_other_workspace_windows() {
    let path = unique_temp_state_path();
    let windows = vec![
        make_tiled_window(1, 7, 1, 1, 900, 700),
        make_tiled_window(2, 8, 1, 1, 900, 700),
    ];

    save_layout_state(&path, 1, 7, &windows).expect("save should succeed");
    let state = load_layout_state(&path).expect("load should succeed");

    assert_eq!(state.windows.len(), 1);
    assert_eq!(state.windows[0].id, 1);

    remove_file_if_exists(&path);
}
