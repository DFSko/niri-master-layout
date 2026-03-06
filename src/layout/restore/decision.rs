use super::types::{RestoreDecision, TargetRestoreState};

pub fn decide_restore_action(state: &TargetRestoreState, target_column: usize) -> RestoreDecision {
    if state.snapshot.current_column == target_column && !state.has_foreign_windows {
        return RestoreDecision::Done;
    }

    if state.snapshot.column_window_ids.len() > 1 && state.has_foreign_windows {
        return RestoreDecision::ExpelForeign;
    }

    RestoreDecision::MoveColumn
}
