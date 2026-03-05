use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use niri_ipc::Window;
use serde::{Deserialize, Serialize};

use crate::window_utils::tiled_pos;

const STATE_FILE_NAME: &str = "niri-master-layout.state";

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SavedWindowSize {
    pub id: u64,
    pub width: i32,
    pub height: i32,
    pub column: usize,
    pub row: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SavedLayoutState {
    pub master_id: u64,
    pub windows: Vec<SavedWindowSize>,
}

pub fn state_file_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(STATE_FILE_NAME);
    path
}

pub fn save_layout_state(
    path: &Path,
    master_id: u64,
    workspace_id: u64,
    windows: &[Window],
) -> io::Result<()> {
    let saved_windows = windows
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
        .collect();

    let state = SavedLayoutState {
        master_id,
        windows: saved_windows,
    };

    let text = serde_json::to_string(&state)
        .map_err(|error| io::Error::other(format!("failed to serialize state: {error}")))?;

    fs::write(path, text)
}

pub fn load_layout_state(path: &Path) -> io::Result<SavedLayoutState> {
    let text = fs::read_to_string(path)?;

    serde_json::from_str(&text)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, format!("invalid state: {error}")))
}
