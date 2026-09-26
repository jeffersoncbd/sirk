use crate::harness::HarnessAdapter;

use super::{CodexAdapter, OllamaAdapter, OllamaWebAdapter, OpenCodeAdapter, OpenRouterAdapter};

pub fn resolve(name: &str) -> Option<Box<dyn HarnessAdapter>> {
    match name {
        "codex" => Some(Box::new(CodexAdapter::default())),
        "ollama" => Some(Box::new(OllamaAdapter::default())),
        "ollama-web" => Some(Box::new(OllamaWebAdapter::default())),
        "opencode" => Some(Box::new(OpenCodeAdapter::default())),
        "openrouter" => Some(Box::new(OpenRouterAdapter::default())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_web_adapters_and_advertises_them_to_agent_generation() {
        assert_eq!(resolve("ollama").unwrap().id(), "ollama");
        assert_eq!(resolve("ollama-web").unwrap().id(), "ollama-web");
        assert_eq!(resolve("openrouter").unwrap().id(), "openrouter");
        assert!(super::super::AVAILABLE.contains(&"ollama"));
        assert!(super::super::AVAILABLE.contains(&"ollama-web"));
        assert!(super::super::AVAILABLE.contains(&"openrouter"));
    }
}
