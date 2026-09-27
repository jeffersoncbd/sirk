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
const TITLE: &str = "S.I.R.K. — CONVERSATION HISTORY v2\n---\n";
// Existing v2 transcripts remain resumable after the project rename.
const LEGACY_TITLE: &str = "NEW HARNESS — CONVERSATION HISTORY v2\n---\n";
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
        let source = fs::read_to_string(&history.path).unwrap();
        assert!(source.starts_with(TITLE));
        let (_, steps, _) = History::parse(&source).unwrap();
        assert_eq!(history.steps, steps);
        let legacy = source.replacen(TITLE, LEGACY_TITLE, 1);
        let (snapshot, steps, labels) = History::parse(&legacy).unwrap();
        assert_eq!(snapshot.directory, directory);
        assert_eq!(history.steps, steps);
        assert_eq!(labels, vec!["Step 1 — test"]);
        assert!(History::parse(&source.replacen("v2", "v1", 1)).is_err());
        assert!(History::parse(&legacy.replacen("v2", "v1", 1)).is_err());
        assert!(History::open(&history.path).is_err());
        let path = history.path.clone();
        drop(history);
        fs::write(&path, legacy).unwrap();
        let history = History::open(&path).unwrap();
        assert_eq!(history.steps, steps);
        history.save().unwrap();
        assert!(fs::read_to_string(&path).unwrap().starts_with(TITLE));
        drop(history);
        fs::remove_dir_all(directory).unwrap();
    }
}
