mod api_key;
mod default;
mod dotenv;
mod endpoint;
mod new;
mod nonempty;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::json;

use crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest};

const DEFAULT_URL: &str = "https://openrouter.ai/api/v1/chat/completions";

/// Translates requests into non-streaming calls to OpenRouter's chat API.
#[derive(Debug)]
pub struct OpenRouterAdapter {
    executable: String,
    base_url: Option<String>,
    api_key: Option<String>,
}

impl HarnessAdapter for OpenRouterAdapter {
    fn id(&self) -> &'static str {
        "openrouter"
    }

    fn invocation(&self, request: &RunRequest) -> Result<Invocation, HarnessError> {
        let model = request
            .model
            .as_ref()
            .ok_or(HarnessError::MissingModel { adapter: self.id() })?;
        if request.event_stream {
            return Err(HarnessError::UnsupportedOption {
                adapter: self.id(),
                option: "event streams",
            });
        }

        let mut environment = BTreeMap::new();
        environment.insert(
            "OPENROUTER_AUTHORIZATION".to_owned(),
            format!(
                "Authorization: Bearer {}",
                self.api_key(&request.working_directory)?
            ),
        );
        Ok(Invocation {
            program: self.executable.clone(),
            arguments: vec![
                // Keep OpenRouter's JSON error body visible on HTTP failures.
                "--fail-with-body".to_owned(),
                "--silent".to_owned(),
                "--show-error".to_owned(),
                "--request".to_owned(),
                "POST".to_owned(),
                "--header".to_owned(),
                "Content-Type: application/json".to_owned(),
                "--header".to_owned(),
                "$OPENROUTER_AUTHORIZATION".to_owned(),
                "--data".to_owned(),
                json!({
                    "model": model,
                    "messages": [{ "role": "user", "content": request.prompt }],
                    "stream": false,
                })
                .to_string(),
                self.endpoint(&request.working_directory)?,
            ],
            working_directory: request.working_directory.clone(),
            environment,
        })
    }

    fn response(&self, stdout: String) -> Result<String, HarnessError> {
        #[derive(Deserialize)]
        struct ChatCompletion {
            choices: Vec<Choice>,
        }
        #[derive(Deserialize)]
        struct Choice {
            message: Message,
        }
        #[derive(Deserialize)]
        struct Message {
            content: String,
        }

        let response = serde_json::from_str::<ChatCompletion>(&stdout).map_err(|error| {
            HarnessError::InvalidResponse {
                adapter: self.id(),
                message: error.to_string(),
            }
        })?;
        response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| HarnessError::InvalidResponse {
                adapter: self.id(),
                message: "response contains no choices".to_owned(),
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    fn request(directory: PathBuf) -> RunRequest {
        RunRequest {
            prompt: "Fix the failing test".to_owned(),
            working_directory: directory,
            model: Some("openai/gpt-5".to_owned()),
            event_stream: false,
        }
    }

    #[test]
    fn posts_a_non_streaming_chat_request_without_exposing_the_api_key() {
        let invocation = OpenRouterAdapter::new(
            "curl",
            "https://example.test/openrouter/api/v1/",
            Some("secret".to_owned()),
        )
        .invocation(&request(PathBuf::from("/workspace")))
        .unwrap();

        assert_eq!(
            invocation.arguments.last(),
            Some(&"https://example.test/openrouter/api/v1/chat/completions".to_owned())
        );
        assert!(
            invocation
                .arguments
                .contains(&"--fail-with-body".to_owned())
        );
        assert!(invocation.arguments.iter().any(|argument| {
            argument.contains("\"messages\"") && argument.contains("\"stream\":false")
        }));
        assert!(
            !invocation
                .arguments
                .iter()
                .any(|argument| argument.contains("secret"))
        );
        assert_eq!(
            invocation.environment.get("OPENROUTER_AUTHORIZATION"),
            Some(&"Authorization: Bearer secret".to_owned())
        );
    }

    #[test]
    fn extracts_the_first_choice_content_and_rejects_missing_choices() {
        let adapter = OpenRouterAdapter::new("curl", "https://example.test", Some("key".into()));
        assert_eq!(
            adapter
                .response(r#"{"choices":[{"message":{"content":"Done."}}]}"#.to_owned())
                .unwrap(),
            "Done."
        );
        assert!(adapter.response(r#"{"choices":[]}"#.to_owned()).is_err());
    }

    #[test]
    fn reads_configuration_from_the_execution_directory_dotenv() {
        let directory = std::env::temp_dir().join(format!(
            "openrouter-dotenv-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        fs::write(
            directory.join(".env"),
            "OPENROUTER_API_KEY=dotenv-secret\nOPENROUTER_URL=https://remote.example/api/v1\n",
        )
        .unwrap();

        let invocation = OpenRouterAdapter::default()
            .invocation(&request(directory.clone()))
            .unwrap();
        assert_eq!(
            invocation.environment.get("OPENROUTER_AUTHORIZATION"),
            Some(&"Authorization: Bearer dotenv-secret".to_owned())
        );
        assert_eq!(
            invocation.arguments.last(),
            Some(&"https://remote.example/api/v1/chat/completions".to_owned())
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn requires_a_model_and_api_key_and_rejects_event_streams() {
        let adapter = OpenRouterAdapter::new("curl", "https://example.test", None);
        let missing_model = RunRequest {
            model: None,
            ..request(PathBuf::from("/workspace"))
        };
        assert_eq!(
            adapter.invocation(&missing_model),
            Err(HarnessError::MissingModel {
                adapter: "openrouter"
            })
        );

        let missing_key = request(PathBuf::from("/workspace"));
        assert!(matches!(
            adapter.invocation(&missing_key),
            Err(HarnessError::InvalidConfiguration { .. })
        ));

        let event_stream = RunRequest {
            event_stream: true,
            ..request(PathBuf::from("/workspace"))
        };
        assert_eq!(
            adapter.invocation(&event_stream),
            Err(HarnessError::UnsupportedOption {
                adapter: "openrouter",
                option: "event streams",
            })
        );
    }
}
