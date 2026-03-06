use crate::window_utils::tiled_pos;

pub(super) fn stack_window_ids(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    stack_column: usize,
) -> Vec<u64> {
    windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id)
                .and_then(|(column, _)| (column == stack_column).then_some(window.id))
        })
        .collect()
}

pub(super) fn has_other_windows_in_column(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    column: usize,
    ignored_id: u64,
) -> bool {
    windows.iter().any(|window| {
        window.id != ignored_id
            && tiled_pos(window, workspace_id)
                .is_some_and(|(window_column, _)| window_column == column)
    })
}
