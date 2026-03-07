use std::collections::HashMap;

use crate::state::SavedWindowSize;

pub fn desired_columns_by_id(saved: &[SavedWindowSize]) -> HashMap<u64, usize> {
    saved
        .iter()
        .map(|window| (window.id, window.column))
        .collect()
}
