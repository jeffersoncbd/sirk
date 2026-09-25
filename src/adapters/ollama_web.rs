use std::{collections::BTreeMap, io, path::Path};

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

impl Default for OllamaWebAdapter {
    fn default() -> Self {
        Self {
            executable: "curl".to_owned(),
            base_url: None,
            api_key: None,
        }
    }
}

impl OllamaWebAdapter {
    pub fn new(
        executable: impl Into<String>,
        base_url: impl Into<String>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            executable: executable.into(),
            base_url: nonempty(base_url.into()),
            api_key: api_key.filter(|key| !key.trim().is_empty()),
        }
    }

    fn endpoint(&self, directory: &Path) -> Result<String, HarnessError> {
        let base_url = match self
            .base_url
            .clone()
            .or_else(|| std::env::var("OLLAMA_WEB_URL").ok().and_then(nonempty))
        {
            Some(base_url) => base_url,
            None => self
                .dotenv_value(directory, "OLLAMA_WEB_URL")?
                .unwrap_or_else(|| DEFAULT_URL.to_owned()),
        };
        let base_url = base_url.trim_end_matches('/');
        if base_url.is_empty() {
            return Err(HarnessError::InvalidResponse {
                adapter: self.id(),
                message: "OLLAMA_WEB_URL cannot be empty".to_owned(),
            });
        }
        if base_url.ends_with("/api/generate") {
            Ok(base_url.to_owned())
        } else if base_url.ends_with("/api") {
            Ok(format!("{base_url}/generate"))
        } else {
            Ok(format!("{base_url}/api/generate"))
        }
    }

    fn api_key(&self, directory: &Path) -> Result<Option<String>, HarnessError> {
        if self.api_key.is_some() {
            return Ok(self.api_key.clone());
        }
        if let Ok(api_key) = std::env::var("OLLAMA_API_KEY") {
            return Ok(nonempty(api_key));
        }
        self.dotenv_value(directory, "OLLAMA_API_KEY")
    }

    fn dotenv_value(&self, directory: &Path, key: &str) -> Result<Option<String>, HarnessError> {
        let dotenv = directory.join(".env");
        let variables = match dotenvy::from_path_iter(&dotenv) {
            Ok(variables) => variables,
            Err(dotenvy::Error::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(error) => {
                return Err(HarnessError::InvalidConfiguration {
                    adapter: self.id(),
                    message: format!("cannot read `{}`: {error}", dotenv.display()),
                });
            }
        };
        for variable in variables {
            let (name, value) = variable.map_err(|error| HarnessError::InvalidConfiguration {
                adapter: self.id(),
                message: format!("cannot parse `{}`: {error}", dotenv.display()),
            })?;
            if name == key {
                return Ok(nonempty(value));
            }
        }
        Ok(None)
    }
}

fn nonempty(value: String) -> Option<String> {
    (!value.trim().is_empty()).then_some(value)
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
