mod default;
mod execute_bytes_to;
mod execute_streaming;
mod execute_to;
mod new;
mod render;

use std::process::ExitStatus;

/// Shared process service for features that must run through Bash.
///
/// Commands are rendered with POSIX single-quote escaping before being passed to
/// `bash -lc`; adapters and future services should never interpolate user input
/// into a shell string themselves.
#[derive(Debug, Clone)]
pub struct BashService {
    executable: String,
}

#[derive(Debug)]
pub struct ProcessOutput {
    pub status: ExitStatus,
    pub stdout: String,
}

pub(crate) struct BinaryProcessOutput {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use std::{
        io::{self, Write},
        path::PathBuf,
    };

    use super::*;
    use crate::services::Invocation;

    #[test]
    fn bash_preserves_arguments_and_captures_streamed_bytes() {
        let arguments = [
            "",
            "spaces and 'quotes'",
            "$(exit 7); `exit 8`",
            "line\nbreak",
            "--flag",
        ];
        let invocation = Invocation {
            program: "/usr/bin/printf".to_owned(),
            arguments: std::iter::once("%s\\n".to_owned())
                .chain(arguments.iter().map(|value| (*value).to_owned()))
                .collect(),
            working_directory: std::env::current_dir().unwrap(),
            environment: Default::default(),
        };
        let mut streamed = Vec::new();
        let result = BashService::default()
            .execute_to(&invocation, &mut streamed)
            .unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, format!("{}\n", arguments.join("\n")));
        assert_eq!(streamed, result.stdout.as_bytes());
    }

    #[test]
    fn preserves_exit_status_and_working_directory() {
        let invocation = Invocation {
            program: "/bin/sh".to_owned(),
            arguments: vec!["-c".to_owned(), "pwd; exit 17".to_owned()],
            working_directory: std::env::temp_dir().canonicalize().unwrap(),
            environment: Default::default(),
        };
        let result = BashService::default()
            .execute_to(&invocation, &mut Vec::new())
            .unwrap();
        assert_eq!(result.status.code(), Some(17));
        assert_eq!(
            result.stdout.trim(),
            invocation.working_directory.to_str().unwrap()
        );
    }

    #[test]
    fn reports_a_broken_output_sink() {
        struct BrokenSink;
        impl Write for BrokenSink {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::from(io::ErrorKind::BrokenPipe))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let invocation = Invocation {
            program: "/usr/bin/printf".to_owned(),
            arguments: vec!["hello".to_owned()],
            working_directory: std::env::current_dir().unwrap(),
            environment: Default::default(),
        };
        let error = BashService::default()
            .execute_to(&invocation, &mut BrokenSink)
            .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn renders_each_argument_as_a_single_shell_word() {
        let invocation = Invocation {
            program: "codex".to_owned(),
            arguments: vec![
                "exec".to_owned(),
                "Prompt with spaces; $(not executed) and 'quotes'".to_owned(),
            ],
            working_directory: PathBuf::from("/workspace"),
            environment: Default::default(),
        };

        assert_eq!(
            BashService::default().render(&invocation),
            "exec -- 'codex' 'exec' 'Prompt with spaces; $(not executed) and '\"'\"'quotes'\"'\"''"
        );
    }

    #[test]
    fn passes_marked_environment_values_without_rendering_their_contents() {
        let invocation = Invocation {
            program: "/usr/bin/printf".to_owned(),
            arguments: vec!["%s".to_owned(), "$TEST_AUTHORIZATION".to_owned()],
            working_directory: std::env::current_dir().unwrap(),
            environment: [("TEST_AUTHORIZATION".to_owned(), "Bearer secret".to_owned())]
                .into_iter()
                .collect(),
        };

        let command = BashService::default().render(&invocation);
        assert!(command.contains("\"$TEST_AUTHORIZATION\""));
        assert!(!command.contains("Bearer secret"));
        let output = BashService::default()
            .execute_to(&invocation, &mut Vec::new())
            .unwrap();
        assert_eq!(output.stdout, "Bearer secret");
    }
}
