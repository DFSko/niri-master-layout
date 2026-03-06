use std::io;
use std::path::Path;

use niri_ipc::Action;

use crate::ipc::{
    IpcClient, set_window_height_fixed_best_effort, set_window_width_fixed_best_effort,
};
use crate::layout::restore_columns;
use crate::state_file::load_layout_state;

use super::cleanup::remove_file_if_exists;

pub fn restore_layout_state(client: &mut impl IpcClient, path: &Path) -> io::Result<()> {
    let state = load_layout_state(path)?;

    restore_columns(client, &state.windows)?;

    for window in state.windows {
        set_window_width_fixed_best_effort(client, window.id, window.width)?;
        set_window_height_fixed_best_effort(client, window.id, window.height)?;
    }

    client.run_action_best_effort(Action::FocusWindow {
        id: state.master_id,
    })?;
    remove_file_if_exists(path)
}
