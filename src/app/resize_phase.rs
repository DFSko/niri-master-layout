use std::collections::BTreeMap;
use std::io;

use crate::cli::AppCommand;
use crate::ipc::IpcClient;
use crate::state_file::load_layout_state;
use crate::window_utils::tiled_pos;

use super::context::{FocusedContext, focus_master_with_width};

const RESIZE_STEP_PERCENT: f64 = 10.0;
const MIN_MASTER_WIDTH_PERCENT: f64 = 10.0;
const MAX_MASTER_WIDTH_PERCENT: f64 = 90.0;

pub fn resize_master_window(
    client: &mut impl IpcClient,
    context: &FocusedContext,
    command: AppCommand,
) -> io::Result<()> {
    if !context.state_path.exists() {
        return Ok(());
    }

    let state = load_layout_state(&context.state_path)?;
    let windows = client.windows()?;
    let Some(layout) = ActiveMasterLayout::detect(&windows, context.workspace_id, state.master_id)
    else {
        return Ok(());
    };

    match command {
        AppCommand::GrowMaster => {
            let stack_width_percent = shrink_column_width_percent(layout.stack_width_percent);
            let master_width_percent = 100.0 - stack_width_percent;
            set_windows_width_percent(client, &layout.stack_window_ids, stack_width_percent)?;
            focus_master_with_width(client, layout.master_id, master_width_percent)?;
        }
        AppCommand::ShrinkMaster => {
            let master_width_percent = shrink_column_width_percent(layout.master_width_percent);
            let stack_width_percent = 100.0 - master_width_percent;
            focus_master_with_width(client, layout.master_id, master_width_percent)?;
            set_windows_width_percent(client, &layout.stack_window_ids, stack_width_percent)?;
        }
        AppCommand::Toggle => {}
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

fn shrink_column_width_percent(current_percent: f64) -> f64 {
    let current_step = (current_percent / RESIZE_STEP_PERCENT).round() * RESIZE_STEP_PERCENT;
    (current_step - RESIZE_STEP_PERCENT).clamp(MIN_MASTER_WIDTH_PERCENT, MAX_MASTER_WIDTH_PERCENT)
}

struct ActiveMasterLayout {
    master_id: u64,
    master_width_percent: f64,
    stack_width_percent: f64,
    stack_window_ids: Vec<u64>,
}

impl ActiveMasterLayout {
    fn detect(windows: &[niri_ipc::Window], workspace_id: u64, master_id: u64) -> Option<Self> {
        let mut column_widths = BTreeMap::<usize, i32>::new();
        let mut master_column = None;

        for window in windows {
            let Some((column, row)) = tiled_pos(window, workspace_id) else {
                continue;
            };

            column_widths
                .entry(column)
                .and_modify(|width| {
                    *width = (*width).max(window.layout.tile_size.0.max(1.0) as i32)
                })
                .or_insert(window.layout.tile_size.0.max(1.0) as i32);

            if window.id == master_id {
                if master_column.replace(column).is_some() {
                    return None;
                }
                if row != 1 {
                    return None;
                }
                continue;
            }
        }

        let master_column = master_column?;
        let leftmost_column = *column_widths.keys().next()?;
        if master_column != leftmost_column {
            return None;
        }

        let stack_column = column_widths
            .keys()
            .copied()
            .filter(|column| *column > master_column)
            .min()?;
        let stack_window_ids = windows
            .iter()
            .filter_map(|window| {
                tiled_pos(window, workspace_id)
                    .and_then(|(column, _)| (column == stack_column).then_some(window.id))
            })
            .collect::<Vec<_>>();
        if stack_window_ids.is_empty() {
            return None;
        }

        for window in windows {
            let Some((column, _)) = tiled_pos(window, workspace_id) else {
                continue;
            };

            if window.id == master_id {
                continue;
            }

            if column == master_column {
                return None;
            }
        }

        let total_width: i32 = column_widths.values().copied().sum();
        let master_width = *column_widths.get(&master_column)?;
        let stack_width = *column_widths.get(&stack_column)?;
        if total_width <= 0 || master_width <= 0 {
            return None;
        }

        Some(Self {
            master_id,
            master_width_percent: master_width as f64 * 100.0 / total_width as f64,
            stack_width_percent: stack_width as f64 * 100.0 / total_width as f64,
            stack_window_ids,
        })
    }
}
