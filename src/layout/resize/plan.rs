use crate::app::AppCommand;

pub(super) enum ResizeDirection {
    GrowMaster,
    ShrinkMaster,
}

pub(super) struct ResizePlan {
    pub(super) master_width_percent: f64,
    pub(super) stack_width_percent: f64,
    pub(super) focus_master_first: bool,
}

const RESIZE_STEP_PERCENT: f64 = 10.0;
const MIN_MASTER_WIDTH_PERCENT: f64 = 10.0;
const MAX_MASTER_WIDTH_PERCENT: f64 = 90.0;

impl ResizeDirection {
    pub(super) fn from_command(command: AppCommand) -> Option<Self> {
        match command {
            AppCommand::GrowMaster => Some(Self::GrowMaster),
            AppCommand::ShrinkMaster => Some(Self::ShrinkMaster),
            AppCommand::Toggle => None,
        }
    }
}

pub(super) fn resize_plan(
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
