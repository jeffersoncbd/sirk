//! Public entry points for workflow execution and recovery.
//!
//! The stateful execution engine lives in `runner/engine.rs`. Keeping this
//! module as a facade makes the stable runner API independent of the engine's
//! internal decomposition.

mod engine;

pub use engine::{continue_with, resume, run, run_interactive_with, run_with};
