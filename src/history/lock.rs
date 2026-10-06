use super::History;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

impl History {
    pub(super) fn lock(path: &Path) -> Result<File, String> {
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path.with_file_name(".lock"))
            .map_err(|e| e.to_string())?;
        lock.try_lock()
            .map_err(|e| format!("history is already in use or cannot be locked: {e}"))?;
        Ok(lock)
    }
}
