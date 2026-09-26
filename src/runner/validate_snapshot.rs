use crate::{adapters, history::Snapshot};

use super::all_steps::all_steps;

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
