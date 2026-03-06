use std::io;
use std::path::Path;

use niri_ipc::Window;

use crate::state_file::model::{SavedLayoutState, SavedWindowSize};
use crate::window_utils::tiled_pos;

use super::atomic::write_atomic;

pub fn save_layout_state(
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
