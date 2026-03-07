use std::io;
use std::path::PathBuf;

use niri_ipc::Action;

use crate::ipc::{IpcClient, set_width_percent};

pub struct WorkspaceContext {
    pub master_id: u64,
    pub workspace_id: u64,
    pub state_path: PathBuf,
}

pub fn current_workspace(
    client: &mut impl IpcClient,
    state_path_for_workspace: impl FnOnce(u64) -> PathBuf,
) -> io::Result<Option<WorkspaceContext>> {
    let Some(window) = client.focused()? else {
        return Ok(None);
    };
    let Some(workspace_id) = window.workspace_id else {
        return Ok(None);
    };

    Ok(Some(WorkspaceContext {
        master_id: window.id,
        workspace_id,
        state_path: state_path_for_workspace(workspace_id),
    }))
}

pub fn focus(client: &mut impl IpcClient, window_id: u64) -> io::Result<()> {
    client.action(Action::FocusWindow { id: window_id })
}

pub fn focus_with_width(client: &mut impl IpcClient, window_id: u64, width_percent: f64) -> io::Result<()> {
    focus(client, window_id)?;
    set_width_percent(client, window_id, width_percent)
}
