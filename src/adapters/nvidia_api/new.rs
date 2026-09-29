use super::{NvidiaApiAdapter, nonempty::nonempty};

impl NvidiaApiAdapter {
    pub fn new(executable: impl Into<String>, api_key: Option<String>) -> Self {
        Self {
            executable: executable.into(),
            api_key: api_key.and_then(nonempty),
        }
    }
}
