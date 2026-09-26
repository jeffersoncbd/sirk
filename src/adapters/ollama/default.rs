use super::OllamaAdapter;

impl Default for OllamaAdapter {
    fn default() -> Self {
        Self::new("ollama")
    }
}
