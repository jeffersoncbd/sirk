use super::{DEFAULT_URL, OpenRouterAdapter, nonempty::nonempty};
use crate::harness::{HarnessAdapter, HarnessError};
use std::path::Path;

impl OpenRouterAdapter {
    pub(super) fn endpoint(&self, directory: &Path) -> Result<String, HarnessError> {
        let base_url = match self
            .base_url
            .clone()
            .or_else(|| std::env::var("OPENROUTER_URL").ok().and_then(nonempty))
        {
            Some(base_url) => base_url,
            None => self
                .dotenv_value(directory, "OPENROUTER_URL")?
                .unwrap_or_else(|| DEFAULT_URL.to_owned()),
        };
        let base_url = base_url.trim_end_matches('/');
        if base_url.is_empty() {
            return Err(HarnessError::InvalidConfiguration {
                adapter: self.id(),
                message: "OPENROUTER_URL cannot be empty".to_owned(),
            });
        }
        if base_url.ends_with("/chat/completions") {
            Ok(base_url.to_owned())
        } else if base_url.ends_with("/api/v1") {
            Ok(format!("{base_url}/chat/completions"))
        } else {
            Ok(format!("{base_url}/api/v1/chat/completions"))
        }
    }
}
