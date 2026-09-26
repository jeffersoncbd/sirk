use super::OllamaAdapter;

impl OllamaAdapter {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
        }
    }
}
