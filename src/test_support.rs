use niri_ipc::{Window, WindowLayout};

pub fn make_tiled_window(
    id: u64,
    workspace_id: u64,
    column: usize,
    row: usize,
    width: i32,
    height: i32,
) -> Window {
    Window {
        id,
        title: None,
        app_id: None,
        pid: None,
        workspace_id: Some(workspace_id),
        is_focused: false,
        is_floating: false,
        is_urgent: false,
        layout: WindowLayout {
            pos_in_scrolling_layout: Some((column, row)),
            tile_size: (width as f64, height as f64),
            window_size: (width, height),
            tile_pos_in_workspace_view: None,
            window_offset_in_tile: (0.0, 0.0),
        },
        focus_timestamp: None,
    }
}

pub fn make_floating_window(id: u64, workspace_id: u64, width: i32, height: i32) -> Window {
    Window {
        id,
        title: None,
        app_id: None,
        pid: None,
        workspace_id: Some(workspace_id),
        is_focused: false,
        is_floating: true,
        is_urgent: false,
        layout: WindowLayout {
            pos_in_scrolling_layout: None,
            tile_size: (width as f64, height as f64),
            window_size: (width, height),
            tile_pos_in_workspace_view: None,
            window_offset_in_tile: (0.0, 0.0),
        },
        focus_timestamp: None,
    }
}
