use std::io;
use std::path::PathBuf;

use crate::ipc::IpcClient;

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
