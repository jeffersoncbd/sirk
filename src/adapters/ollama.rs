use crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest};

/// Translates provider-neutral requests into Ollama CLI invocations.
#[derive(Debug)]
pub struct OllamaAdapter {
    executable: String,
}

impl Default for OllamaAdapter {
    fn default() -> Self {
        Self::new("ollama")
    }
}

impl OllamaAdapter {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
        }
    }
}

impl HarnessAdapter for OllamaAdapter {
    fn id(&self) -> &'static str {
        "ollama"
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

        Ok(Invocation {
            program: self.executable.clone(),
            arguments: vec!["run".to_owned(), model.clone(), request.prompt.clone()],
            working_directory: request.working_directory.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn translates_request_to_ollama_run() {
        let request = RunRequest {
            prompt: "Fix the failing test".to_owned(),
            working_directory: PathBuf::from("/workspace"),
            model: Some("qwen3-coder".to_owned()),
            event_stream: false,
        };

        assert_eq!(
            OllamaAdapter::default().invocation(&request).unwrap(),
            Invocation {
                program: "ollama".to_owned(),
                arguments: ["run", "qwen3-coder", "Fix the failing test"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
                working_directory: PathBuf::from("/workspace"),
            }
        );
    }

    #[test]
    fn requires_a_model_and_rejects_event_streams() {
        let no_model = RunRequest {
            prompt: "Inspect the project".to_owned(),
            working_directory: PathBuf::from("/workspace"),
            model: None,
            event_stream: false,
        };
        assert_eq!(
            OllamaAdapter::default().invocation(&no_model),
            Err(HarnessError::MissingModel { adapter: "ollama" })
        );

        let event_stream = RunRequest {
            model: Some("qwen3-coder".to_owned()),
            event_stream: true,
            ..no_model
        };
        assert_eq!(
            OllamaAdapter::default().invocation(&event_stream),
            Err(HarnessError::UnsupportedOption {
                adapter: "ollama",
                option: "event streams",
            })
        );
    }
}
