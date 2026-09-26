use std::collections::BTreeMap;

/// A command description shared by any feature that uses Bash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: String,
    pub arguments: Vec<String>,
    pub working_directory: std::path::PathBuf,
    /// Environment values supplied only to the child process, never rendered
    /// into its shell command.
    pub environment: BTreeMap<String, String>,
}

impl Invocation {
    /// Runs this invocation through a nonempty command prefix without shell parsing.
    /// For example, `["docker", "exec", "container"]` becomes
    /// `docker exec container <program> <arguments...>`.
    pub fn with_prefix(mut self, prefix: &[String]) -> Self {
        let Some((program, arguments)) = prefix.split_first() else {
            return self;
        };
        let original_program = std::mem::replace(&mut self.program, program.clone());
        let mut prefixed_arguments = arguments.to_vec();
        prefixed_arguments.push(original_program);
        prefixed_arguments.append(&mut self.arguments);
        self.arguments = prefixed_arguments;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Invocation;

    #[test]
    fn prefixes_program_and_arguments_without_changing_execution_settings() {
        let invocation = Invocation {
            program: "codex".into(),
            arguments: vec!["exec".into(), "Prompt".into()],
            working_directory: "/workspace".into(),
            environment: [("TOKEN".into(), "secret".into())].into_iter().collect(),
        }
        .with_prefix(&["docker".into(), "exec".into(), "container".into()]);

        assert_eq!(invocation.program, "docker");
        assert_eq!(
            invocation.arguments,
            ["exec", "container", "codex", "exec", "Prompt"]
        );
        assert_eq!(
            invocation.working_directory,
            std::path::PathBuf::from("/workspace")
        );
        assert_eq!(invocation.environment["TOKEN"], "secret");
    }
}
