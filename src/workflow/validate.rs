use super::{Workflow, validate_steps};
use std::collections::{BTreeMap, BTreeSet};

impl Workflow {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!(
                "unsupported workflow version `{}` (expected 1)",
                self.version
            ));
        }
        if self.steps.is_empty() {
            return Err("a workflow requires at least one step".to_owned());
        }
        validate_steps(
            &self.steps,
            &mut BTreeMap::new(),
            None,
            &mut BTreeSet::new(),
        )
    }
}
