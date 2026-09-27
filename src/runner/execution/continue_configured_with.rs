use super::super::{history_validation::validate_blocks, validate_snapshot::validate_snapshot};
use super::Engine;
use crate::{history::History, input::UserInput, services::Invocation};
use std::collections::BTreeMap;

pub(crate) fn continue_configured_with(
    history: &mut History,
    execute: &mut impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
    present: bool,
) -> Result<BTreeMap<String, String>, String> {
    validate_snapshot(&history.snapshot)?;
    validate_blocks(history)?;
    let mut outputs = BTreeMap::new();
    let steps = history.snapshot.workflow.steps.clone();
    Engine {
        history,
        execute,
        input,
        cursor: 0,
        present,
    }
    .run_steps(&steps, "", &mut outputs, &mut None)?;
    Ok(outputs)
}
