use super::BashService;

impl BashService {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
        }
    }
}
