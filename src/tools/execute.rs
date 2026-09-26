use std::path::Path;

use super::execute_with_input::execute_with_input;

pub fn execute(name: &str, directory: &Path) -> Result<String, String> {
    execute_with_input(name, "", directory)
}
