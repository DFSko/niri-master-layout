use std::fs;
use std::io;
use std::path::Path;

use crate::state_file::model::SavedLayoutState;

pub fn load_layout_state(path: &Path) -> io::Result<SavedLayoutState> {
    let text = fs::read_to_string(path)?;

    serde_json::from_str(&text).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid state: {error}"),
        )
    })
}
