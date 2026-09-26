use super::OpenCodeAdapter;

impl OpenCodeAdapter {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
        }
    }
}
