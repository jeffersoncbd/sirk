use std::io::{self, Read, Write};
use std::process::{Command, ExitStatus, Stdio};

use super::Invocation;

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

impl Default for BashService {
    fn default() -> Self {
        Self::new("bash")
    }
}

impl BashService {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
        }
    }

    pub fn execute_streaming(&self, invocation: &Invocation) -> io::Result<ProcessOutput> {
        self.execute_to(invocation, &mut io::stdout().lock())
    }

    /// Streams stdout to a caller-owned sink and captures the same bytes.
    /// Stderr stays attached; stdin is closed so the orchestrator owns user input.
    pub fn execute_to(
        &self,
        invocation: &Invocation,
        output: &mut impl Write,
    ) -> io::Result<ProcessOutput> {
        let result = self.execute_bytes_to(invocation, output)?;
        let stdout = String::from_utf8(result.stdout)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        Ok(ProcessOutput {
            status: result.status,
            stdout,
        })
    }

    /// Capture binary output, including Git's NUL-delimited filesystem paths.
    pub(crate) fn execute_bytes_to(
        &self,
        invocation: &Invocation,
        output: &mut impl Write,
    ) -> io::Result<BinaryProcessOutput> {
        let mut child = Command::new(&self.executable)
            .args(["-lc", &self.render(invocation)])
            .current_dir(&invocation.working_directory)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let mut child_stdout = child.stdout.take().expect("stdout was piped");
        let captured = (|| -> io::Result<Vec<u8>> {
            let mut captured = Vec::new();
            let mut buffer = [0; 8_192];
            loop {
                let bytes_read = match child_stdout.read(&mut buffer) {
                    Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                    result => result?,
                };
                if bytes_read == 0 {
                    break;
                }
                output.write_all(&buffer[..bytes_read])?;
                output.flush()?;
                captured.extend_from_slice(&buffer[..bytes_read]);
            }
            Ok(captured)
        })();
        drop(child_stdout);
        if let Err(error) = captured {
            // Reap the child even when the terminal or pipe stops accepting output.
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }
        let status = child.wait()?;
        Ok(BinaryProcessOutput {
            status,
            stdout: captured?,
        })
    }

    pub fn render(&self, invocation: &Invocation) -> String {
        std::iter::once("exec --".to_owned())
            .chain(std::iter::once(shell_quote(&invocation.program)))
            .chain(
                invocation
                    .arguments
                    .iter()
                    .map(|argument| shell_quote(argument)),
            )
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

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
        };

        assert_eq!(
            BashService::default().render(&invocation),
            "exec -- 'codex' 'exec' 'Prompt with spaces; $(not executed) and '\"'\"'quotes'\"'\"''"
        );
    }
}
