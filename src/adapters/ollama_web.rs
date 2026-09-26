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

const DEFAULT_URL: &str = "https://ollama.com/api";

/// Translates requests into non-streaming calls to Ollama's HTTP API.
#[derive(Debug)]
pub struct OllamaWebAdapter {
    executable: String,
    base_url: Option<String>,
    api_key: Option<String>,
}

impl HarnessAdapter for OllamaWebAdapter {
    fn id(&self) -> &'static str {
        "ollama-web"
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
        let mut arguments = vec![
            "--fail".to_owned(),
            "--silent".to_owned(),
            "--show-error".to_owned(),
            "--request".to_owned(),
            "POST".to_owned(),
            "--header".to_owned(),
            "Content-Type: application/json".to_owned(),
        ];
        if let Some(api_key) = self.api_key(&request.working_directory)? {
            environment.insert(
                "OLLAMA_WEB_AUTHORIZATION".to_owned(),
                format!("Authorization: Bearer {api_key}"),
            );
            arguments.extend([
                "--header".to_owned(),
                "$OLLAMA_WEB_AUTHORIZATION".to_owned(),
            ]);
        }
        arguments.extend([
            "--data".to_owned(),
            json!({ "model": model, "prompt": request.prompt, "stream": false }).to_string(),
            self.endpoint(&request.working_directory)?,
        ]);

        Ok(Invocation {
            program: self.executable.clone(),
            arguments,
            working_directory: request.working_directory.clone(),
            environment,
        })
    }

    fn response(&self, stdout: String) -> Result<String, HarnessError> {
        #[derive(Deserialize)]
        struct GenerateResponse {
            response: String,
        }
        serde_json::from_str::<GenerateResponse>(&stdout)
            .map(|response| response.response)
            .map_err(|error| HarnessError::InvalidResponse {
                adapter: self.id(),
                message: error.to_string(),
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

    #[test]
    fn posts_a_non_streaming_request_and_keeps_the_api_key_out_of_arguments() {
        let request = RunRequest {
            prompt: "Fix the failing test".to_owned(),
            working_directory: PathBuf::from("/workspace"),
            model: Some("qwen3-coder".to_owned()),
            event_stream: false,
        };
        let invocation = OllamaWebAdapter::new(
            "curl",
            "https://example.test/ollama/api/",
            Some("secret".to_owned()),
        )
        .invocation(&request)
        .unwrap();

        assert_eq!(invocation.program, "curl");
        assert_eq!(
            invocation.arguments.last().unwrap(),
            "https://example.test/ollama/api/generate"
        );
        assert!(invocation.arguments.contains(&"--data".to_owned()));
        assert!(
            invocation
                .arguments
                .iter()
                .any(|argument| argument.contains("\"stream\":false"))
        );
        assert!(
            !invocation
                .arguments
                .iter()
                .any(|argument| argument.contains("secret"))
        );
        assert_eq!(
            invocation.environment.get("OLLAMA_WEB_AUTHORIZATION"),
            Some(&"Authorization: Bearer secret".to_owned())
        );
    }

    #[test]
    fn extracts_the_response_field_and_rejects_invalid_json() {
        let adapter = OllamaWebAdapter::new("curl", "https://example.test", None);
        assert_eq!(
            adapter
                .response(r#"{"response":"Done."}"#.to_owned())
                .unwrap(),
            "Done."
        );
        assert!(adapter.response("not json".to_owned()).is_err());
    }

    #[test]
    fn reads_api_key_from_the_execution_directory_dotenv() {
        let directory = std::env::temp_dir().join(format!(
            "ollama-web-dotenv-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        fs::write(
            directory.join(".env"),
            "OLLAMA_API_KEY=dotenv-secret\nOLLAMA_WEB_URL=https://remote.example/api\n",
        )
        .unwrap();
        let request = RunRequest {
            prompt: "Hello".to_owned(),
            working_directory: directory.clone(),
            model: Some("gemma4".to_owned()),
            event_stream: false,
        };
        let invocation = OllamaWebAdapter::default().invocation(&request).unwrap();
        assert_eq!(
            invocation.environment.get("OLLAMA_WEB_AUTHORIZATION"),
            Some(&"Authorization: Bearer dotenv-secret".to_owned())
        );
        assert_eq!(
            invocation.arguments.last(),
            Some(&"https://remote.example/api/generate".to_owned())
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
