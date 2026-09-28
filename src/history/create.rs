use super::{History, Snapshot};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

impl History {
    pub fn create(snapshot: Snapshot) -> Result<Self, String> {
        let directory = snapshot.directory.join("history");
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let path = directory.join(format!("run-{time}-{}.log", std::process::id()));
        let lock = Self::lock(&path)?;
        let history = Self {
            path,
            snapshot,
            blocks: Vec::new(),
            _lock: lock,
        };
        history.save()?;
        Ok(history)
    }
}
