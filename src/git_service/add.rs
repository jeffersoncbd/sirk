use super::{project::project, run::run};
use std::path::Path;

pub(crate) fn add(directory: &Path) -> Result<(), String> {
    let project = project(directory)?;
    run(&project.directory, &["add", "--all", "--", "."])?;
    Ok(())
}
