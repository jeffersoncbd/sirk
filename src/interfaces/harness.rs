mod display;

use crate::interfaces::Invocation;
use std::path::PathBuf;

/// A provider-neutral request to run a coding-agent harness.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    pub prompt: String,
    pub working_directory: PathBuf,
    pub model: Option<String>,
    pub event_stream: bool,
}

/// Token counts reported by a provider for one model invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

/// A normalized harness response with optional provider-reported usage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessResponse {
    pub text: String,
    pub usage: Option<TokenUsage>,
}

/// Errors at the boundary between the workflow engine and a harness adapter.
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

impl std::error::Error for HarnessError {}

/// The integration boundary for a coding-agent CLI.
pub trait HarnessAdapter {
    fn id(&self) -> &'static str;
    fn invocation(&self, request: &RunRequest) -> Result<Invocation, HarnessError>;

    fn response(&self, stdout: String) -> Result<HarnessResponse, HarnessError> {
        Ok(HarnessResponse {
            text: stdout,
            usage: None,
        })
    }
}
