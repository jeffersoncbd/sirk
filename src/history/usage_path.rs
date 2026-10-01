use super::History;
use std::path::Path;

impl History {
    pub(super) fn usage_path(
        directory: &Path,
        flow_id: &str,
    ) -> Result<std::path::PathBuf, String> {
        Ok(directory
            .canonicalize()
            .map_err(|error| error.to_string())?
            .join("history")
            .join(format!("USAGE_{flow_id}.log")))
    }
}
