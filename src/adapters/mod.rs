mod codex;
mod ollama;
mod ollama_web;
mod opencode;
mod openrouter;
mod resolve;

pub use codex::CodexAdapter;
pub use ollama::OllamaAdapter;
pub use ollama_web::OllamaWebAdapter;
pub use opencode::OpenCodeAdapter;
pub use openrouter::OpenRouterAdapter;

pub use resolve::resolve;

pub const AVAILABLE: &[&str] = &["codex", "ollama", "ollama-web", "opencode", "openrouter"];
