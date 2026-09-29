use crate::agents::Agent;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub directory: PathBuf,
    pub agent: Agent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Ask(String),
    Input(String),
    Output(String),
    Tree(String),
    Read(String),
    Edit(String),
    Write(String),
    Delete(String),
}

mod marker;
mod text;
