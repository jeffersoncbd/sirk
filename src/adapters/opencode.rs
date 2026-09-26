mod default;
mod new;

use crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest};

/// Translates provider-neutral requests into OpenCode CLI invocations.
#[derive(Debug)]
pub struct OpenCodeAdapter {
    executable: String,
}

impl HarnessAdapter for OpenCodeAdapter {
    fn id(&self) -> &'static str {
        "opencode"
    }

    fn invocation(&self, request: &RunRequest) -> Result<Invocation, HarnessError> {
        let mut arguments = vec!["run".to_owned()];

        // Do not pass OpenCode's --auto option: it enables automatic approval
        // of permissions not explicitly denied.
        // Supplying a title prevents OpenCode from spending a separate model
        // request to generate one for each short-lived harness session.
        arguments.extend(["--title".to_owned(), "new-harness".to_owned()]);
        if request.event_stream {
            arguments.extend(["--format".to_owned(), "json".to_owned()]);
        }
        if let Some(model) = &request.model {
            arguments.extend(["--model".to_owned(), model.clone()]);
        }
        arguments.push("--".to_owned());
        arguments.push(request.prompt.clone());

        Ok(Invocation {
            program: self.executable.clone(),
            arguments,
            working_directory: request.working_directory.clone(),
            environment: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn translates_generic_options_to_opencode_run() {
        let request = RunRequest {
            prompt: "Fix the failing test".to_owned(),
            working_directory: PathBuf::from("/workspace"),
            model: Some("openai/gpt-5.3-codex".to_owned()),
            event_stream: true,
        };

        assert_eq!(
            OpenCodeAdapter::default().invocation(&request).unwrap(),
            Invocation {
                program: "opencode".to_owned(),
                arguments: vec![
                    "run",
                    "--title",
                    "new-harness",
                    "--format",
                    "json",
                    "--model",
                    "openai/gpt-5.3-codex",
                    "--",
                    "Fix the failing test",
                ]
                .into_iter()
                .map(str::to_owned)
                .collect(),
                working_directory: PathBuf::from("/workspace"),
                environment: Default::default(),
            }
        );
    }

    #[test]
    fn does_not_enable_automatic_permission_approval() {
        let request = RunRequest {
            prompt: "Inspect the project".to_owned(),
            working_directory: PathBuf::from("/workspace"),
            model: None,
            event_stream: false,
        };

        let invocation = OpenCodeAdapter::default().invocation(&request).unwrap();

        assert!(
            !invocation
                .arguments
                .iter()
                .any(|argument| argument == "--auto")
        );
    }
}
