use super::CodexAdapter;

impl CodexAdapter {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
        }
    }
}
