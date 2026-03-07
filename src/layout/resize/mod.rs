use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use crate::app::AppCommand;
use crate::ipc::{IpcClient, focus_with_width, set_width_percent_for_windows};
use crate::layout::tiled_pos;
use crate::state::load_state;

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
        set_width_percent_for_windows(client, &layout.stack_window_ids, plan.stack_width_percent)?;
    } else {
        set_width_percent_for_windows(client, &layout.stack_window_ids, plan.stack_width_percent)?;
        focus_with_width(client, layout.master_id, plan.master_width_percent)?;
    }

    Ok(())
}

struct ActiveMasterLayout {
    master_id: u64,
    master_width_percent: f64,
    stack_width_percent: f64,
    stack_window_ids: Vec<u64>,
}

impl ActiveMasterLayout {
    fn detect(windows: &[niri_ipc::Window], workspace_id: u64, master_id: u64) -> Option<Self> {
        let (column_widths, master_column) =
            collect_column_widths(windows, workspace_id, master_id)?;
        let master_column = master_column?;
        let stack_column = next_column(&column_widths, master_column)?;
        let stack_window_ids = column_window_ids(windows, workspace_id, stack_column);

        if stack_window_ids.is_empty()
            || has_foreign_windows(windows, workspace_id, master_column, master_id)
        {
            return None;
        }

        Some(Self {
            master_id,
            master_width_percent: width_percent(&column_widths, master_column)?,
            stack_width_percent: width_percent(&column_widths, stack_column)?,
            stack_window_ids,
        })
    }

    fn resize_plan(&self, direction: ResizeDirection) -> ResizePlan {
        resize_plan(
            self.master_width_percent,
            self.stack_width_percent,
            direction,
        )
    }
}

enum ResizeDirection {
    GrowMaster,
    ShrinkMaster,
}

struct ResizePlan {
    master_width_percent: f64,
    stack_width_percent: f64,
    focus_master_first: bool,
}

const RESIZE_STEP_PERCENT: f64 = 10.0;
const MIN_MASTER_WIDTH_PERCENT: f64 = 10.0;
const MAX_MASTER_WIDTH_PERCENT: f64 = 90.0;

impl ResizeDirection {
    fn from_command(command: AppCommand) -> Option<Self> {
        match command {
            AppCommand::GrowMaster => Some(Self::GrowMaster),
            AppCommand::ShrinkMaster => Some(Self::ShrinkMaster),
            AppCommand::Toggle => None,
        }
    }
}

fn resize_plan(
    master_width_percent: f64,
    stack_width_percent: f64,
    direction: ResizeDirection,
) -> ResizePlan {
    match direction {
        ResizeDirection::GrowMaster => {
            let stack_width_percent = shrink_column_width_percent(stack_width_percent);
            ResizePlan {
                master_width_percent: 100.0 - stack_width_percent,
                stack_width_percent,
                focus_master_first: false,
            }
        }
        ResizeDirection::ShrinkMaster => {
            let master_width_percent = shrink_column_width_percent(master_width_percent);
            ResizePlan {
                master_width_percent,
                stack_width_percent: 100.0 - master_width_percent,
                focus_master_first: true,
            }
        }
    }
}

fn shrink_column_width_percent(current_percent: f64) -> f64 {
    let current_step = (current_percent / RESIZE_STEP_PERCENT).round() * RESIZE_STEP_PERCENT;
    (current_step - RESIZE_STEP_PERCENT).clamp(MIN_MASTER_WIDTH_PERCENT, MAX_MASTER_WIDTH_PERCENT)
}

fn collect_column_widths(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    master_id: u64,
) -> Option<(BTreeMap<usize, i32>, Option<usize>)> {
    let mut column_widths = BTreeMap::<usize, i32>::new();
    let mut master_column = None;

    for window in windows {
        let Some((column, row)) = tiled_pos(window, workspace_id) else {
            continue;
        };

        column_widths
            .entry(column)
            .and_modify(|width| *width = (*width).max(window.layout.tile_size.0.max(1.0) as i32))
            .or_insert_with(|| window.layout.tile_size.0.max(1.0) as i32);

        if window.id == master_id && (row != 1 || master_column.replace(column).is_some()) {
            return None;
        }
    }

    let leftmost_column = *column_widths.keys().next()?;
    (master_column == Some(leftmost_column)).then_some((column_widths, master_column))
}

fn next_column(column_widths: &BTreeMap<usize, i32>, column: usize) -> Option<usize> {
    column_widths.keys().copied().find(|value| *value > column)
}

fn column_window_ids(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    stack_column: usize,
) -> Vec<u64> {
    windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id)
                .and_then(|(column, _)| (column == stack_column).then_some(window.id))
        })
        .collect()
}

fn has_foreign_windows(
    windows: &[niri_ipc::Window],
    workspace_id: u64,
    column: usize,
    ignored_id: u64,
) -> bool {
    windows.iter().any(|window| {
        window.id != ignored_id
            && tiled_pos(window, workspace_id)
                .is_some_and(|(window_column, _)| window_column == column)
    })
}

fn width_percent(column_widths: &BTreeMap<usize, i32>, column: usize) -> Option<f64> {
    let total_width: i32 = column_widths.values().copied().sum();
    let width = *column_widths.get(&column)?;
    (total_width > 0 && width > 0).then_some(width as f64 * 100.0 / total_width as f64)
}
