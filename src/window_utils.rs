use niri_ipc::Window;

pub fn tiled_pos(window: &Window, workspace_id: u64) -> Option<(usize, usize)> {
    if window.is_floating || window.workspace_id != Some(workspace_id) {
        return None;
    }

    window.layout.pos_in_scrolling_layout
}
