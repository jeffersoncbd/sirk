use super::NvidiaApiAdapter;

impl Default for NvidiaApiAdapter {
    fn default() -> Self {
        Self {
            executable: "curl".to_owned(),
            api_key: None,
        }
    }
}
