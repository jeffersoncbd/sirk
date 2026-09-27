//! Public entry points for workflow execution and recovery.
//!
//! The stateful execution engine is composed below this module. Keeping this
//! module as a facade makes the stable runner API independent of the internal
//! decomposition.

mod all_steps;
mod execute;
#[path = "execution.rs"]
mod execution;
#[path = "external.rs"]
mod external;
#[path = "history_validation.rs"]
mod history_validation;
mod question;
mod resume;
mod run;
mod run_interactive;
mod run_interactive_configured;
mod run_silent_with;
mod run_with;
mod validate_snapshot;

pub use execution::continue_with;
pub use resume::resume;
pub use run::run;
pub use run_interactive::run_interactive_with;
pub(crate) use run_silent_with::run_silent_with;
pub use run_with::run_with;

#[cfg(test)]
pub(crate) use external::{DUPLICATE_EDIT_RESULT, EDIT_FAILURE_PREFIX};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
