use std::io;

use niri_ipc::{Action, SizeChange};

use super::trait_def::IpcClient;

pub fn set_window_width_percent(
    client: &mut impl IpcClient,
    window_id: u64,
    percent: f64,
) -> io::Result<()> {
    client.run_action(Action::SetWindowWidth {
        id: Some(window_id),
        change: SizeChange::SetProportion(percent),
    })
}

pub fn set_window_width_fixed_best_effort(
    client: &mut impl IpcClient,
    window_id: u64,
    width: i32,
) -> io::Result<()> {
    client.run_action_best_effort(Action::SetWindowWidth {
        id: Some(window_id),
        change: SizeChange::SetFixed(width.max(1)),
    })
}

pub fn set_window_height_fixed_best_effort(
    client: &mut impl IpcClient,
    window_id: u64,
    height: i32,
) -> io::Result<()> {
    client.run_action_best_effort(Action::SetWindowHeight {
        id: Some(window_id),
        change: SizeChange::SetFixed(height.max(1)),
    })
}
