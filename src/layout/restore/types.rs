#[derive(Clone)]
pub struct RestoreTarget {
    pub has_foreign_windows: bool,
    pub column: ColumnSnapshot,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RestoreDecision {
    Done,
    ExpelForeign,
    MoveColumn,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ColumnSnapshot {
    pub index: usize,
    pub window_ids: Vec<u64>,
}
