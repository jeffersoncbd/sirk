//! Editable transcript and durable execution state.
mod create;
mod drop;
mod finish;
mod lock;
mod open;
mod parse;
mod reserved;
mod save;

pub use crate::interfaces::{Block, Snapshot};
use std::{fs::File, path::PathBuf};
const TITLE: &str = "NEW HARNESS — CONVERSATION HISTORY v2\n---\n";
const SEPARATOR: &str = "============================================================";
type ParsedHistory = (Snapshot, Vec<Vec<Block>>, Vec<String>);

pub struct History {
    pub path: PathBuf,
    pub snapshot: Snapshot,
    pub steps: Vec<Vec<Block>>,
    pub labels: Vec<String>,
    _lock: File,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };
    #[test]
    fn round_trip_preserves_markers_and_whitespace() {
        let directory = std::env::temp_dir().join(format!(
            "transcript-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        let snapshot = Snapshot {
            directory: directory.clone(),
            workflow: serde_yaml::from_str("version: 1\nsteps:\n- agent: test\n").unwrap(),
            agents: vec![],
        };
        let mut history = History::create(snapshot).unwrap();
        history.steps.push(vec![
            Block::Input("hello\n==> INPUT\n<== OUTPUT\nStep 2 — fake\n\\literal\n\n".into()),
            Block::Delete(String::new()),
        ]);
        history.save().unwrap();
        let (_, steps, _) = History::parse(&fs::read_to_string(&history.path).unwrap()).unwrap();
        assert_eq!(history.steps, steps);
        assert!(History::open(&history.path).is_err());
        drop(history);
        fs::remove_dir_all(directory).unwrap();
    }
}
