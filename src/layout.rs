use std::collections::{HashMap, HashSet};
use std::io;

use niri_ipc::{Action, Window};

use crate::ipc::{IpcClient, set_window_width_percent};
use crate::state_file::SavedWindowSize;
use crate::window_utils::tiled_pos;

const STACK_WINDOW_WIDTH_PERCENT: f64 = 40.0;

pub fn nearest_right_column_anchor(
    windows: &[Window],
    workspace_id: u64,
    master_column: usize,
    master_id: u64,
) -> Option<u64> {
    windows
        .iter()
        .filter(|window| window.id != master_id)
        .filter_map(|window| {
            tiled_pos(window, workspace_id).map(|(column, row)| (column, row, window.id))
        })
        .filter(|(column, _, _)| *column > master_column)
        .min_by_key(|(column, row, _)| (*column, *row))
        .map(|(_, _, id)| id)
}

pub fn pull_windows_into_stack(
    client: &mut impl IpcClient,
    workspace_id: u64,
    max_in_stack: usize,
) -> io::Result<()> {
    loop {
        let Some(stack_column) = focused_stack_column(client)? else {
            break;
        };

        let all_windows = client.windows()?;
        let (stack_count, has_more_right) =
            stack_column_state(&all_windows, workspace_id, stack_column);

        if stack_count >= max_in_stack || !has_more_right {
            break;
        }

        client.run_action(Action::ConsumeWindowIntoColumn {})?;
    }

    Ok(())
}

pub fn style_stack_column(client: &mut impl IpcClient, workspace_id: u64) -> io::Result<()> {
    let Some(stack_column) = focused_stack_column(client)? else {
        return Ok(());
    };

    let all_windows = client.windows()?;
    let mut stack_windows: Vec<(usize, u64)> = all_windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id).map(|(column, row)| (column, row, window.id))
        })
        .filter(|(column, _, _)| *column == stack_column)
        .map(|(_, row, id)| (row, id))
        .collect();

    stack_windows.sort_by_key(|(row, _)| *row);
    for (_, window_id) in stack_windows {
        set_window_width_percent(client, window_id, STACK_WINDOW_WIDTH_PERCENT)?;
        client.run_action(Action::ResetWindowHeight {
            id: Some(window_id),
        })?;
    }

    Ok(())
}

pub fn restore_columns(client: &mut impl IpcClient, saved: &[SavedWindowSize]) -> io::Result<()> {
    let mut targets = saved.to_vec();
    targets.sort_by_key(|window| (window.column, window.row));
    let desired_by_id: HashMap<u64, usize> = saved
        .iter()
        .map(|window| (window.id, window.column))
        .collect();

    for target in targets {
        if target.column == 0 {
            continue;
        }

        let mut seen_states = HashSet::new();
        loop {
            let all_windows = client.windows()?;
            let Some(state) =
                target_restore_state(&all_windows, target.id, target.column, &desired_by_id)
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

fn target_restore_state(
    all_windows: &[Window],
    target_id: u64,
    target_column: usize,
    desired_by_id: &HashMap<u64, usize>,
) -> Option<TargetRestoreState> {
    let current = all_windows.iter().find(|window| window.id == target_id)?;
    let (current_column, _) = current.layout.pos_in_scrolling_layout?;
    let workspace_id = current.workspace_id?;

    let mut has_foreign_windows = false;
    let mut column_window_ids = Vec::new();

    for window in all_windows {
        let Some((window_column, _)) = tiled_pos(window, workspace_id) else {
            continue;
        };
        if window_column != current_column {
            continue;
        }

        column_window_ids.push(window.id);

        let desired_column = desired_by_id.get(&window.id).copied().unwrap_or(usize::MAX);
        has_foreign_windows |= desired_column != target_column;
    }

    column_window_ids.sort_unstable();

    Some(TargetRestoreState {
        has_foreign_windows,
        snapshot: RestoreSnapshot {
            current_column,
            column_window_ids,
        },
    })
}

fn stack_column_state(
    all_windows: &[Window],
    workspace_id: u64,
    stack_column: usize,
) -> (usize, bool) {
    let mut count_in_column = 0usize;
    let mut has_right_columns = false;

    for window in all_windows {
        let Some((column, _)) = tiled_pos(window, workspace_id) else {
            continue;
        };

        if column == stack_column {
            count_in_column += 1;
        } else if column > stack_column {
            has_right_columns = true;
        }
    }

    (count_in_column, has_right_columns)
}

fn focused_stack_column(client: &mut impl IpcClient) -> io::Result<Option<usize>> {
    let Some(current) = client.focused_window()? else {
        return Ok(None);
    };

    Ok(current
        .layout
        .pos_in_scrolling_layout
        .map(|(column, _)| column))
}

#[derive(Clone)]
struct TargetRestoreState {
    has_foreign_windows: bool,
    snapshot: RestoreSnapshot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RestoreDecision {
    Done,
    ExpelForeign,
    MoveColumn,
}

fn decide_restore_action(state: &TargetRestoreState, target_column: usize) -> RestoreDecision {
    if state.snapshot.current_column == target_column && !state.has_foreign_windows {
        return RestoreDecision::Done;
    }

    if state.snapshot.column_window_ids.len() > 1 && state.has_foreign_windows {
        return RestoreDecision::ExpelForeign;
    }

    RestoreDecision::MoveColumn
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct RestoreSnapshot {
    current_column: usize,
    column_window_ids: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ipc::testing::FakeClient;
    use crate::state_file::SavedWindowSize;
    use crate::test_support::make_tiled_window;

    #[test]
    fn restore_decision_done_when_target_column_without_foreign() {
        let state = TargetRestoreState {
            has_foreign_windows: false,
            snapshot: RestoreSnapshot {
                current_column: 2,
                column_window_ids: vec![10],
            },
        };

        assert_eq!(decide_restore_action(&state, 2), RestoreDecision::Done);
    }

    #[test]
    fn restore_decision_expel_when_mixed_column() {
        let state = TargetRestoreState {
            has_foreign_windows: true,
            snapshot: RestoreSnapshot {
                current_column: 1,
                column_window_ids: vec![10, 20],
            },
        };

        assert_eq!(
            decide_restore_action(&state, 2),
            RestoreDecision::ExpelForeign
        );
    }

    #[test]
    fn restore_decision_move_when_single_non_target() {
        let state = TargetRestoreState {
            has_foreign_windows: false,
            snapshot: RestoreSnapshot {
                current_column: 1,
                column_window_ids: vec![10],
            },
        };

        assert_eq!(
            decide_restore_action(&state, 2),
            RestoreDecision::MoveColumn
        );
    }

    #[test]
    fn restore_columns_uses_expel_then_move_flow() {
        let workspace_id = 1;
        let target_id = 10;

        let snapshots = vec![
            vec![
                make_tiled_window(target_id, workspace_id, 1, 1, 800, 600),
                make_tiled_window(20, workspace_id, 1, 2, 800, 600),
            ],
            vec![
                make_tiled_window(target_id, workspace_id, 1, 1, 800, 600),
                make_tiled_window(20, workspace_id, 3, 1, 800, 600),
            ],
            vec![
                make_tiled_window(target_id, workspace_id, 2, 1, 800, 600),
                make_tiled_window(20, workspace_id, 3, 1, 800, 600),
            ],
        ];

        let saved = vec![SavedWindowSize {
            id: target_id,
            width: 800,
            height: 600,
            column: 2,
            row: 1,
        }];

        let mut client = FakeClient::with_windows(snapshots);
        restore_columns(&mut client, &saved).expect("restore should succeed");

        assert_eq!(client.best_effort_actions.len(), 5);
        assert!(matches!(
            client.best_effort_actions[0],
            Action::FocusWindow { id } if id == target_id
        ));
        assert!(matches!(
            client.best_effort_actions[1],
            Action::ExpelWindowFromColumn {}
        ));
        assert!(matches!(
            client.best_effort_actions[2],
            Action::FocusWindow { id } if id == target_id
        ));
        assert!(matches!(
            client.best_effort_actions[3],
            Action::FocusWindow { id } if id == target_id
        ));
        assert!(matches!(
            client.best_effort_actions[4],
            Action::MoveColumnToIndex { index } if index == 2
        ));
    }
}
