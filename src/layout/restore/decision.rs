use super::types::{RestoreDecision, RestoreTarget};

pub fn decide_restore_action(target: &RestoreTarget, target_column: usize) -> RestoreDecision {
    if target.column.index == target_column && !target.has_foreign_windows {
        return RestoreDecision::Done;
    }

    if target.column.window_ids.len() > 1 && target.has_foreign_windows {
        return RestoreDecision::ExpelForeign;
    }

    RestoreDecision::MoveColumn
}
