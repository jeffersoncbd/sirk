mod bash;

use std::collections::BTreeMap;

pub use bash::{BashService, ProcessOutput};

/// A command description shared by any feature that uses Bash
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: std::path::PathBuf,
    /// Environment values supplied only to the child process, never rendered
    /// into its shell command.
    pub environment: BTreeMap<String, String>,
}
