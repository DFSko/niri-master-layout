use std::io;

use crate::cli::AppCommand;
use crate::ipc::IpcClient;
use crate::state_file::load_layout_state;

use super::context::{FocusedContext, focus_master_with_width};

mod layout;
mod plan;
mod scan;
mod windows;

use layout::ActiveMasterLayout;
use plan::ResizeDirection;

pub fn resize_master_window(
    client: &mut impl IpcClient,
    context: &FocusedContext,
    command: AppCommand,
) -> io::Result<()> {
    let Some(direction) = ResizeDirection::from_command(command) else {
        return Ok(());
    };

    if !context.state_path.exists() {
        return Ok(());
    }

    let state = load_layout_state(&context.state_path)?;
    let windows = client.windows()?;
    let Some(layout) = ActiveMasterLayout::detect(&windows, context.workspace_id, state.master_id)
    else {
        return Ok(());
    };
    let plan = layout.resize_plan(direction);

    if plan.focus_master_first {
        focus_master_with_width(client, layout.master_id, plan.master_width_percent)?;
        set_windows_width_percent(client, &layout.stack_window_ids, plan.stack_width_percent)?;
    } else {
        set_windows_width_percent(client, &layout.stack_window_ids, plan.stack_width_percent)?;
        focus_master_with_width(client, layout.master_id, plan.master_width_percent)?;
    }

    Ok(())
}

fn set_windows_width_percent(
    client: &mut impl IpcClient,
    window_ids: &[u64],
    width_percent: f64,
) -> io::Result<()> {
    for &window_id in window_ids {
        crate::ipc::set_window_width_percent(client, window_id, width_percent)?;
    }

    Ok(())
}
