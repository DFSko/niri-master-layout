use std::collections::{HashMap, HashSet};
use std::io;

use niri_ipc::socket::Socket;
use niri_ipc::{Action, Window};

use crate::ipc::{
    focused_window, run_action, run_action_best_effort, set_window_width_percent, windows,
};
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
    socket: &mut Socket,
    workspace_id: u64,
    max_in_stack: usize,
) -> io::Result<()> {
    loop {
        let Some(stack_column) = focused_stack_column(socket)? else {
            break;
        };

        let all_windows = windows(socket)?;
        let (stack_count, has_more_right) =
            stack_column_state(&all_windows, workspace_id, stack_column);

        if stack_count >= max_in_stack || !has_more_right {
            break;
        }

        run_action(socket, Action::ConsumeWindowIntoColumn {})?;
    }

    Ok(())
}

pub fn style_stack_column(socket: &mut Socket, workspace_id: u64) -> io::Result<()> {
    let Some(stack_column) = focused_stack_column(socket)? else {
        return Ok(());
    };

    let all_windows = windows(socket)?;
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
        set_window_width_percent(socket, window_id, STACK_WINDOW_WIDTH_PERCENT)?;
        run_action(
            socket,
            Action::ResetWindowHeight {
                id: Some(window_id),
            },
        )?;
    }

    Ok(())
}

pub fn restore_columns(socket: &mut Socket, saved: &[SavedWindowSize]) -> io::Result<()> {
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
            let all_windows = windows(socket)?;
            let Some(state) =
                target_restore_state(&all_windows, target.id, target.column, &desired_by_id)
            else {
                break;
            };

            if state.current_column == target.column && !state.has_foreign_windows {
                break;
            }

            if !seen_states.insert(state.snapshot) {
                eprintln!(
                    "detected restore cycle for window {}, stop restoring this target",
                    target.id
                );
                break;
            }

            run_action_best_effort(socket, Action::FocusWindow { id: target.id })?;

            if state.windows_in_current_column > 1 && state.has_foreign_windows {
                run_action_best_effort(socket, Action::ExpelWindowFromColumn {})?;
                run_action_best_effort(socket, Action::FocusWindow { id: target.id })?;
                continue;
            }

            run_action_best_effort(
                socket,
                Action::MoveColumnToIndex {
                    index: target.column,
                },
            )?;
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

    let mut windows_in_current_column = 0usize;
    let mut has_foreign_windows = false;
    let mut column_window_ids = Vec::new();

    for window in all_windows {
        let Some((window_column, _)) = tiled_pos(window, workspace_id) else {
            continue;
        };
        if window_column != current_column {
            continue;
        }

        windows_in_current_column += 1;
        column_window_ids.push(window.id);

        let desired_column = desired_by_id.get(&window.id).copied().unwrap_or(usize::MAX);
        has_foreign_windows |= desired_column != target_column;
    }

    column_window_ids.sort_unstable();

    Some(TargetRestoreState {
        current_column,
        windows_in_current_column,
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
    let analysis = analyze_column(all_windows, workspace_id, stack_column);
    (analysis.count_in_column, analysis.has_right_columns)
}

fn focused_stack_column(socket: &mut Socket) -> io::Result<Option<usize>> {
    let Some(current) = focused_window(socket)? else {
        return Ok(None);
    };

    Ok(current
        .layout
        .pos_in_scrolling_layout
        .map(|(column, _)| column))
}

struct ColumnAnalysis {
    count_in_column: usize,
    has_right_columns: bool,
}

fn analyze_column(all_windows: &[Window], workspace_id: u64, column: usize) -> ColumnAnalysis {
    all_windows
        .iter()
        .filter_map(|window| {
            tiled_pos(window, workspace_id).map(|(window_column, _)| window_column)
        })
        .fold(
            ColumnAnalysis {
                count_in_column: 0,
                has_right_columns: false,
            },
            |mut analysis, window_column| {
                if window_column == column {
                    analysis.count_in_column += 1;
                } else if window_column > column {
                    analysis.has_right_columns = true;
                }

                analysis
            },
        )
}

struct TargetRestoreState {
    current_column: usize,
    windows_in_current_column: usize,
    has_foreign_windows: bool,
    snapshot: RestoreSnapshot,
}

#[derive(Hash, PartialEq, Eq)]
struct RestoreSnapshot {
    current_column: usize,
    column_window_ids: Vec<u64>,
}
