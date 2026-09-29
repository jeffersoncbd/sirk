use std::path::Path;

use super::{NvidiaApiAdapter, nonempty::nonempty};
use crate::harness::{HarnessAdapter, HarnessError};

impl NvidiaApiAdapter {
    pub(super) fn api_key(&self, directory: &Path) -> Result<String, HarnessError> {
        self.api_key
            .clone()
            .or_else(|| std::env::var("NVIDIA_API_KEY").ok().and_then(nonempty))
            .or(self.dotenv_value(directory, "NVIDIA_API_KEY")?)
            .ok_or_else(|| HarnessError::InvalidConfiguration {
                adapter: self.id(),
                message: "NVIDIA_API_KEY is required".to_owned(),
            })
    }
}
