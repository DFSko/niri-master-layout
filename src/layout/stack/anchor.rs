use niri_ipc::Window;

use crate::layout::tiled_pos;

pub fn nearest_right_column_anchor(
    windows: &[Window],
    workspace_id: u64,
    master_column: usize,
    master_id: u64,
) -> Option<u64> {
    windows
        .iter()
        .filter(|window| window.id != master_id)
        .filter_map(|window| {
            tiled_pos(window, workspace_id).map(|(column, row)| (column, row, window.id))
        })
        .filter(|(column, _, _)| *column > master_column)
        .min_by_key(|(column, row, _)| (*column, *row))
        .map(|(_, _, id)| id)
}
