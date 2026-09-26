use std::{fs::File, path::Path};

pub(super) fn sync_parent(path: &Path) -> Result<(), String> {
    File::open(path.parent().unwrap())
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())
}
