use crate::{input::UserInput, services::Invocation, workflow::Workflow};
use std::{collections::BTreeMap, path::Path};

use super::run_interactive_configured::run_interactive_configured_with;

pub fn run_interactive_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
) -> Result<BTreeMap<String, String>, String> {
    run_interactive_configured_with(workflow, directory, execute, input, true)
}
