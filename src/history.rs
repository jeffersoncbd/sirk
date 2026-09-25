//! Editable transcript and durable execution state.
use crate::{agents::Agent, workflow::Workflow};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
const TITLE: &str = "NEW HARNESS — CONVERSATION HISTORY v2\n---\n";
const SEPARATOR: &str = "============================================================";
type ParsedHistory = (Snapshot, Vec<Vec<Block>>, Vec<String>);

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub directory: PathBuf,
    pub workflow: Workflow,
    pub agents: Vec<Agent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Ask(String),
    Input(String),
    Output(String),
    Tree(String),
    Read(String),
    Edit(String),
}
impl Block {
    pub fn text(&self) -> &str {
        match self {
            Self::Ask(s)
            | Self::Input(s)
            | Self::Output(s)
            | Self::Tree(s)
            | Self::Read(s)
            | Self::Edit(s) => s,
        }
    }
    fn marker(&self) -> &str {
        match self {
            Self::Ask(_) => "==> ASK",
            Self::Input(_) => "==> INPUT",
            Self::Output(_) => "<== OUTPUT",
            Self::Tree(_) => "==> TREE",
            Self::Read(_) => "==> READ",
            Self::Edit(_) => "==> EDIT",
        }
    }
}

pub struct History {
    pub path: PathBuf,
    pub snapshot: Snapshot,
    pub steps: Vec<Vec<Block>>,
    pub labels: Vec<String>,
    _lock: File,
}
impl Drop for History {
    fn drop(&mut self) {
        // Explicitly release even if a concurrently spawned child briefly inherited the fd.
        let _ = self._lock.unlock();
    }
}
fn reserved(line: &str) -> bool {
    matches!(
        line,
        "==> ASK" | "==> INPUT" | "<== OUTPUT" | "==> TREE" | "==> READ" | "==> EDIT" | SEPARATOR
    ) || line.starts_with("Step ")
        || line.starts_with('\\')
}
impl History {
    pub fn create(snapshot: Snapshot) -> Result<Self, String> {
        let directory = snapshot.directory.join("history");
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let path = directory.join(format!("run-{time}-{}.log", std::process::id()));
        let lock = Self::lock(&path)?;
        let history = Self {
            path,
            snapshot,
            steps: Vec::new(),
            labels: Vec::new(),
            _lock: lock,
        };
        history.save()?;
        Ok(history)
    }
    fn lock(path: &Path) -> Result<File, String> {
        let lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path.with_extension("log.lock"))
            .map_err(|e| e.to_string())?;
        lock.try_lock()
            .map_err(|e| format!("history is already in use or cannot be locked: {e}"))?;
        Ok(lock)
    }
    pub fn open(path: &Path) -> Result<Self, String> {
        let path = path.canonicalize().map_err(|e| e.to_string())?;
        let lock = Self::lock(&path)?;
        let source = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let (snapshot, steps, labels) = Self::parse(&source)?;
        Ok(Self {
            path,
            snapshot,
            steps,
            labels,
            _lock: lock,
        })
    }
    fn parse(source: &str) -> Result<ParsedHistory, String> {
        let source = source
            .strip_prefix(TITLE)
            .ok_or("unsupported history format; only v2 transcripts can be resumed")?;
        let (metadata, body) = source
            .split_once("\n---\n")
            .ok_or("missing history configuration boundary")?;
        let snapshot: Snapshot = serde_yaml::from_str(metadata)
            .map_err(|e| format!("invalid history configuration: {e}"))?;
        let mut steps: Vec<Vec<Block>> = Vec::new();
        let mut labels = Vec::new();
        let mut active: Option<(&str, Vec<String>)> = None;
        fn finish(
            active: &mut Option<(&str, Vec<String>)>,
            steps: &mut [Vec<Block>],
        ) -> Result<(), String> {
            if let Some((marker, mut lines)) = active.take() {
                if lines.last().is_some_and(String::is_empty) {
                    lines.pop();
                }
                let value = lines.join("\n");
                let block = match marker {
                    "==> ASK" => Block::Ask(value),
                    "==> INPUT" => Block::Input(value),
                    "==> TREE" => Block::Tree(value),
                    "==> READ" => Block::Read(value),
                    "==> EDIT" => Block::Edit(value),
                    _ => Block::Output(value),
                };
                steps
                    .last_mut()
                    .ok_or("content before a step header")?
                    .push(block);
            }
            Ok(())
        }
        for line in body.split_terminator('\n') {
            if line == SEPARATOR
                || line.starts_with("Step ")
                || matches!(
                    line,
                    "==> ASK" | "==> INPUT" | "<== OUTPUT" | "==> TREE" | "==> READ" | "==> EDIT"
                )
            {
                finish(&mut active, &mut steps)?;
                if line.starts_with("Step ") {
                    labels.push(line.to_owned());
                    steps.push(Vec::new());
                } else if line != SEPARATOR {
                    active = Some((line, Vec::new()));
                }
            } else if let Some((_, lines)) = &mut active {
                lines.push(line.strip_prefix('\\').unwrap_or(line).to_owned());
            } else if !line.is_empty() {
                return Err(format!("unexpected history line: {line}"));
            }
        }
        finish(&mut active, &mut steps)?;
        Ok((snapshot, steps, labels))
    }
    pub fn save(&self) -> Result<(), String> {
        let metadata = serde_yaml::to_string(&self.snapshot).map_err(|e| e.to_string())?;
        let mut source = format!("{TITLE}{metadata}---\n");
        for (index, blocks) in self.steps.iter().enumerate() {
            let label = self.labels.get(index).cloned().unwrap_or_else(|| {
                format!(
                    "Step {} — {}",
                    index + 1,
                    self.snapshot.workflow.steps[index].name()
                )
            });
            source.push_str(&format!("{SEPARATOR}\n{label}\n\n"));
            for block in blocks {
                source.push_str(block.marker());
                source.push('\n');
                for line in block.text().split('\n') {
                    if reserved(line) {
                        source.push('\\');
                    }
                    source.push_str(line);
                    source.push('\n');
                }
                source.push('\n');
            }
        }
        let temporary = self.path.with_extension("log.tmp");
        let mut file = File::create(&temporary).map_err(|e| e.to_string())?;
        file.write_all(source.as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| e.to_string())?;
        fs::rename(&temporary, &self.path).map_err(|e| e.to_string())?;
        File::open(self.path.parent().unwrap())
            .and_then(|f| f.sync_all())
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        history.steps.push(vec![Block::Input(
            "hello\n==> INPUT\n<== OUTPUT\nStep 2 — fake\n\\literal\n\n".into(),
        )]);
        history.save().unwrap();
        let (_, steps, _) = History::parse(&fs::read_to_string(&history.path).unwrap()).unwrap();
        assert_eq!(history.steps, steps);
        assert!(History::open(&history.path).is_err());
        drop(history);
        fs::remove_dir_all(directory).unwrap();
    }
}
