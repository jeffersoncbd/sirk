mod bash;

pub use bash::{BashService, ProcessOutput};

/// A command description shared by any feature that uses Bash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: std::path::PathBuf,
}
