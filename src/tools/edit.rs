//! Version-checked agent edits with durable preparation and plain diffs.
mod apply_to;
mod commit;
mod diff;
mod display;
mod pending_validate;
mod prepare;
mod read_optional;
mod render;
mod request_validate;
mod sync_parent;
mod target;
mod version;

use serde::{Deserialize, Serialize};

pub use display::display;
pub use version::version;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Insert,
    Delete,
    Replace,
    Prepend,
    Append,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub path: String,
    pub operation: Operation,
    pub line: Option<usize>,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub version: Option<String>,
    pub input: String,
}

/// Stored in the existing INPUT block. Preparation is committed before mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pending {
    pub request: Request,
    pub before: Option<String>,
    #[serde(default)]
    pub was_missing: bool,
}

#[cfg(test)]
#[path = "edit/tests.rs"]
mod tests;
