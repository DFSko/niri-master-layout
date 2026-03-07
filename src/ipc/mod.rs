mod client;
mod socket;
mod window;

pub use client::IpcClient;
pub use socket::SocketClient;
pub use window::{
    focus, focus_best_effort, focus_with_width, set_height, set_width, set_width_percent,
    set_width_percent_for_windows,
};
