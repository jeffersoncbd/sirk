mod default;
mod new;

use crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest};

#[derive(Debug)]
pub struct CodexAdapter {
    executable: String,
}

impl HarnessAdapter for CodexAdapter {
    fn id(&self) -> &'static str {
        "codex"
    }

    fn invocation(&self, request: &RunRequest) -> Result<Invocation, HarnessError> {
        let mut arguments = vec!["exec".to_owned()];

        // Workflows can be stored outside a Git repository, for example in a
        // directory containing only orchestration files.
        arguments.push("--skip-git-repo-check".to_owned());
        arguments.extend(["--sandbox".to_owned(), "read-only".to_owned()]);
        arguments.extend(["-c".to_owned(), "approval_policy=\"never\"".to_owned()]);

        if request.event_stream {
            arguments.push("--json".to_owned());
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
    fn translates_generic_options_to_codex_exec() {
        let request = RunRequest {
            prompt: "Fix the failing test".to_owned(),
            working_directory: PathBuf::from("/workspace"),
            model: Some("gpt-5.3-codex".to_owned()),
            event_stream: true,
        };

        assert_eq!(
            CodexAdapter::default().invocation(&request).unwrap(),
            Invocation {
                program: "codex".to_owned(),
                arguments: vec![
                    "exec",
                    "--skip-git-repo-check",
                    "--sandbox",
                    "read-only",
                    "-c",
                    "approval_policy=\"never\"",
                    "--json",
                    "--model",
                    "gpt-5.3-codex",
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
}
