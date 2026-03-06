mod common;

use niri_master_layout::layout::restore_columns;
use niri_master_layout::state_file::SavedWindowSize;

use common::fake_client::FakeClient;
use common::windows::make_tiled_window;

#[test]
fn restore_columns_noop_when_target_already_in_place() {
    let snapshots = vec![vec![
        make_tiled_window(10, 1, 2, 1, 800, 600),
        make_tiled_window(20, 1, 3, 1, 800, 600),
    ]];

    let saved = vec![SavedWindowSize {
        id: 10,
        width: 800,
        height: 600,
        column: 2,
        row: 1,
    }];

    let mut client = FakeClient::with_windows(snapshots);
    restore_columns(&mut client, &saved).expect("restore should succeed");

    assert!(client.best_effort_actions.is_empty());
}
