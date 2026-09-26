use std::path::{Path, PathBuf};

pub(super) fn workflow_path(name: &str) -> Result<PathBuf, String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        return Err(format!(
            "invalid flow name `{name}`; use letters, digits, `_` or `-`"
        ));
    }
    Ok(Path::new("flows").join(format!("{name}.yml")))
}
