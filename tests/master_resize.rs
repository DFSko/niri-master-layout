mod common;

use niri_ipc::{Action, SizeChange};
use niri_master_layout::app;
use niri_master_layout::cli::AppCommand;
use niri_master_layout::state_file::{save_layout_state, state_file_path};

use common::fake_client::FakeClient;
use common::remove_file_if_exists;
use common::windows::make_tiled_window;

#[test]
fn grow_master_changes_width_when_master_layout_is_active() {
    let workspace_id = 7001;
    let master_id = 10;
    let state_path = state_file_path(workspace_id);
    let windows = vec![
        make_tiled_window(master_id, workspace_id, 1, 1, 600, 900),
        make_tiled_window(11, workspace_id, 2, 1, 400, 450),
        make_tiled_window(12, workspace_id, 2, 2, 400, 450),
    ];
    save_layout_state(&state_path, master_id, workspace_id, &windows).expect("save should succeed");
    let mut focused = make_tiled_window(99, workspace_id, 3, 1, 200, 900);
    focused.is_focused = true;
    let mut client = FakeClient::default();
    client.focused.push_back(Ok(Some(focused)));
    client.windows.push_back(Ok(windows));

    app::run(&mut client, AppCommand::GrowMaster).expect("run should succeed");

    assert_eq!(client.actions.len(), 4);
    assert!(matches!(client.actions[0], Action::FocusWindow { id } if id == master_id));
    assert!(matches!(
        client.actions[1],
        Action::SetWindowWidth {
            id: Some(id),
            change: SizeChange::SetProportion(width),
        } if id == master_id && (width - 70.0).abs() < f64::EPSILON
    ));
    assert!(matches!(
        client.actions[2],
        Action::SetWindowWidth {
            id: Some(id),
            change: SizeChange::SetProportion(width),
        } if id == 11 && (width - 30.0).abs() < f64::EPSILON
    ));
    assert!(matches!(
        client.actions[3],
        Action::SetWindowWidth {
            id: Some(id),
            change: SizeChange::SetProportion(width),
        } if id == 12 && (width - 30.0).abs() < f64::EPSILON
    ));

    remove_file_if_exists(&state_path);
}

#[test]
fn resize_is_noop_when_master_layout_is_not_active() {
    let workspace_id = 7002;
    let master_id = 10;
    let state_path = state_file_path(workspace_id);
    let saved_windows = vec![
        make_tiled_window(master_id, workspace_id, 1, 1, 600, 900),
        make_tiled_window(11, workspace_id, 2, 1, 400, 450),
    ];
    save_layout_state(&state_path, master_id, workspace_id, &saved_windows)
        .expect("save should succeed");
    let live_windows = vec![
        make_tiled_window(master_id, workspace_id, 1, 1, 600, 900),
        make_tiled_window(11, workspace_id, 1, 2, 600, 450),
    ];
    let mut focused = make_tiled_window(master_id, workspace_id, 1, 1, 600, 900);
    focused.is_focused = true;
    let mut client = FakeClient::default();
    client.focused.push_back(Ok(Some(focused)));
    client.windows.push_back(Ok(live_windows));

    app::run(&mut client, AppCommand::ShrinkMaster).expect("run should succeed");

    assert!(client.actions.is_empty());

    remove_file_if_exists(&state_path);
}

#[test]
fn shrink_master_clamps_width_to_minimum() {
    let workspace_id = 7003;
    let master_id = 10;
    let state_path = state_file_path(workspace_id);
    let windows = vec![
        make_tiled_window(master_id, workspace_id, 1, 1, 100, 900),
        make_tiled_window(11, workspace_id, 2, 1, 900, 900),
    ];
    save_layout_state(&state_path, master_id, workspace_id, &windows).expect("save should succeed");
    let mut focused = make_tiled_window(master_id, workspace_id, 1, 1, 100, 900);
    focused.is_focused = true;
    let mut client = FakeClient::default();
    client.focused.push_back(Ok(Some(focused)));
    client.windows.push_back(Ok(windows));

    app::run(&mut client, AppCommand::ShrinkMaster).expect("run should succeed");

    assert_eq!(client.actions.len(), 3);
    assert!(matches!(client.actions[0], Action::FocusWindow { id } if id == master_id));
    assert!(matches!(
        client.actions[1],
        Action::SetWindowWidth {
            id: Some(id),
            change: SizeChange::SetProportion(width),
        } if id == master_id && (width - 10.0).abs() < f64::EPSILON
    ));
    assert!(matches!(
        client.actions[2],
        Action::SetWindowWidth {
            id: Some(id),
            change: SizeChange::SetProportion(width),
        } if id == 11 && (width - 90.0).abs() < f64::EPSILON
    ));

    remove_file_if_exists(&state_path);
}
