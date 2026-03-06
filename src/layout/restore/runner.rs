mod actions;

use std::collections::HashSet;
use std::io;

use crate::ipc::IpcClient;
use crate::state_file::SavedWindowSize;

use super::decision::decide_restore_action;
use super::desired::map_desired_columns;
use super::state::target_restore_state;
use super::types::RestoreDecision;
use actions::apply_restore_decision;

pub fn restore_columns(client: &mut impl IpcClient, saved: &[SavedWindowSize]) -> io::Result<()> {
    let mut targets = saved.to_vec();
    targets.sort_by_key(|window| (window.column, window.row));
    let desired_by_id = map_desired_columns(saved);

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
        let Some(state) = target_restore_state(&windows, target.id, target.column, desired_by_id)
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

        apply_restore_decision(client, target.id, target.column, decision)?;
    }

    Ok(())
}
