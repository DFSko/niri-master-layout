use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SavedWindowSize {
    pub id: u64,
    pub width: i32,
    pub height: i32,
    pub column: usize,
    pub row: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SavedLayoutState {
    pub workspace_id: u64,
    pub master_id: u64,
    pub windows: Vec<SavedWindowSize>,
}
