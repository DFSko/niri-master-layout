use std::collections::HashMap;

use niri_ipc::Window;

use crate::layout::tiled_pos;

use super::types::{ColumnSnapshot, RestoreTarget};

pub fn snapshot_target(
    windows: &[Window],
    target_id: u64,
    target_column: usize,
    desired_by_id: &HashMap<u64, usize>,
) -> Option<RestoreTarget> {
    let current = windows.iter().find(|window| window.id == target_id)?;
    let (current_column, _) = current.layout.pos_in_scrolling_layout?;
    let workspace_id = current.workspace_id?;

    let mut column_window_ids = windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id).and_then(|(window_column, _)| {
                (window_column == current_column).then_some(window.id)
            })
        })
        .collect::<Vec<_>>();
    let has_foreign_windows = column_window_ids.iter().any(|window_id| {
        desired_by_id.get(window_id).copied().unwrap_or(usize::MAX) != target_column
    });

    column_window_ids.sort_unstable();

    Some(RestoreTarget {
        has_foreign_windows,
        column: ColumnSnapshot {
            index: current_column,
            window_ids: column_window_ids,
        },
    })
}
