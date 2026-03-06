use std::path::PathBuf;

const STATE_FILE_NAME_PREFIX: &str = "niri-master-layout";

pub fn state_file_path(workspace_id: u64) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("{STATE_FILE_NAME_PREFIX}-{workspace_id}.state"));
    path
}
