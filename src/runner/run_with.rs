mod no_input;

use crate::{services::Invocation, workflow::Workflow};
use std::{collections::BTreeMap, path::Path};

use super::run_interactive::run_interactive_with;
use no_input::NoInput;

pub fn run_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    run_interactive_with(workflow, directory, execute, &mut NoInput)
}
