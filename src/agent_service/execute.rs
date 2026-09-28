use crate::services::{BashService, Invocation};
use std::io;

pub(super) fn execute(invocation: &Invocation) -> Result<String, String> {
    let result = BashService::default()
        .execute_to(invocation, &mut io::sink())
        .map_err(|error| error.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "{} exited with {}; incomplete output was not committed",
            invocation.program, result.status
        ));
    }
    Ok(result.stdout)
}
