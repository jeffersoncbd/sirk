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

    /// Converts successful process output into the agent's response.
    fn response(&self, stdout: String) -> Result<String, HarnessError> {
        Ok(stdout)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessError {
    UnsupportedOption {
        adapter: &'static str,
        option: &'static str,
    },
    MissingModel {
        adapter: &'static str,
    },
    InvalidResponse {
        adapter: &'static str,
        message: String,
    },
    InvalidConfiguration {
        adapter: &'static str,
        message: String,
    },
}

impl fmt::Display for HarnessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOption { adapter, option } => {
                write!(formatter, "adapter `{adapter}` does not support `{option}`")
            }
            Self::MissingModel { adapter } => {
                write!(formatter, "adapter `{adapter}` requires a model")
            }
            Self::InvalidResponse { adapter, message } => {
                write!(
                    formatter,
                    "adapter `{adapter}` returned an invalid response: {message}"
                )
            }
            Self::InvalidConfiguration { adapter, message } => {
                write!(
                    formatter,
                    "adapter `{adapter}` has invalid configuration: {message}"
                )
            }
        }
    }
}

impl std::error::Error for HarnessError {}
