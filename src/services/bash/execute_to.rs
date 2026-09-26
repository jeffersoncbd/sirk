use super::{BashService, ProcessOutput};
use crate::services::Invocation;
use std::io::{self, Write};

impl BashService {
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
}
