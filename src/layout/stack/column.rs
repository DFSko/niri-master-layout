use std::io;

use niri_ipc::Window;

use crate::ipc::IpcClient;
use crate::layout::tiled_pos;

pub fn focused_stack_column(client: &mut impl IpcClient) -> io::Result<Option<usize>> {
    let Some(current) = client.focused()? else {
        return Ok(None);
    };

    Ok(current
        .layout
        .pos_in_scrolling_layout
        .map(|(column, _)| column))
}

pub fn stack_column_state(
    windows: &[Window],
    workspace_id: u64,
    stack_column: usize,
) -> (usize, bool) {
    let mut count_in_column = 0usize;
    let mut has_right_columns = false;

    for window in windows {
        let Some((column, _)) = tiled_pos(window, workspace_id) else {
            continue;
        };

        if column == stack_column {
            count_in_column += 1;
        } else if column > stack_column {
            has_right_columns = true;
        }
    }

    (count_in_column, has_right_columns)
}
