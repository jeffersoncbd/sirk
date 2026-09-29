use super::History;
use std::path::Path;

impl History {
    pub(super) fn resume_path(
        directory: &Path,
        flow_id: &str,
    ) -> Result<std::path::PathBuf, String> {
        Ok(directory
            .canonicalize()
            .map_err(|error| error.to_string())?
            .join("history")
            .join(format!("RESUME_{flow_id}.log")))
    }
}
