//! Editable transcript and durable execution state.
mod create;
mod drop;
mod lock;
mod reserved;
mod save;

pub use crate::interfaces::{Block, Snapshot};
use std::{fs::File, path::PathBuf};
const TITLE: &str = "S.I.R.K. — CONVERSATION HISTORY v3\n---\n";
const SEPARATOR: &str = "============================================================";

pub struct History {
    pub path: PathBuf,
    pub snapshot: Snapshot,
    pub blocks: Vec<Block>,
    _lock: File,
}
