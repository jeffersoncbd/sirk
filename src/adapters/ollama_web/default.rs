use super::OllamaWebAdapter;

impl Default for OllamaWebAdapter {
    fn default() -> Self {
        Self {
            executable: "curl".to_owned(),
            base_url: None,
            api_key: None,
        }
    }
}
