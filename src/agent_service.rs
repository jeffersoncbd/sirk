//! Transport-independent agent execution used by protocol servers.

mod conversation;
mod delete_request;
mod edit_request;
mod execute;
mod outcome;
mod prompt;
mod run;
mod write_request;

pub(crate) use outcome::{AgentExecution, AgentOutcome};
pub(crate) use run::run;
