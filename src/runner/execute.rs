use crate::services::{BashService, Invocation};

pub(super) fn execute(invocation: &Invocation) -> Result<String, String> {
    let result = BashService::default()
        .execute_streaming(invocation)
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "{} exited with {}; incomplete output was not committed",
            invocation.program, result.status
        ));
    }
    Ok(result.stdout)
}
