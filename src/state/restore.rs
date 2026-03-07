use std::io;
use std::path::Path;

use niri_ipc::Action;

use crate::ipc::{IpcClient, set_height, set_width};
use crate::layout::restore_columns;
use crate::state::load_state;

use super::cleanup::remove_file_if_exists;

pub fn restore(client: &mut impl IpcClient, path: &Path) -> io::Result<()> {
    let state = load_state(path)?;

    restore_columns(client, &state.windows)?;

    for window in state.windows {
        set_width(client, window.id, window.width)?;
        set_height(client, window.id, window.height)?;
    }

    client.action_best_effort(Action::FocusWindow {
        id: state.master_id,
    })?;
    remove_file_if_exists(path)
}

pub fn restore_if_present(client: &mut impl IpcClient, path: &Path) -> io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    match restore(client, path) {
        Ok(()) => Ok(true),
        Err(error) => {
            eprintln!(
                "error restore_layout path={} reason={error}",
                path.display()
            );
            if let Err(remove_error) = remove_file_if_exists(path) {
                eprintln!(
                    "error remove_stale_state path={} reason={remove_error}",
                    path.display()
                );
            }
            Ok(false)
        }
    }
}
