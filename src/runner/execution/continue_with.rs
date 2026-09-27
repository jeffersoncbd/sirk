use crate::{history::History, input::UserInput, services::Invocation};
use std::collections::BTreeMap;

pub fn continue_with(
    history: &mut History,
    mut execute: impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
) -> Result<BTreeMap<String, String>, String> {
    super::continue_configured_with::continue_configured_with(history, &mut execute, input, true)
}
