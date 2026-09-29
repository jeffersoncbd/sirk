use crate::harness::HarnessAdapter;

use super::{
    CodexAdapter, NvidiaApiAdapter, OllamaAdapter, OllamaWebAdapter, OpenCodeAdapter,
    OpenRouterAdapter,
};

pub fn resolve(name: &str) -> Option<Box<dyn HarnessAdapter>> {
    match name {
        "codex" => Some(Box::new(CodexAdapter::default())),
        "nvidia-api" => Some(Box::new(NvidiaApiAdapter::default())),
        "ollama" => Some(Box::new(OllamaAdapter::default())),
        "ollama-web" => Some(Box::new(OllamaWebAdapter::default())),
        "opencode" => Some(Box::new(OpenCodeAdapter::default())),
        "openrouter" => Some(Box::new(OpenRouterAdapter::default())),
        _ => None,
    }
}

#[cfg(test)]
mod tests;
