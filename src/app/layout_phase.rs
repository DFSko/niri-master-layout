use std::io;

use crate::ipc::IpcClient;

use super::context::FocusedContext;
use super::layout_anchor::move_master_and_find_anchor;
use super::layout_snapshot::save_snapshot;
use super::layout_stack::build_stack_layout;

pub fn apply_master_stack_layout(
    client: &mut impl IpcClient,
    context: &FocusedContext,
) -> io::Result<()> {
    let mut state_cleanup = save_snapshot(client, context)?;

    let Some(stack_anchor_id) = move_master_and_find_anchor(client, context)? else {
        return Ok(());
    };

    state_cleanup.keep();
    build_stack_layout(client, context, stack_anchor_id)
}
