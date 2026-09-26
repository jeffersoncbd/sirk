use super::{DEFAULT_URL, OllamaWebAdapter, nonempty::nonempty};
use crate::harness::{HarnessAdapter, HarnessError};
use std::path::Path;

impl OllamaWebAdapter {
    pub(super) fn endpoint(&self, directory: &Path) -> Result<String, HarnessError> {
        let base_url = match self
            .base_url
            .clone()
            .or_else(|| std::env::var("OLLAMA_WEB_URL").ok().and_then(nonempty))
        {
            Some(base_url) => base_url,
            None => self
                .dotenv_value(directory, "OLLAMA_WEB_URL")?
                .unwrap_or_else(|| DEFAULT_URL.to_owned()),
        };
        let base_url = base_url.trim_end_matches('/');
        if base_url.is_empty() {
            return Err(HarnessError::InvalidResponse {
                adapter: self.id(),
                message: "OLLAMA_WEB_URL cannot be empty".to_owned(),
            });
        }
        if base_url.ends_with("/api/generate") {
            Ok(base_url.to_owned())
        } else if base_url.ends_with("/api") {
            Ok(format!("{base_url}/generate"))
        } else {
            Ok(format!("{base_url}/api/generate"))
        }
    }
}
