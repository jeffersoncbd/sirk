use crate::{input::UserInput, services::BashService};
use std::path::{Path, PathBuf};

pub fn create(directory: &Path, input: &mut impl UserInput) -> Result<PathBuf, String> {
    super::create_with(directory, input, |invocation| {
        let result = BashService::default()
            .execute_streaming(invocation)
            .map_err(|e| format!("agent generation failed: {e}"))?;
        if !result.status.success() {
            return Err(format!(
                "agent generation exited with {}; no agent was saved",
                result.status
            ));
        }
        Ok(result.stdout)
    })
}
