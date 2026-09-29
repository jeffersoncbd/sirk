//! In-memory conversation state and flow-scoped agent transcripts.
mod create;
mod drop;
mod flow_id;
mod lock;
mod new_flow;
mod path;
mod record_input;
mod record_output;
mod valid_flow_id;
mod validate_flow;

pub use crate::interfaces::{Block, Snapshot};
use std::{fs::File, path::PathBuf};
pub use valid_flow_id::valid_flow_id;

pub struct History {
    path: PathBuf,
    pub snapshot: Snapshot,
    pub blocks: Vec<Block>,
    _lock: File,
}

pub use new_flow::new_flow;
pub use validate_flow::validate_flow;
