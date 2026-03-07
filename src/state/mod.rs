mod cleanup;
mod io;
mod path;
mod restore;
mod types;

pub use cleanup::{CleanupGuard, remove_file_if_exists};
pub use io::{load_state, save_state};
pub use path::state_path;
pub use restore::{restore, restore_if_present};
pub use types::SavedWindowSize;
