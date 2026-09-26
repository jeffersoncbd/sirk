//! CUSTOM-TOOL: execute a project-local Bash script with positional arguments.
mod args;
mod name;
mod run;

pub use args::arguments;
pub use name::valid_name;
pub use run::execute;
