use super::{Pending, read_optional::read_optional, sync_parent::sync_parent, target::target};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

impl Pending {
    pub fn commit(&self, directory: &Path) -> Result<String, String> {
        self.validate()?;
        let before = self.before.as_deref().ok_or("EDIT is not prepared")?;
        let after = self.request.apply_to(before)?;
        let prepared_target = target(directory, &self.request.path, self.was_missing)?;
        let current = read_optional(&prepared_target)?;
        if current.as_deref() == Some(&after) {
            File::open(&prepared_target)
                .and_then(|f| f.sync_all())
                .map_err(|e| e.to_string())?;
            sync_parent(&prepared_target)?;
            return self.diff();
        }
        let expected = if self.was_missing { None } else { Some(before) };
        if current.as_deref() != expected {
            return Err(
                "EDIT conflict: file differs from both prepared and resulting content".into(),
            );
        }
        if self.was_missing {
            fs::create_dir_all(prepared_target.parent().unwrap()).map_err(|e| e.to_string())?;
        }
        let target = target(directory, &self.request.path, self.was_missing)?;
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (temporary, mut file) = loop {
            let name = format!(
                ".sirk-edit-{}-{}.tmp",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            );
            let temporary = target.parent().unwrap().join(name);
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
            {
                Ok(file) => break (temporary, file),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.to_string()),
            }
        };
        let result = (|| -> Result<(), String> {
            if !self.was_missing {
                file.set_permissions(
                    fs::metadata(&target)
                        .map_err(|e| e.to_string())?
                        .permissions(),
                )
                .map_err(|e| e.to_string())?;
            }
            file.write_all(after.as_bytes())
                .and_then(|_| file.sync_all())
                .map_err(|e| e.to_string())?;
            // Detect ordinary intervening edits immediately before replacement.
            if target != super::target::target(directory, &self.request.path, self.was_missing)?
                || read_optional(&target)?.as_deref() != expected
            {
                return Err("EDIT conflict: file changed during preparation".into());
            }
            if self.was_missing {
                // Publish a complete file atomically without replacing a concurrently created target.
                fs::hard_link(&temporary, &target)
                    .map_err(|e| format!("EDIT cannot create target without overwriting: {e}"))?;
                fs::remove_file(&temporary).map_err(|e| e.to_string())?;
            } else {
                fs::rename(&temporary, &target).map_err(|e| e.to_string())?;
            }
            sync_parent(&target)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
        self.diff()
    }
}
