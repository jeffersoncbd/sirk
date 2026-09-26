//! WRITE: create a UTF-8 file within the execution directory.
mod directory;
mod run;
mod run_with;

pub use run::write;
pub use run_with::write_with_options;
