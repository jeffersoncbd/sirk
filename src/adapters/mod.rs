mod codex;
mod ollama;
mod opencode;

use crate::harness::HarnessAdapter;

pub use codex::CodexAdapter;
pub use ollama::OllamaAdapter;
pub use opencode::OpenCodeAdapter;

pub fn resolve(name: &str) -> Option<Box<dyn HarnessAdapter>> {
    match name {
        "codex" => Some(Box::new(CodexAdapter::default())),
        "ollama" => Some(Box::new(OllamaAdapter::default())),
        "opencode" => Some(Box::new(OpenCodeAdapter::default())),
        _ => None,
    }
}

pub const AVAILABLE: &[&str] = &["codex", "ollama", "opencode"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_ollama_and_advertises_it_to_agent_generation() {
        assert_eq!(resolve("ollama").unwrap().id(), "ollama");
        assert!(AVAILABLE.contains(&"ollama"));
    }
}
