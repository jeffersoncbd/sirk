use std::{fs, io, path::Path};

pub(super) fn read_ignored(root: &Path, file: &Path) -> Result<bool, String> {
    let source = match fs::read_to_string(root.join(".readignore")) {
        Ok(source) => source,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(_) => return Err("AccessDenied".into()),
    };
    let relative = file
        .strip_prefix(root)
        .expect("validated READ path is inside its root")
        .to_string_lossy()
        .replace('\\', "/");
    Ok(source.lines().map(str::trim).any(|pattern| {
        !pattern.is_empty()
            && !pattern.starts_with('#')
            && super::ignore::matches_ignore(pattern, &relative)
    }))
}
