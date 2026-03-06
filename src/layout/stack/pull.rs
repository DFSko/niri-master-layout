use std::io;

use niri_ipc::Action;

use crate::ipc::IpcClient;

use super::column::{focused_stack_column, stack_column_state};

pub fn pull_windows_into_stack(
    client: &mut impl IpcClient,
    workspace_id: u64,
    max_in_stack: usize,
) -> io::Result<()> {
    loop {
        let Some(stack_column) = focused_stack_column(client)? else {
            break;
        };

        let windows = client.windows()?;
        let (stack_count, has_more_right) =
            stack_column_state(&windows, workspace_id, stack_column);

        if stack_count >= max_in_stack || !has_more_right {
            break;
        }

        client.run_action(Action::ConsumeWindowIntoColumn {})?;
    }

    Ok(())
}
