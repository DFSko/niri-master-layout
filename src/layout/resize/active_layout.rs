use super::plan::resize_plan;
use super::plan::{ResizeDirection, ResizePlan};
use super::scan::{collect_column_widths, next_column};
use super::windows::{column_window_ids, has_foreign_windows};

pub(super) struct ActiveMasterLayout {
    pub(super) master_id: u64,
    pub(super) master_width_percent: f64,
    pub(super) stack_width_percent: f64,
    pub(super) stack_window_ids: Vec<u64>,
}

impl ActiveMasterLayout {
    pub(super) fn detect(
        windows: &[niri_ipc::Window],
        workspace_id: u64,
        master_id: u64,
    ) -> Option<Self> {
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

    pub(super) fn resize_plan(&self, direction: ResizeDirection) -> ResizePlan {
        resize_plan(
            self.master_width_percent,
            self.stack_width_percent,
            direction,
        )
    }
}

fn width_percent(
    column_widths: &std::collections::BTreeMap<usize, i32>,
    column: usize,
) -> Option<f64> {
    let total_width: i32 = column_widths.values().copied().sum();
    let width = *column_widths.get(&column)?;
    (total_width > 0 && width > 0).then_some(width as f64 * 100.0 / total_width as f64)
}
