use std::collections::HashSet;
use std::io;

use niri_ipc::Action;

use crate::ipc::IpcClient;
use crate::state::SavedWindowSize;

use super::decision::decide_restore_action;
use super::desired::desired_columns_by_id;
use super::snapshot::snapshot_target;
use super::types::RestoreDecision;

pub fn restore_columns(client: &mut impl IpcClient, saved: &[SavedWindowSize]) -> io::Result<()> {
    let mut targets = saved.to_vec();
    targets.sort_by_key(|window| (window.column, window.row));
    let desired_by_id = desired_columns_by_id(saved);

    for target in targets.into_iter().filter(|target| target.column > 0) {
        restore_target(client, &desired_by_id, target)?;
    }

    Ok(())
}

fn restore_target(
    client: &mut impl IpcClient,
    desired_by_id: &std::collections::HashMap<u64, usize>,
    target: SavedWindowSize,
) -> io::Result<()> {
    let mut seen_states = HashSet::new();

    loop {
        let windows = client.windows()?;
        let Some(snapshot) = snapshot_target(&windows, target.id, target.column, desired_by_id)
        else {
            break;
        };
        let decision = decide_restore_action(&snapshot, target.column);

        if decision == RestoreDecision::Done {
            break;
        }
        if !seen_states.insert(snapshot.column.clone()) {
            eprintln!(
                "warn restore_cycle_detected window_id={} column={}",
                target.id, target.column
            );
            break;
        }

        apply_restore_decision(client, target.id, target.column, decision)?;
    }

    Ok(())
}

fn apply_restore_decision(
    client: &mut impl IpcClient,
    target_id: u64,
    target_column: usize,
    decision: RestoreDecision,
) -> io::Result<()> {
    client.action_best_effort(Action::FocusWindow { id: target_id })?;

    match decision {
        RestoreDecision::ExpelForeign => {
            client.action_best_effort(Action::ExpelWindowFromColumn {})?;
            client.action_best_effort(Action::FocusWindow { id: target_id })?;
        }
        RestoreDecision::MoveColumn => {
            client.action_best_effort(Action::MoveColumnToIndex {
                index: target_column,
            })?;
        }
        RestoreDecision::Done => {}
    }

    Ok(())
}
