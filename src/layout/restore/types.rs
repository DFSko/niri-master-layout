#[derive(Clone)]
pub struct TargetRestoreState {
    pub has_foreign_windows: bool,
    pub snapshot: RestoreSnapshot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RestoreDecision {
    Done,
    ExpelForeign,
    MoveColumn,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct RestoreSnapshot {
    pub current_column: usize,
    pub column_window_ids: Vec<u64>,
}
