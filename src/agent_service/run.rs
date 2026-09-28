use crate::{
    adapters,
    agents::Agent,
    history::{History, Snapshot},
};
use std::path::Path;

pub(crate) fn run(directory: &Path, agent: String, input: String) -> Result<String, String> {
    let directory = directory
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let agent = Agent::load(&directory.join(".agents"), &agent)?;
    adapters::resolve(&agent.adapter)
        .ok_or_else(|| format!("unknown adapter `{}`", agent.adapter))?;
    let mut history = History::create(Snapshot { directory, agent })?;
    super::conversation::conversation(&mut history, input, super::execute::execute)
}
