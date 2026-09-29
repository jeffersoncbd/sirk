//! In-memory conversation state and flow-scoped agent transcripts.
mod create;
mod drop;
mod flow_id;
mod lock;
mod new_flow;
mod path;
mod record_input;
mod record_model_call;
mod record_output;
mod record_resume;
mod record_usage;
mod resume_call;
mod resume_path;
mod valid_flow_id;
mod validate_flow;

pub use crate::interfaces::{Block, Snapshot};
use resume_call::ResumeCall;
use std::{fs::File, path::PathBuf};
pub use valid_flow_id::valid_flow_id;

pub struct History {
    path: PathBuf,
    resume_path: PathBuf,
    pub snapshot: Snapshot,
    pub blocks: Vec<Block>,
    resume_calls: Vec<ResumeCall>,
    _lock: File,
}

pub use new_flow::new_flow;
pub use validate_flow::validate_flow;
