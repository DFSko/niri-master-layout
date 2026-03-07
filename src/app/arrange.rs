use std::io;

use niri_ipc::{Action, ColumnDisplay};

use super::context::WorkspaceContext;
use crate::ipc::{IpcClient, focus, focus_with_width};
use crate::layout::{nearest_right_column_anchor, pull_windows_into_stack, style_stack_column};
use crate::state::{CleanupGuard, save_state};

const MASTER_WIDTH_PERCENT: f64 = 60.0;
const MAX_STACK_WINDOWS: usize = 3;
const MASTER_COLUMN_INDEX: usize = 1;

pub fn arrange(client: &mut impl IpcClient, context: &WorkspaceContext) -> io::Result<()> {
    let all_windows_before = client.windows()?;
    save_state(
        &context.state_path,
        context.master_id,
        context.workspace_id,
        &all_windows_before,
    )?;
    let mut state_cleanup = CleanupGuard::new(&context.state_path);

    focus(client, context.master_id)?;
    client.action(Action::MoveColumnToIndex {
        index: MASTER_COLUMN_INDEX,
    })?;

    let all_windows_after = client.windows()?;
    let Some(master_after_move) = all_windows_after
        .iter()
        .find(|window| window.id == context.master_id)
    else {
        return Ok(());
    };
    let Some((master_column, _)) = master_after_move.layout.pos_in_scrolling_layout else {
        return Ok(());
    };

    let Some(stack_anchor_id) = nearest_right_column_anchor(
        &all_windows_after,
        context.workspace_id,
        master_column,
        context.master_id,
    ) else {
        eprintln!(
            "warn no_right_columns workspace_id={} master_id={}",
            context.workspace_id, context.master_id
        );
        return Ok(());
    };

    state_cleanup.keep();
    focus_with_width(client, context.master_id, MASTER_WIDTH_PERCENT)?;

    client.action(Action::FocusWindow {
        id: stack_anchor_id,
    })?;
    client.action(Action::SetColumnDisplay {
        display: ColumnDisplay::Normal,
    })?;

    pull_windows_into_stack(client, context.workspace_id, MAX_STACK_WINDOWS)?;
    style_stack_column(client, context.workspace_id)?;

    focus_with_width(client, context.master_id, MASTER_WIDTH_PERCENT)
}
