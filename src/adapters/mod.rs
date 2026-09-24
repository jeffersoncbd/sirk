mod codex;

use crate::harness::HarnessAdapter;

pub use codex::CodexAdapter;

pub fn resolve(name: &str) -> Option<Box<dyn HarnessAdapter>> {
    match name {
        "codex" => Some(Box::new(CodexAdapter::default())),
        _ => None,
    }
}

pub const AVAILABLE: &[&str] = &["codex"];
