use std::io;
use std::path::PathBuf;

use niri_ipc::Action;

use crate::ipc::{IpcClient, set_window_width_percent};

pub struct FocusedContext {
    pub master_id: u64,
    pub workspace_id: u64,
    pub state_path: PathBuf,
}

pub fn focused_context(
    client: &mut impl IpcClient,
    state_path_for_workspace: impl FnOnce(u64) -> PathBuf,
) -> io::Result<Option<FocusedContext>> {
    let Some(window) = client.focused_window()? else {
        return Ok(None);
    };
    let Some(workspace_id) = window.workspace_id else {
        return Ok(None);
    };

    Ok(Some(FocusedContext {
        master_id: window.id,
        workspace_id,
        state_path: state_path_for_workspace(workspace_id),
    }))
}

pub fn focus_master(client: &mut impl IpcClient, master_id: u64) -> io::Result<()> {
    client.run_action(Action::FocusWindow { id: master_id })
}

pub fn focus_master_with_width(
    client: &mut impl IpcClient,
    master_id: u64,
    width_percent: f64,
) -> io::Result<()> {
    focus_master(client, master_id)?;
    set_window_width_percent(client, master_id, width_percent)
}
