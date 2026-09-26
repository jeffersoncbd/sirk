use super::{BashService, ProcessOutput};
use crate::services::Invocation;
use std::io;

impl BashService {
    pub fn execute_streaming(&self, invocation: &Invocation) -> io::Result<ProcessOutput> {
        self.execute_to(invocation, &mut io::stdout().lock())
    }
}
