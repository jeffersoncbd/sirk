use super::{OpenRouterAdapter, nonempty::nonempty};
use crate::harness::{HarnessAdapter, HarnessError};
use std::path::Path;

impl OpenRouterAdapter {
    pub(super) fn api_key(&self, directory: &Path) -> Result<String, HarnessError> {
        match self
            .api_key
            .clone()
            .or_else(|| std::env::var("OPENROUTER_API_KEY").ok().and_then(nonempty))
        {
            Some(api_key) => Ok(api_key),
            None => self
                .dotenv_value(directory, "OPENROUTER_API_KEY")?
                .ok_or_else(|| HarnessError::InvalidConfiguration {
                    adapter: self.id(),
                    message: "OPENROUTER_API_KEY is required".to_owned(),
                }),
        }
    }
}
