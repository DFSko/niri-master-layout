#![allow(dead_code)]

use std::io;
use std::path::{Path, PathBuf};

pub mod fake_client;
pub mod windows;

pub fn unique_temp_state_path() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    std::env::temp_dir().join(format!("niri-master-layout-test-{nanos}.state"))
}

pub fn remove_file_if_exists(path: &Path) {
    if let Err(error) = std::fs::remove_file(path)
        && error.kind() != io::ErrorKind::NotFound
    {
        panic!("failed to remove test file {}: {error}", path.display());
    }
}
