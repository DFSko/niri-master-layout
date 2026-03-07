mod client;
mod socket;
mod window;

pub use client::IpcClient;
pub use socket::SocketClient;
pub use window::{set_height, set_width, set_width_percent};
