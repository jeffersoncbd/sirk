mod codex;
mod ollama;
mod ollama_web;
mod opencode;

use crate::harness::HarnessAdapter;

pub use codex::CodexAdapter;
pub use ollama::OllamaAdapter;
pub use ollama_web::OllamaWebAdapter;
pub use opencode::OpenCodeAdapter;

pub fn resolve(name: &str) -> Option<Box<dyn HarnessAdapter>> {
    match name {
        "codex" => Some(Box::new(CodexAdapter::default())),
        "ollama" => Some(Box::new(OllamaAdapter::default())),
        "ollama-web" => Some(Box::new(OllamaWebAdapter::default())),
        "opencode" => Some(Box::new(OpenCodeAdapter::default())),
        _ => None,
    }
}

pub const AVAILABLE: &[&str] = &["codex", "ollama", "ollama-web", "opencode"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_ollama_adapters_and_advertises_them_to_agent_generation() {
        assert_eq!(resolve("ollama").unwrap().id(), "ollama");
        assert_eq!(resolve("ollama-web").unwrap().id(), "ollama-web");
        assert!(AVAILABLE.contains(&"ollama"));
        assert!(AVAILABLE.contains(&"ollama-web"));
    }
}
