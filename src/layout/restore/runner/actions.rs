use std::io;

use niri_ipc::Action;

use crate::ipc::IpcClient;

use super::super::types::RestoreDecision;

pub(super) fn apply_restore_decision(
    client: &mut impl IpcClient,
    target_id: u64,
    target_column: usize,
    decision: RestoreDecision,
) -> io::Result<()> {
    client.run_action_best_effort(Action::FocusWindow { id: target_id })?;

    match decision {
        RestoreDecision::ExpelForeign => {
            client.run_action_best_effort(Action::ExpelWindowFromColumn {})?;
            client.run_action_best_effort(Action::FocusWindow { id: target_id })?;
        }
        RestoreDecision::MoveColumn => {
            client.run_action_best_effort(Action::MoveColumnToIndex {
                index: target_column,
            })?;
        }
        RestoreDecision::Done => {}
    }

    Ok(())
}
