use std::path::Path;

pub(super) fn resume(path: &Path) -> Result<(), String> {
    new_harness::runner::resume(path)?;
    Ok(())
}
