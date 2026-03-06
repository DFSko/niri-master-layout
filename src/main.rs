mod ipc;
mod layout;
mod state_file;
mod window_utils;

use std::fs;
use std::io;
use std::path::Path;

use niri_ipc::socket::Socket;
use niri_ipc::{Action, ColumnDisplay};

use crate::ipc::{
    focused_window, run_action, run_action_best_effort, set_window_height_fixed_best_effort,
    set_window_width_fixed_best_effort, set_window_width_percent, windows,
};
use crate::layout::{
    nearest_right_column_anchor, pull_windows_into_stack, restore_columns, style_stack_column,
};
use crate::state_file::{load_layout_state, save_layout_state, state_file_path};

fn main() -> io::Result<()> {
    let mut socket = Socket::connect()?;
    let Some(initial_master) = focused_window(&mut socket)? else {
        return Ok(());
    };
    let Some(initial_workspace_id) = initial_master.workspace_id else {
        return Ok(());
    };
    let state_path = state_file_path(initial_workspace_id);

    if state_path.exists() {
        match restore_layout_state(&mut socket, &state_path) {
            Ok(()) => return Ok(()),
            Err(error) => {
                eprintln!("failed to restore saved layout, deleting stale state: {error}");
                let _ = fs::remove_file(&state_path);
            }
        }
    }

    let Some(master) = focused_window(&mut socket)? else {
        return Ok(());
    };

    let Some(workspace_id) = master.workspace_id else {
        return Ok(());
    };
    let state_path = state_file_path(workspace_id);

    let all_windows_before = windows(&mut socket)?;

    save_layout_state(&state_path, master.id, workspace_id, &all_windows_before)?;
    let mut state_cleanup = PendingStateCleanup::new(&state_path);

    run_action(&mut socket, Action::FocusWindow { id: master.id })?;
    run_action(&mut socket, Action::MoveColumnToIndex { index: 1 })?;

    let all_windows_after = windows(&mut socket)?;
    let Some(master_after_move) = all_windows_after.iter().find(|window| window.id == master.id) else {
        return Ok(());
    };
    let Some((master_column, _)) = master_after_move.layout.pos_in_scrolling_layout else {
        return Ok(());
    };

    let Some(stack_anchor_id) = nearest_right_column_anchor(
        &all_windows_after,
        workspace_id,
        master_column,
        master.id,
    ) else {
        eprintln!("no right-side columns found; nothing to stack");
        return Ok(());
    };

    state_cleanup.keep();

    set_window_width_percent(&mut socket, master.id, 60.0)?;

    run_action(&mut socket, Action::FocusWindow { id: stack_anchor_id })?;
    run_action(
        &mut socket,
        Action::SetColumnDisplay {
            display: ColumnDisplay::Normal,
        },
    )?;

    pull_windows_into_stack(&mut socket, workspace_id, 3)?;
    style_stack_column(&mut socket, workspace_id)?;
    run_action(&mut socket, Action::FocusWindow { id: master.id })?;
    set_window_width_percent(&mut socket, master.id, 60.0)?;

    Ok(())
}

struct PendingStateCleanup<'a> {
    path: &'a Path,
    keep: bool,
}

impl<'a> PendingStateCleanup<'a> {
    fn new(path: &'a Path) -> Self {
        Self { path, keep: false }
    }

    fn keep(&mut self) {
        self.keep = true;
    }
}

impl Drop for PendingStateCleanup<'_> {
    fn drop(&mut self) {
        if self.keep {
            return;
        }

        if let Err(error) = fs::remove_file(self.path) {
            if error.kind() != io::ErrorKind::NotFound {
                eprintln!("failed to remove stale state file: {error}");
            }
        }
    }
}

fn restore_layout_state(socket: &mut Socket, path: &Path) -> io::Result<()> {
    let state = load_layout_state(path)?;

    restore_columns(socket, &state.windows)?;

    for window in state.windows {
        set_window_width_fixed_best_effort(socket, window.id, window.width)?;
        set_window_height_fixed_best_effort(socket, window.id, window.height)?;
    }

    run_action_best_effort(socket, Action::FocusWindow { id: state.master_id })?;

    if let Err(error) = fs::remove_file(path) {
        if error.kind() != io::ErrorKind::NotFound {
            return Err(error);
        }
    }

    Ok(())
}
