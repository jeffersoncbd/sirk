use crate::{
    agents::Agent,
    history::{History, Snapshot},
    input::UserInput,
    services::Invocation,
    workflow::Workflow,
};
use std::{collections::BTreeMap, path::Path};

use super::{all_steps::all_steps, execution::continue_with, validate_snapshot::validate_snapshot};

pub fn run_interactive_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
) -> Result<BTreeMap<String, String>, String> {
    workflow.validate()?;
    let directory = directory.canonicalize().map_err(|e| e.to_string())?;
    let mut agents = BTreeMap::new();
    for step in all_steps(&workflow.steps) {
        let Some(id) = &step.agent else {
            continue;
        };
        if !agents.contains_key(id) {
            agents.insert(id.clone(), Agent::load(&directory.join(".agents"), id)?);
        }
    }
    let snapshot = Snapshot {
        directory,
        workflow: workflow.clone(),
        agents: agents.into_values().collect(),
    };
    validate_snapshot(&snapshot)?;
    continue_with(&mut History::create(snapshot)?, execute, input)
}
