use serde::Deserialize;

use super::NvidiaApiAdapter;
use crate::harness::{HarnessAdapter, HarnessError, HarnessResponse, TokenUsage};

pub(super) fn parse(
    adapter: &NvidiaApiAdapter,
    stdout: String,
) -> Result<HarnessResponse, HarnessError> {
    #[derive(Deserialize)]
    struct ChatCompletion {
        choices: Vec<Choice>,
        usage: Option<Usage>,
    }
    #[derive(Deserialize)]
    struct Choice {
        message: Message,
    }
    #[derive(Deserialize)]
    struct Message {
        content: String,
    }
    #[derive(Deserialize)]
    struct Usage {
        prompt_tokens: Option<u64>,
        completion_tokens: Option<u64>,
    }

    let response = serde_json::from_str::<ChatCompletion>(&stdout).map_err(|error| {
        HarnessError::InvalidResponse {
            adapter: adapter.id(),
            message: error.to_string(),
        }
    })?;
    let usage = response.usage.and_then(|usage| {
        usage
            .prompt_tokens
            .zip(usage.completion_tokens)
            .map(|(input_tokens, output_tokens)| TokenUsage {
                input_tokens,
                output_tokens,
            })
    });
    let text = response
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .ok_or_else(|| HarnessError::InvalidResponse {
            adapter: adapter.id(),
            message: "response contains no choices".to_owned(),
        })?;
    Ok(HarnessResponse { text, usage })
}
