mod resize;
mod restore;
mod stack;

use niri_ipc::Window;

pub use resize::resize;
pub use restore::restore_columns;
pub use stack::{nearest_right_column_anchor, pull_windows_into_stack, style_stack_column};

pub fn tiled_pos(window: &Window, workspace_id: u64) -> Option<(usize, usize)> {
    if window.is_floating || window.workspace_id != Some(workspace_id) {
        return None;
    }

    window.layout.pos_in_scrolling_layout
}
