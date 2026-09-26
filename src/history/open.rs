use super::History;
use std::{fs, path::Path};

impl History {
    pub fn open(path: &Path) -> Result<Self, String> {
        let path = path.canonicalize().map_err(|e| e.to_string())?;
        let lock = Self::lock(&path)?;
        let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let (snapshot, steps, labels) = Self::parse(&source)?;
        Ok(Self {
            path,
            snapshot,
            steps,
            labels,
            _lock: lock,
        })
    }
}
