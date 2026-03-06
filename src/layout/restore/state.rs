use std::collections::HashMap;

use niri_ipc::Window;

use crate::window_utils::tiled_pos;

use super::types::{RestoreSnapshot, TargetRestoreState};

pub fn target_restore_state(
    windows: &[Window],
    target_id: u64,
    target_column: usize,
    desired_by_id: &HashMap<u64, usize>,
) -> Option<TargetRestoreState> {
    let current = windows.iter().find(|window| window.id == target_id)?;
    let (current_column, _) = current.layout.pos_in_scrolling_layout?;
    let workspace_id = current.workspace_id?;

    let mut has_foreign_windows = false;
    let mut column_window_ids = Vec::new();

    for window in windows {
        let Some((window_column, _)) = tiled_pos(window, workspace_id) else {
            continue;
        };
        if window_column != current_column {
            continue;
        }

        column_window_ids.push(window.id);
        let desired_column = desired_by_id.get(&window.id).copied().unwrap_or(usize::MAX);
        has_foreign_windows |= desired_column != target_column;
    }

    column_window_ids.sort_unstable();

    Some(TargetRestoreState {
        has_foreign_windows,
        snapshot: RestoreSnapshot {
            current_column,
            column_window_ids,
        },
    })
}
