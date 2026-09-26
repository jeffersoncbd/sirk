//! Sequential conversations reconstructed from an editable transcript.
use crate::{
    adapters,
    agents::Agent,
    history::{History, Snapshot},
    input::{TerminalInput, UserInput},
    services::{BashService, Invocation},
    workflow::{Step, Workflow},
};
use std::{collections::BTreeMap, path::Path};

use super::execution::continue_with;

fn execute(invocation: &Invocation) -> Result<String, String> {
    let result = BashService::default()
        .execute_streaming(invocation)
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "{} exited with {}; incomplete output was not committed",
            invocation.program, result.status
        ));
    }
    Ok(result.stdout)
}
pub fn run(workflow: &Workflow, directory: &Path) -> Result<BTreeMap<String, String>, String> {
    run_interactive_with(workflow, directory, execute, &mut TerminalInput)
}
pub fn resume(path: &Path) -> Result<BTreeMap<String, String>, String> {
    continue_with(&mut History::open(path)?, execute, &mut TerminalInput)
}
pub fn run_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    struct NoInput;
    impl UserInput for NoInput {
        fn ask(&mut self, _: &str) -> Result<String, String> {
            Err("this execution requires user input".into())
        }
    }
    run_interactive_with(workflow, directory, execute, &mut NoInput)
}
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
pub(super) fn validate_snapshot(snapshot: &Snapshot) -> Result<(), String> {
    snapshot.workflow.validate()?;
    if !snapshot.directory.is_dir() {
        return Err("execution directory no longer exists".into());
    }
    for step in all_steps(&snapshot.workflow.steps) {
        let Some(id) = &step.agent else {
            continue;
        };
        let agent = snapshot
            .agents
            .iter()
            .find(|a| &a.id == id)
            .ok_or("missing agent configuration")?;
        adapters::resolve(&agent.adapter)
            .ok_or_else(|| format!("unknown adapter `{}`", agent.adapter))?;
        if agent.json {
            return Err("JSON event streams cannot be used with resumable conversations".into());
        }
        if agent.ask.as_ref().is_some_and(|s| s.trim().is_empty()) {
            return Err("`ask` cannot be empty".into());
        }
    }
    Ok(())
}
pub(super) fn question(text: &str) -> Option<&str> {
    text.trim_start().strip_prefix("ASK:").map(str::trim)
}

fn all_steps(steps: &[Step]) -> Vec<&Step> {
    let mut result = Vec::new();
    for step in steps {
        result.push(step);
        result.extend(all_steps(&step.iter));
        result.extend(all_steps(&step.is_true));
        result.extend(all_steps(&step.is_false));
    }
    result
}
