use std::io;

use niri_ipc::Action;

use crate::ipc::{IpcClient, set_width_percent};
use crate::layout::tiled_pos;

use super::column::focused_stack_column;

const STACK_WINDOW_WIDTH_PERCENT: f64 = 40.0;

pub fn style_stack_column(client: &mut impl IpcClient, workspace_id: u64) -> io::Result<()> {
    let Some(stack_column) = focused_stack_column(client)? else {
        return Ok(());
    };

    let windows = client.windows()?;
    let mut stack_windows = collect_stack_window_ids(&windows, workspace_id, stack_column);

    stack_windows.sort_by_key(|(row, _)| *row);
    for (_, window_id) in stack_windows {
        set_width_percent(client, window_id, STACK_WINDOW_WIDTH_PERCENT)?;
        client.action(Action::ResetWindowHeight {
            id: Some(window_id),
        })?;
    }

    Ok(())
}

fn collect_stack_window_ids(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    stack_column: usize,
) -> Vec<(usize, u64)> {
    windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id).map(|(column, row)| (column, row, window.id))
        })
        .filter(|(column, _, _)| *column == stack_column)
        .map(|(_, row, id)| (row, id))
        .collect()
}
