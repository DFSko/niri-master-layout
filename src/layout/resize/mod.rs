use std::io;
use std::path::Path;

use niri_ipc::Action;

use crate::app::AppCommand;
use crate::ipc::IpcClient;
use crate::state::load_state;

mod active_layout;
mod plan;
mod scan;
mod windows;

use active_layout::ActiveMasterLayout;
use plan::ResizeDirection;

pub fn resize(
    client: &mut impl IpcClient,
    workspace_id: u64,
    state_path: &Path,
    command: AppCommand,
) -> io::Result<()> {
    let Some(direction) = ResizeDirection::from_command(command) else {
        return Ok(());
    };

    if !state_path.exists() {
        return Ok(());
    }

    let state = load_state(state_path)?;
    let windows = client.windows()?;
    let Some(layout) = ActiveMasterLayout::detect(&windows, workspace_id, state.master_id) else {
        return Ok(());
    };
    let plan = layout.resize_plan(direction);

    if plan.focus_master_first {
        focus_with_width(client, layout.master_id, plan.master_width_percent)?;
        set_width_percent(client, &layout.stack_window_ids, plan.stack_width_percent)?;
    } else {
        set_width_percent(client, &layout.stack_window_ids, plan.stack_width_percent)?;
        focus_with_width(client, layout.master_id, plan.master_width_percent)?;
    }

    Ok(())
}

fn focus_with_width(
    client: &mut impl IpcClient,
    window_id: u64,
    width_percent: f64,
) -> io::Result<()> {
    client.action(Action::FocusWindow { id: window_id })?;
    crate::ipc::set_width_percent(client, window_id, width_percent)
}

fn set_width_percent(client: &mut impl IpcClient, window_ids: &[u64], width_percent: f64) -> io::Result<()> {
    for &window_id in window_ids {
        crate::ipc::set_width_percent(client, window_id, width_percent)?;
    }

    Ok(())
}
