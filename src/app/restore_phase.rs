use std::io;
use std::path::Path;

use crate::ipc::IpcClient;

use super::state_restore::{remove_file_if_exists, restore_layout_state};

pub fn try_restore_layout(client: &mut impl IpcClient, path: &Path) -> io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }

    match restore_layout_state(client, path) {
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
