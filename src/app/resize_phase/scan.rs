use std::collections::BTreeMap;

use crate::window_utils::tiled_pos;

pub(super) fn collect_columns_and_master_column(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    master_id: u64,
) -> Option<(BTreeMap<usize, i32>, Option<usize>)> {
    let mut column_widths = BTreeMap::<usize, i32>::new();
    let mut master_column = None;

    for window in windows {
        let Some((column, row)) = tiled_pos(window, workspace_id) else {
            continue;
        };

        column_widths
            .entry(column)
            .and_modify(|width| *width = (*width).max(tile_width(window)))
            .or_insert_with(|| tile_width(window));

        if window.id == master_id && (row != 1 || master_column.replace(column).is_some()) {
            return None;
        }
    }

    let leftmost_column = *column_widths.keys().next()?;
    (master_column == Some(leftmost_column)).then_some((column_widths, master_column))
}

pub(super) fn next_column_after(
    column_widths: &BTreeMap<usize, i32>,
    column: usize,
) -> Option<usize> {
    column_widths.keys().copied().find(|value| *value > column)
}

fn tile_width(window: &niri_ipc::Window) -> i32 {
    window.layout.tile_size.0.max(1.0) as i32
}
