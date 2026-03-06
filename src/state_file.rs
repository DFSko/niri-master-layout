use std::ffi::OsString;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use niri_ipc::Window;
use serde::{Deserialize, Serialize};

use crate::window_utils::tiled_pos;

const STATE_FILE_NAME_PREFIX: &str = "niri-master-layout";

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SavedWindowSize {
    pub id: u64,
    pub width: i32,
    pub height: i32,
    pub column: usize,
    pub row: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SavedLayoutState {
    pub workspace_id: u64,
    pub master_id: u64,
    pub windows: Vec<SavedWindowSize>,
}

pub fn state_file_path(workspace_id: u64) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("{STATE_FILE_NAME_PREFIX}-{workspace_id}.state"));
    path
}

pub fn save_layout_state(
    path: &Path,
    master_id: u64,
    workspace_id: u64,
    windows: &[Window],
) -> io::Result<()> {
    let mut saved_windows: Vec<_> = windows
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

pub fn load_layout_state(path: &Path) -> io::Result<SavedLayoutState> {
    let text = fs::read_to_string(path)?;

    serde_json::from_str(&text).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid state: {error}"),
        )
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::test_support::{make_floating_window, make_tiled_window};

    fn unique_temp_path() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        std::env::temp_dir().join(format!("niri-master-layout-test-{nanos}.state"))
    }

    #[test]
    fn save_and_load_roundtrip_with_workspace() {
        let path = unique_temp_path();
        let workspace_id = 42;

        let windows = vec![
            make_tiled_window(10, workspace_id, 2, 1, 1200, 800),
            make_tiled_window(20, workspace_id, 1, 1, 800, 600),
            make_floating_window(30, workspace_id, 500, 300),
        ];

        save_layout_state(&path, 10, workspace_id, &windows).expect("save should succeed");
        let state = load_layout_state(&path).expect("load should succeed");

        assert_eq!(state.workspace_id, workspace_id);
        assert_eq!(state.master_id, 10);
        assert_eq!(state.windows.len(), 2);
        assert_eq!(state.windows[0].id, 20);
        assert_eq!(state.windows[1].id, 10);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_invalid_json_returns_invalid_data() {
        let path = unique_temp_path();
        fs::write(&path, "{not-json").expect("write should succeed");

        let error = load_layout_state(&path).expect_err("invalid json should fail");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn save_layout_state_ignores_other_workspace_windows() {
        let path = unique_temp_path();
        let workspace_id = 7;

        let windows = vec![
            make_tiled_window(1, workspace_id, 1, 1, 900, 700),
            make_tiled_window(2, workspace_id + 1, 1, 1, 900, 700),
        ];

        save_layout_state(&path, 1, workspace_id, &windows).expect("save should succeed");
        let state = load_layout_state(&path).expect("load should succeed");

        assert_eq!(state.windows.len(), 1);
        assert_eq!(state.windows[0].id, 1);

        let _ = fs::remove_file(path);
    }
}
