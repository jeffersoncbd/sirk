use crate::{input::TerminalInput, workflow::Workflow};
use std::{collections::BTreeMap, path::Path};

use super::{execute::execute, run_interactive::run_interactive_with};

pub fn run(workflow: &Workflow, directory: &Path) -> Result<BTreeMap<String, String>, String> {
    run_interactive_with(workflow, directory, execute, &mut TerminalInput)
}
