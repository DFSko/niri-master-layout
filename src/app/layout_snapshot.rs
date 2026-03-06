use std::io;

use crate::ipc::IpcClient;
use crate::state_file::save_layout_state;

use super::context::FocusedContext;
use super::state_restore::PendingStateCleanup;

pub fn save_snapshot<'a>(
    client: &mut impl IpcClient,
    context: &'a FocusedContext,
) -> io::Result<PendingStateCleanup<'a>> {
    let all_windows_before = client.windows()?;

    save_layout_state(
        &context.state_path,
        context.master_id,
        context.workspace_id,
        &all_windows_before,
    )?;

    Ok(PendingStateCleanup::new(&context.state_path))
}
