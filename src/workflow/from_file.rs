use super::Workflow;
use std::{fs, path::Path};

impl Workflow {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let source = fs::read_to_string(path)
            .map_err(|error| format!("could not read workflow `{}`: {error}", path.display()))?;
        let workflow: Self = serde_yaml::from_str(&source)
            .map_err(|error| format!("invalid workflow `{}`: {error}", path.display()))?;
        workflow.validate()?;
        Ok(workflow)
    }
}
