use super::{BashService, BinaryProcessOutput};
use crate::services::Invocation;
use std::{
    io::{self, Read, Write},
    process::{Command, Stdio},
};

impl BashService {
    /// Capture binary output, including Git's NUL-delimited filesystem paths.
    pub(crate) fn execute_bytes_to(
        &self,
        invocation: &Invocation,
        output: &mut impl Write,
    ) -> io::Result<BinaryProcessOutput> {
        let mut child = Command::new(&self.executable)
            .args(["-lc", &self.render(invocation)])
            .current_dir(&invocation.working_directory)
            .envs(&invocation.environment)
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
}
