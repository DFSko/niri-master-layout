use std::collections::HashMap;

use crate::state_file::SavedWindowSize;

pub fn map_desired_columns(saved: &[SavedWindowSize]) -> HashMap<u64, usize> {
    saved
        .iter()
        .map(|window| (window.id, window.column))
        .collect()
}
