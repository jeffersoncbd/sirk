//! Shared types used across independent application modules.

mod harness;
mod history;
mod input;
mod invocation;
mod workflow;

pub use harness::{HarnessAdapter, HarnessError, RunRequest};
pub use history::{Block, Snapshot};
pub use input::UserInput;
pub use invocation::Invocation;
pub use workflow::{EditCoordinate, Step, StepInput, Workflow};
