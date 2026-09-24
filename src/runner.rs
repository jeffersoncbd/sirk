//! Sequential conversations reconstructed from an editable transcript.
use crate::{
    adapters,
    agents::Agent,
    harness::RunRequest,
    history::{Block, History, Snapshot},
    input::{TerminalInput, UserInput},
    services::{BashService, Invocation},
    workflow::{Workflow, render_input},
};
use std::{collections::BTreeMap, path::Path};

fn execute(invocation: &Invocation) -> Result<String, String> {
    let result = BashService::default()
        .execute_streaming(invocation)
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err(format!(
            "{} exited with {}; incomplete output was not committed",
            invocation.program, result.status
        ));
    }
    Ok(result.stdout)
}
pub fn run(workflow: &Workflow, directory: &Path) -> Result<BTreeMap<String, String>, String> {
    run_interactive_with(workflow, directory, execute, &mut TerminalInput)
}
pub fn resume(path: &Path) -> Result<BTreeMap<String, String>, String> {
    continue_with(&mut History::open(path)?, execute, &mut TerminalInput)
}
pub fn run_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
) -> Result<BTreeMap<String, String>, String> {
    struct NoInput;
    impl UserInput for NoInput {
        fn ask(&mut self, _: &str) -> Result<String, String> {
            Err("this execution requires user input".into())
        }
    }
    run_interactive_with(workflow, directory, execute, &mut NoInput)
}
pub fn run_interactive_with(
    workflow: &Workflow,
    directory: &Path,
    execute: impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
) -> Result<BTreeMap<String, String>, String> {
    workflow.validate()?;
    let directory = directory.canonicalize().map_err(|e| e.to_string())?;
    let mut agents = BTreeMap::new();
    for step in &workflow.steps {
        if !agents.contains_key(&step.agent) {
            agents.insert(
                step.agent.clone(),
                Agent::load(&directory.join(".agents"), &step.agent)?,
            );
        }
    }
    let snapshot = Snapshot {
        directory,
        workflow: workflow.clone(),
        agents: agents.into_values().collect(),
    };
    validate_snapshot(&snapshot)?;
    continue_with(&mut History::create(snapshot)?, execute, input)
}
fn validate_snapshot(snapshot: &Snapshot) -> Result<(), String> {
    snapshot.workflow.validate()?;
    if !snapshot.directory.is_dir() {
        return Err("execution directory no longer exists".into());
    }
    for step in &snapshot.workflow.steps {
        let agent = snapshot
            .agents
            .iter()
            .find(|a| a.id == step.agent)
            .ok_or("missing agent configuration")?;
        adapters::resolve(&agent.adapter)
            .ok_or_else(|| format!("unknown adapter `{}`", agent.adapter))?;
        if agent.json {
            return Err("JSON event streams cannot be used with resumable conversations".into());
        }
        if agent.ask.as_ref().is_some_and(|s| s.trim().is_empty()) {
            return Err("`ask` cannot be empty".into());
        }
    }
    Ok(())
}
fn question(text: &str) -> Option<&str> {
    text.trim_start().strip_prefix("ASK:").map(str::trim)
}
fn validate_blocks(history: &History) -> Result<(), String> {
    for (index, blocks) in history.steps.iter().enumerate() {
        let mut expects_input = true;
        let mut complete = false;
        for (position, block) in blocks.iter().enumerate() {
            let valid = match block {
                Block::Ask(text) => position == 0 && !text.trim().is_empty(),
                Block::Input(_) => {
                    let valid = expects_input && !complete;
                    expects_input = false;
                    valid
                }
                Block::Output(text) => {
                    let valid = !expects_input && !complete && !text.trim().is_empty();
                    expects_input = true;
                    complete = question(text).is_none();
                    valid
                }
            };
            if !valid {
                return Err(format!(
                    "invalid conversation in step {}; remove the edited response and everything after it",
                    index + 1
                ));
            }
        }
        if index + 1 < history.steps.len() && !complete {
            return Err(format!(
                "step {} is pending but later steps exist; remove the later steps before resuming",
                index + 1
            ));
        }
    }
    Ok(())
}
pub fn continue_with(
    history: &mut History,
    mut execute: impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
) -> Result<BTreeMap<String, String>, String> {
    validate_snapshot(&history.snapshot)?;
    validate_blocks(history)?;
    eprintln!("History: {}", history.path.display());
    let mut outputs = BTreeMap::new();
    for index in 0..history.snapshot.workflow.steps.len() {
        let step = history.snapshot.workflow.steps[index].clone();
        let agent = history
            .snapshot
            .agents
            .iter()
            .find(|a| a.id == step.agent)
            .unwrap()
            .clone();
        let initial = render_input(&step.input, &outputs)?;
        if index == history.steps.len() {
            history.steps.push(Vec::new());
        }
        if history.steps[index].is_empty() {
            history.steps[index].push(match &agent.ask {
                Some(ask) => Block::Ask(ask.clone()),
                None => Block::Input(initial.clone()),
            });
            history.save()?;
        }
        loop {
            let last = history.steps[index].last().unwrap().clone();
            match last {
                Block::Ask(ref text) => {
                    let answer = input.ask(text)?;
                    history.steps[index].push(Block::Input(answer));
                    history.save()?;
                }
                Block::Output(ref text) => {
                    if let Some(ask) = question(text) {
                        if ask.is_empty() {
                            return Err(
                                "agent returned an empty ASK question; remove that output to retry"
                                    .into(),
                            );
                        }
                        let answer = input.ask(ask)?;
                        history.steps[index].push(Block::Input(answer));
                        history.save()?;
                    } else {
                        if let Some(name) = &step.output {
                            outputs.insert(name.clone(), text.clone());
                        }
                        break;
                    }
                }
                Block::Input(_) => {
                    let mut prompt = agent.instructions.clone();
                    if agent.ask.is_some() && !initial.is_empty() {
                        prompt.push_str(&format!("\n\nWorkflow input:\n{initial}"));
                    }
                    prompt.push_str("\n\nConversation (continue from the last user message):\n");
                    for block in &history.steps[index] {
                        let role = match block {
                            Block::Ask(_) => "Initial question",
                            Block::Input(_) => "User",
                            Block::Output(_) => "Assistant",
                        };
                        prompt.push_str(&format!("\n{role}:\n{}\n", block.text()));
                    }
                    let adapter = adapters::resolve(&agent.adapter).unwrap();
                    let invocation = adapter
                        .invocation(&RunRequest {
                            prompt,
                            working_directory: history.snapshot.directory.clone(),
                            model: agent.model.clone(),
                            event_stream: false,
                        })
                        .map_err(|e| e.to_string())?;
                    let response = execute(&invocation)?;
                    if response.trim().is_empty() {
                        return Err(
                            "agent returned an empty response; input remains pending".into()
                        );
                    }
                    history.steps[index].push(Block::Output(response));
                    history.save()?;
                }
            }
        }
    }
    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::VecDeque,
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };
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
    #[test]
    fn resumes_failed_turn_with_context_and_replays_edited_output() {
        let project = Project::new();
        let mut calls = 0;
        let result = run_interactive_with(
            &workflow(),
            &project.0,
            |invocation| {
                calls += 1;
                let prompt = invocation.arguments.last().unwrap();
                assert!(prompt.contains("Initial context"));
                if calls == 1 {
                    Ok("ASK: Which business?".into())
                } else {
                    Err("process failed".into())
                }
            },
            &mut answers(&["Scheduling", "Dentist"]),
        );
        assert!(result.is_err());
        let path = project.log();
        let mut history = History::open(&path).unwrap();
        assert_eq!(
            history.steps[0].last(),
            Some(&Block::Input("Dentist".into()))
        );
        // Resumption uses the saved configuration even if source files disappear.
        fs::remove_file(project.0.join(".agents/planner.md")).unwrap();
        let mut calls = 0;
        let outputs = continue_with(
            &mut history,
            |invocation| {
                calls += 1;
                let prompt = invocation.arguments.last().unwrap();
                if calls == 1 {
                    for text in ["Make a plan", "Scheduling", "Which business?", "Dentist"] {
                        assert!(prompt.contains(text));
                    }
                    Ok("Plan v1".into())
                } else {
                    assert!(prompt.contains("Plan v1"));
                    Ok("Review v1".into())
                }
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(outputs["review"], "Review v1");
        drop(history);
        let source = fs::read_to_string(&path).unwrap();
        let cut = source.find("<== OUTPUT\nPlan v1").unwrap();
        fs::write(&path, &source[..cut]).unwrap();
        let mut history = History::open(&path).unwrap();
        let mut calls = 0;
        let outputs = continue_with(
            &mut history,
            |invocation| {
                calls += 1;
                if calls == 1 {
                    assert!(!invocation.arguments.last().unwrap().contains("Plan v1"));
                    Ok("Plan v2".into())
                } else {
                    assert!(invocation.arguments.last().unwrap().contains("Plan v2"));
                    Ok("Review v2".into())
                }
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(outputs["review"], "Review v2");
        drop(history);
        let mut history = History::open(&path).unwrap();
        let outputs = continue_with(
            &mut history,
            |_| panic!("completed execution must not run again"),
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(outputs["plan"], "Plan v2");
    }
    #[test]
    fn resumes_questions_without_repeating_model_calls() {
        let project = Project::new();
        assert!(
            run_interactive_with(
                &workflow(),
                &project.0,
                |_| panic!("must await user"),
                &mut answers(&[])
            )
            .is_err()
        );
        let mut history = History::open(&project.log()).unwrap();
        assert!(
            continue_with(
                &mut history,
                |_| Ok("ASK: Details?".into()),
                &mut answers(&["Idea"])
            )
            .is_err()
        );
        drop(history);
        let mut history = History::open(&project.log()).unwrap();
        let mut calls = 0;
        continue_with(
            &mut history,
            |_| {
                calls += 1;
                Ok("Done".into())
            },
            &mut answers(&["Details"]),
        )
        .unwrap();
        assert_eq!(calls, 2);
    }
    #[test]
    fn rejects_downstream_steps_after_deleted_response() {
        let project = Project::new();
        run_interactive_with(
            &workflow(),
            &project.0,
            |_| Ok("Done".into()),
            &mut answers(&["Idea"]),
        )
        .unwrap();
        let mut history = History::open(&project.log()).unwrap();
        history.steps[0].pop();
        assert!(
            continue_with(
                &mut history,
                |_| panic!("invalid state must not execute"),
                &mut answers(&[])
            )
            .is_err()
        );
    }
    #[test]
    fn runs_without_input_and_keeps_inserted_templates_literal() {
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: first\n- agent: second\n  input: '{{ outputs.first }}'\n  output: final\n").unwrap();
        let mut calls = 0;
        let outputs = run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            if calls == 1 {
                Ok("literal {{ outputs.missing }}\n".into())
            } else {
                assert!(
                    invocation
                        .arguments
                        .last()
                        .unwrap()
                        .contains("literal {{ outputs.missing }}")
                );
                Ok("Done".into())
            }
        })
        .unwrap();
        assert_eq!(outputs["final"], "Done");
        let mut history = History::open(&project.log()).unwrap();
        let restored = continue_with(
            &mut history,
            |_| panic!("already complete"),
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(outputs, restored);
    }
    #[test]
    fn validates_all_agents_before_input_or_execution() {
        let project = Project::new();
        fs::write(project.0.join(".agents/second.md"), "invalid").unwrap();
        assert!(
            run_interactive_with(
                &workflow(),
                &project.0,
                |_| panic!("invalid agent must not execute"),
                &mut answers(&[])
            )
            .is_err()
        );
        assert!(!project.0.join("history").exists());
    }
}
