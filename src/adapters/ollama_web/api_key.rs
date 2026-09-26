use super::{OllamaWebAdapter, nonempty::nonempty};
use crate::harness::HarnessError;
use std::path::Path;

impl OllamaWebAdapter {
    pub(super) fn api_key(&self, directory: &Path) -> Result<Option<String>, HarnessError> {
        if self.api_key.is_some() {
            return Ok(self.api_key.clone());
        }
        if let Ok(api_key) = std::env::var("OLLAMA_API_KEY") {
            return Ok(nonempty(api_key));
        }
        self.dotenv_value(directory, "OLLAMA_API_KEY")
    }
}
