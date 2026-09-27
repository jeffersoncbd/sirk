use super::OpenCodeAdapter;
use crate::harness::{HarnessAdapter, HarnessError, Invocation, RunRequest};

impl HarnessAdapter for OpenCodeAdapter {
    super::id::adapter_id!();

    fn invocation(&self, request: &RunRequest) -> Result<Invocation, HarnessError> {
        let mut arguments = vec!["run".to_owned()];

        // Do not pass OpenCode's --auto option: it enables automatic approval
        // of permissions not explicitly denied.
        // Supplying a title prevents OpenCode from spending a separate model
        // request to generate one for each short-lived harness session.
        arguments.extend(["--title".to_owned(), "S.I.R.K.".to_owned()]);
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
