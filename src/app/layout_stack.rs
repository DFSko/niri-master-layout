use std::io;

use niri_ipc::{Action, ColumnDisplay};

use crate::ipc::IpcClient;
use crate::layout::{pull_windows_into_stack, style_stack_column};

use super::context::{FocusedContext, focus_master_with_width};

const MASTER_WIDTH_PERCENT: f64 = 60.0;
const MAX_STACK_WINDOWS: usize = 3;

pub fn build_stack_layout(
    client: &mut impl IpcClient,
    context: &FocusedContext,
    stack_anchor_id: u64,
) -> io::Result<()> {
    focus_master_with_width(client, context.master_id, MASTER_WIDTH_PERCENT)?;

    client.run_action(Action::FocusWindow {
        id: stack_anchor_id,
    })?;
    client.run_action(Action::SetColumnDisplay {
        display: ColumnDisplay::Normal,
    })?;

    pull_windows_into_stack(client, context.workspace_id, MAX_STACK_WINDOWS)?;
    style_stack_column(client, context.workspace_id)?;

    focus_master_with_width(client, context.master_id, MASTER_WIDTH_PERCENT)
}
