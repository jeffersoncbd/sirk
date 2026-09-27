use std::path::Path;

pub(super) fn resume(path: &Path) -> Result<(), String> {
    sirk::runner::resume(path)?;
    Ok(())
}
