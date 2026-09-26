use super::{OpenRouterAdapter, nonempty::nonempty};

impl OpenRouterAdapter {
    pub fn new(
        executable: impl Into<String>,
        base_url: impl Into<String>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            executable: executable.into(),
            base_url: nonempty(base_url.into()),
            api_key: api_key.filter(|key| !key.trim().is_empty()),
        }
    }
}
