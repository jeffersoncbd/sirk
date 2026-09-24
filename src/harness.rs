use std::fmt;
use std::path::PathBuf;

pub use crate::services::Invocation;

/// A provider-neutral request to run a coding-agent harness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    pub prompt: String,
    pub working_directory: PathBuf,
    pub model: Option<String>,
    pub event_stream: bool,
}

/// The integration boundary for a coding-agent CLI.
pub trait HarnessAdapter {
    fn id(&self) -> &'static str;
    fn invocation(&self, request: &RunRequest) -> Result<Invocation, HarnessError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessError {
    UnsupportedOption {
        adapter: &'static str,
        option: &'static str,
    },
}

impl fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOption { adapter, option } => {
                write!(formatter, "adapter `{adapter}` does not support `{option}`")
            }
        }
    }
}

impl std::error::Error for HarnessError {}
