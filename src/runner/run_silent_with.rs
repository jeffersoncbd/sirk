use crate::{services::Invocation, workflow::Workflow};
use std::{collections::BTreeMap, path::Path};

use super::{run_interactive_configured::run_interactive_configured_with, run_with::NoInput};

pub(crate) fn run_silent_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    run_interactive_configured_with(workflow, directory, execute, &mut NoInput, false)
}
