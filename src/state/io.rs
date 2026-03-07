use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::Path;

use niri_ipc::Window;

use crate::layout::tiled_pos;

use super::types::{SavedLayoutState, SavedWindowSize};

pub fn load_state(path: &Path) -> io::Result<SavedLayoutState> {
    let text = fs::read_to_string(path)?;

    serde_json::from_str(&text).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid state: {error}"),
        )
    })
}

pub fn save_state(
    path: &Path,
    master_id: u64,
    workspace_id: u64,
    windows: &[Window],
) -> io::Result<()> {
    let mut saved_windows = collect_saved_windows(windows, workspace_id);
    saved_windows.sort_by_key(|window| (window.column, window.row, window.id));

    let state = SavedLayoutState {
        workspace_id,
        master_id,
        windows: saved_windows,
    };

    let text = serde_json::to_string(&state)
        .map_err(|error| io::Error::other(format!("failed to serialize state: {error}")))?;

    write_atomic(path, text.as_bytes())
}

fn collect_saved_windows(windows: &[Window], workspace_id: u64) -> Vec<SavedWindowSize> {
    windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id).map(|(column, row)| SavedWindowSize {
                id: window.id,
                width: window.layout.window_size.0.max(1),
                height: window.layout.window_size.1.max(1),
                column,
                row,
            })
        })
        .collect()
}

fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut temp_path = path.to_path_buf();
    let mut file_name = path
        .file_name()
        .map(OsString::from)
        .unwrap_or_else(|| OsString::from("state"));
    file_name.push(format!(".tmp-{}", std::process::id()));
    temp_path.set_file_name(file_name);

    fs::write(&temp_path, bytes)?;
    match fs::rename(&temp_path, path) {
        Ok(()) => Ok(()),
        Err(error) => {
            let _ = fs::remove_file(&temp_path);
            Err(error)
        }
    }
}
