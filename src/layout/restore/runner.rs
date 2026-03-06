use std::collections::HashSet;
use std::io;

use niri_ipc::Action;

use crate::ipc::IpcClient;
use crate::state_file::SavedWindowSize;

use super::decision::decide_restore_action;
use super::desired::map_desired_columns;
use super::state::target_restore_state;
use super::types::RestoreDecision;

pub fn restore_columns(client: &mut impl IpcClient, saved: &[SavedWindowSize]) -> io::Result<()> {
    let mut targets = saved.to_vec();
    targets.sort_by_key(|window| (window.column, window.row));
    let desired_by_id = map_desired_columns(saved);

    for target in targets {
        if target.column == 0 {
            continue;
        }

        let mut seen_states = HashSet::new();
        loop {
            let windows = client.windows()?;
            let Some(state) =
                target_restore_state(&windows, target.id, target.column, &desired_by_id)
            else {
                break;
            };

            let decision = decide_restore_action(&state, target.column);
            if decision == RestoreDecision::Done {
                break;
            }
            if !seen_states.insert(state.snapshot.clone()) {
                eprintln!(
                    "warn restore_cycle_detected window_id={} column={}",
                    target.id, target.column
                );
                break;
            }

            client.run_action_best_effort(Action::FocusWindow { id: target.id })?;
            if decision == RestoreDecision::ExpelForeign {
                client.run_action_best_effort(Action::ExpelWindowFromColumn {})?;
                client.run_action_best_effort(Action::FocusWindow { id: target.id })?;
            } else {
                client.run_action_best_effort(Action::MoveColumnToIndex {
                    index: target.column,
                })?;
            }
        }
    }

    Ok(())
}
