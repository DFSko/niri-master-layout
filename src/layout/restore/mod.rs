use std::collections::{HashMap, HashSet};
use std::io;

use niri_ipc::Action;

use crate::ipc::IpcClient;
use crate::layout::tiled_pos;
use crate::state::SavedWindowSize;

pub fn restore_columns(client: &mut impl IpcClient, saved: &[SavedWindowSize]) -> io::Result<()> {
    let mut targets = saved.to_vec();
    targets.sort_by_key(|window| (window.column, window.row));
    let desired_by_id = saved
        .iter()
        .map(|window| (window.id, window.column))
        .collect::<HashMap<_, _>>();

    for target in targets.into_iter().filter(|target| target.column > 0) {
        restore_target(client, &desired_by_id, target)?;
    }

    Ok(())
}

fn restore_target(
    client: &mut impl IpcClient,
    desired_by_id: &HashMap<u64, usize>,
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

fn snapshot_target(
    windows: &[niri_ipc::Window],
    target_id: u64,
    target_column: usize,
    desired_by_id: &HashMap<u64, usize>,
) -> Option<RestoreTarget> {
    let current = windows.iter().find(|window| window.id == target_id)?;
    let (current_column, _) = current.layout.pos_in_scrolling_layout?;
    let workspace_id = current.workspace_id?;

    let mut column_window_ids = windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id).and_then(|(window_column, _)| {
                (window_column == current_column).then_some(window.id)
            })
        })
        .collect::<Vec<_>>();
    let has_foreign_windows = column_window_ids.iter().any(|window_id| {
        desired_by_id.get(window_id).copied().unwrap_or(usize::MAX) != target_column
    });

    column_window_ids.sort_unstable();

    Some(RestoreTarget {
        has_foreign_windows,
        column: ColumnSnapshot {
            index: current_column,
            window_ids: column_window_ids,
        },
    })
}

fn decide_restore_action(target: &RestoreTarget, target_column: usize) -> RestoreDecision {
    if target.column.index == target_column && !target.has_foreign_windows {
        return RestoreDecision::Done;
    }

    if target.column.window_ids.len() > 1 && target.has_foreign_windows {
        return RestoreDecision::ExpelForeign;
    }

    RestoreDecision::MoveColumn
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

#[derive(Clone)]
struct RestoreTarget {
    has_foreign_windows: bool,
    column: ColumnSnapshot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RestoreDecision {
    Done,
    ExpelForeign,
    MoveColumn,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct ColumnSnapshot {
    index: usize,
    window_ids: Vec<u64>,
}
