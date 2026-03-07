use std::io;

use niri_ipc::{Action, SizeChange};

use super::client::IpcClient;

pub fn focus(client: &mut impl IpcClient, window_id: u64) -> io::Result<()> {
    client.action(Action::FocusWindow { id: window_id })
}

pub fn focus_best_effort(client: &mut impl IpcClient, window_id: u64) -> io::Result<()> {
    client.action_best_effort(Action::FocusWindow { id: window_id })
}

pub fn focus_with_width(
    client: &mut impl IpcClient,
    window_id: u64,
    percent: f64,
) -> io::Result<()> {
    focus(client, window_id)?;
    set_width_percent(client, window_id, percent)
}

pub fn set_width_percent(
    client: &mut impl IpcClient,
    window_id: u64,
    percent: f64,
) -> io::Result<()> {
    client.action(Action::SetWindowWidth {
        id: Some(window_id),
        change: SizeChange::SetProportion(percent),
    })
}

pub fn set_width_percent_for_windows(
    client: &mut impl IpcClient,
    window_ids: &[u64],
    percent: f64,
) -> io::Result<()> {
    for &window_id in window_ids {
        set_width_percent(client, window_id, percent)?;
    }

    Ok(())
}

pub fn set_width(client: &mut impl IpcClient, window_id: u64, width: i32) -> io::Result<()> {
    client.action_best_effort(Action::SetWindowWidth {
        id: Some(window_id),
        change: SizeChange::SetFixed(width.max(1)),
    })
}

pub fn set_height(client: &mut impl IpcClient, window_id: u64, height: i32) -> io::Result<()> {
    client.action_best_effort(Action::SetWindowHeight {
        id: Some(window_id),
        change: SizeChange::SetFixed(height.max(1)),
    })
}
