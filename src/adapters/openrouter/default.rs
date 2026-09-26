use super::OpenRouterAdapter;

impl Default for OpenRouterAdapter {
    fn default() -> Self {
        Self {
            executable: "curl".to_owned(),
            base_url: None,
            api_key: None,
        }
    }
}
