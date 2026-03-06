use std::io;

use niri_ipc::Action;

use crate::ipc::IpcClient;
use crate::layout::nearest_right_column_anchor;

use super::context::{FocusedContext, focus_master};

const MASTER_COLUMN_INDEX: usize = 1;

pub fn move_master_and_find_anchor(
    client: &mut impl IpcClient,
    context: &FocusedContext,
) -> io::Result<Option<u64>> {
    focus_master(client, context.master_id)?;
    client.run_action(Action::MoveColumnToIndex {
        index: MASTER_COLUMN_INDEX,
    })?;

    let all_windows_after = client.windows()?;
    let Some(master_after_move) = all_windows_after
        .iter()
        .find(|window| window.id == context.master_id)
    else {
        return Ok(None);
    };
    let Some((master_column, _)) = master_after_move.layout.pos_in_scrolling_layout else {
        return Ok(None);
    };

    let anchor = nearest_right_column_anchor(
        &all_windows_after,
        context.workspace_id,
        master_column,
        context.master_id,
    );

    if anchor.is_none() {
        eprintln!(
            "warn no_right_columns workspace_id={} master_id={}",
            context.workspace_id, context.master_id
        );
    }

    Ok(anchor)
}
