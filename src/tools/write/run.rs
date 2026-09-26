use std::path::Path;

pub fn write(directory: &Path, path: &str, content: &str, force: bool) -> Result<(), String> {
    super::write_with_options(directory, path, content, force, false)
}
