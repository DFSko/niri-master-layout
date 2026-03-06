#![allow(dead_code, unused_imports)]

pub mod fake_client;
pub mod windows;

pub use fake_client::FakeClient;
pub use windows::{make_floating_window, make_tiled_window};
