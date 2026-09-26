//! Internal composition of workflow execution.

#[path = "bootstrap.rs"]
mod bootstrap;
#[path = "execution.rs"]
mod execution;
#[path = "external.rs"]
mod external;
#[path = "history_validation.rs"]
mod history_validation;

pub use bootstrap::{resume, run, run_interactive_with, run_with};
pub use execution::continue_with;

#[cfg(test)]
pub(crate) use external::{DUPLICATE_EDIT_RESULT, EDIT_FAILURE_PREFIX};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
