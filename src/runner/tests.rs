//! Shared fixtures for runner tests.

use super::{
    DUPLICATE_EDIT_RESULT, EDIT_FAILURE_PREFIX, continue_with, run_interactive_with, run_with,
};
use crate::{
    history::{Block, History, Snapshot},
    input::UserInput,
    services::{BashService, Invocation},
    workflow::Workflow,
};
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

#[path = "tests/control_flow.rs"]
mod control_flow;
#[path = "tests/conversations.rs"]
mod conversations;
#[path = "tests/edits.rs"]
mod edits;
#[path = "tests/files.rs"]
mod files;

struct Project(PathBuf);
impl Project {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "resume-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(path.join(".agents")).unwrap();
        fs::write(path.join(".agents/planner.md"), "---\nadapter: codex\nask: What shall we plan?\n---\nMake a plan. Ask with ASK: when needed.").unwrap();
        fs::write(
            path.join(".agents/second.md"),
            "---\nadapter: codex\n---\nReview the plan.",
        )
        .unwrap();
        Self(path)
    }
    fn log(&self) -> PathBuf {
        fs::read_dir(self.0.join("history"))
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.extension().is_some_and(|e| e == "log"))
            .unwrap()
    }
    fn init_git(&self) {
        let result = BashService::default()
            .execute_to(
                &Invocation {
                    program: "git".into(),
                    arguments: vec!["init".into(), "--quiet".into()],
                    working_directory: self.0.clone(),
                    environment: Default::default(),
                },
                &mut std::io::sink(),
            )
            .unwrap();
        assert!(result.status.success());
        fs::write(self.0.join(".gitignore"), "history/\nignored.txt\n").unwrap();
        fs::write(self.0.join("visible.txt"), "").unwrap();
        fs::write(self.0.join("ignored.txt"), "").unwrap();
    }
}
impl Drop for Project {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
struct Answers(VecDeque<String>);
impl UserInput for Answers {
    fn ask(&mut self, _: &str) -> Result<String, String> {
        self.0.pop_front().ok_or("EOF".into())
    }
}
fn answers(values: &[&str]) -> Answers {
    Answers(values.iter().map(|v| v.to_string()).collect())
}
fn workflow() -> Workflow {
    serde_yaml::from_str("version: 1\nsteps:\n- agent: planner\n  input: Initial context\n  output: plan\n- agent: second\n  input: '{{ outputs.plan }}'\n  output: review\n").unwrap()
}
