use super::{Agent, valid_id};
use std::{fs, path::Path};

impl Agent {
    pub fn load(directory: &Path, id: &str) -> Result<Self, String> {
        if !valid_id(id) {
            return Err(format!(
                "invalid agent name `{id}`; use letters, digits, `_` or `-`"
            ));
        }
        let path = directory.join(format!("{id}.md"));
        let source = fs::read_to_string(&path).map_err(|error| {
            format!(
                "could not load agent `{id}` from `{}`: {error}",
                path.display()
            )
        })?;
        Self::parse(id, &source)
            .map_err(|error| format!("invalid agent file `{}`: {error}", path.display()))
    }
}
