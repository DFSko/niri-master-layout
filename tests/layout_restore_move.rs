mod common;

use niri_ipc::Action;
use niri_master_layout::layout::restore_columns;
use niri_master_layout::state::SavedWindowSize;

use common::fake_client::FakeClient;
use common::windows::make_tiled_window;

#[test]
fn restore_columns_move_when_single_non_target_window() {
    let snapshots = vec![
        vec![make_tiled_window(10, 1, 1, 1, 800, 600)],
        vec![make_tiled_window(10, 1, 2, 1, 800, 600)],
    ];

    let saved = vec![SavedWindowSize {
        id: 10,
        width: 800,
        height: 600,
        column: 2,
        row: 1,
    }];

    let mut client = FakeClient::with_windows(snapshots);
    restore_columns(&mut client, &saved).expect("restore should succeed");

    assert_eq!(client.best_effort_actions.len(), 2);
    assert!(matches!(client.best_effort_actions[0], Action::FocusWindow { id } if id == 10));
    assert!(
        matches!(client.best_effort_actions[1], Action::MoveColumnToIndex { index } if index == 2)
    );
}
