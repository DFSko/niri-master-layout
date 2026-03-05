use std::io;
use std::collections::HashMap;

use niri_ipc::socket::Socket;
use niri_ipc::{Action, Window};

use crate::ipc::{focused_window, run_action, run_action_best_effort, set_window_width_percent, windows};
use crate::state_file::SavedWindowSize;
use crate::window_utils::tiled_pos;

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
            tiled_pos(window, workspace_id).and_then(|(column, row)| {
                if column > master_column {
                    Some((column, row, window.id))
                } else {
                    None
                }
            })
        })
        .min_by_key(|(column, row, _)| (*column, *row))
        .map(|(_, _, id)| id)
}

pub fn pull_windows_into_stack(
    socket: &mut Socket,
    workspace_id: u64,
    max_in_stack: usize,
) -> io::Result<()> {
    loop {
        let Some(current) = focused_window(socket)? else {
            break;
        };
        let Some((stack_column, _)) = current.layout.pos_in_scrolling_layout else {
            break;
        };

        let all_windows = windows(socket)?;
        let (stack_count, has_more_right) = all_windows
            .iter()
            .filter_map(|window| tiled_pos(window, workspace_id))
            .fold((0usize, false), |(count, has_right), (column, _)| {
                if column == stack_column {
                    (count + 1, has_right)
                } else if column > stack_column {
                    (count, true)
                } else {
                    (count, has_right)
                }
            });

        if stack_count >= max_in_stack || !has_more_right {
            break;
        }

        run_action(socket, Action::ConsumeWindowIntoColumn {})?;
    }

    Ok(())
}

pub fn style_stack_column(socket: &mut Socket, workspace_id: u64) -> io::Result<()> {
    let Some(current) = focused_window(socket)? else {
        return Ok(());
    };
    let Some((stack_column, _)) = current.layout.pos_in_scrolling_layout else {
        return Ok(());
    };

    let all_windows = windows(socket)?;
    let mut stack_windows: Vec<(usize, u64)> = all_windows
        .iter()
        .filter_map(|window| tiled_pos(window, workspace_id).map(|(column, row)| (column, row, window.id)))
        .filter(|(column, _, _)| *column == stack_column)
        .map(|(_, row, id)| (row, id))
        .collect();

    stack_windows.sort_by_key(|(row, _)| *row);
    for (_, window_id) in stack_windows {
        set_window_width_percent(socket, window_id, 40.0)?;
        run_action(socket, Action::ResetWindowHeight { id: Some(window_id) })?;
    }

    Ok(())
}

pub fn restore_columns(socket: &mut Socket, saved: &[SavedWindowSize]) -> io::Result<()> {
    let mut targets = saved.to_vec();
    targets.sort_by_key(|window| (window.column, window.row));
    let desired_by_id: HashMap<u64, usize> = saved.iter().map(|window| (window.id, window.column)).collect();

    for target in targets {
        if target.column == 0 {
            continue;
        }

        // Retry because niri actions may shift focus/indices after each step.
        for _ in 0..8 {
            let all_windows = windows(socket)?;
            let Some(current) = all_windows.iter().find(|window| window.id == target.id) else {
                break;
            };
            let Some((current_column, _)) = current.layout.pos_in_scrolling_layout else {
                break;
            };
            let Some(workspace_id) = current.workspace_id else {
                break;
            };

            let mut windows_in_current_column = 0usize;
            let mut has_foreign_windows = false;

            for window in &all_windows {
                let Some((column, _)) = tiled_pos(window, workspace_id) else {
                    continue;
                };
                if column != current_column {
                    continue;
                }

                windows_in_current_column += 1;
                let desired_column = desired_by_id.get(&window.id).copied().unwrap_or(usize::MAX);
                if desired_column != target.column {
                    has_foreign_windows = true;
                }
            }

            if current_column == target.column && !has_foreign_windows {
                break;
            }

            run_action_best_effort(socket, Action::FocusWindow { id: target.id })?;

            if windows_in_current_column > 1 && has_foreign_windows {
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
