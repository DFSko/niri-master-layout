use std::fs;
use std::io;
use std::path::Path;

pub fn remove_file_if_exists(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

pub struct CleanupGuard<'a> {
    path: &'a Path,
    keep: bool,
}

impl<'a> CleanupGuard<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path, keep: false }
    }

    pub fn keep(&mut self) {
        self.keep = true;
    }
}

impl Drop for CleanupGuard<'_> {
    fn drop(&mut self) {
        if self.keep {
            return;
        }

        if let Err(error) = remove_file_if_exists(self.path) {
            eprintln!(
                "error remove_stale_state path={} reason={error}",
                self.path.display()
            );
        }
    }
}
