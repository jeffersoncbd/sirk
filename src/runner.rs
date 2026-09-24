//! Sequential conversations reconstructed from an editable transcript.
use crate::{
    adapters,
    agents::Agent,
    harness::RunRequest,
    history::{Block, History, Snapshot},
    input::{TerminalInput, UserInput},
    services::{BashService, Invocation},
    workflow::{Step, Workflow, loop_items, loop_target},
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
    for step in all_steps(&workflow.steps) {
        let Some(id) = &step.agent else {
            continue;
        };
        if !agents.contains_key(id) {
            agents.insert(id.clone(), Agent::load(&directory.join(".agents"), id)?);
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
    for step in all_steps(&snapshot.workflow.steps) {
        let Some(id) = &step.agent else {
            continue;
        };
        let agent = snapshot
            .agents
            .iter()
            .find(|a| &a.id == id)
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
fn all_steps(steps: &[Step]) -> Vec<&Step> {
    let mut result = Vec::new();
    for step in steps {
        result.push(step);
        result.extend(all_steps(&step.iter));
    }
    result
}

fn validate_blocks(history: &History) -> Result<(), String> {
    fn visit(
        history: &History,
        steps: &[Step],
        prefix: &str,
        cursor: &mut usize,
    ) -> Result<bool, String> {
        for (position, step) in steps.iter().enumerate() {
            let id = format!("{prefix}{}", position + 1);
            let Some(blocks) = history.steps.get(*cursor) else {
                return Ok(false);
            };
            let label = format!("Step {id} — {}", step.name());
            if history
                .labels
                .get(*cursor)
                .is_some_and(|saved| saved != &label)
            {
                return Err(format!(
                    "unexpected history step; expected {label}; remove later blocks after editing"
                ));
            }
            *cursor += 1;
            if step.tool.as_deref() == Some("LOOP") {
                if blocks.is_empty() {
                    return Ok(false);
                }
                let [Block::Input(input)] = blocks.as_slice() else {
                    return Err("LOOP history must contain only its input array".into());
                };
                for (iteration, _) in loop_items(input)?.iter().enumerate() {
                    if !visit(
                        history,
                        &step.iter,
                        &format!("{id}.{}.", iteration + 1),
                        cursor,
                    )? {
                        return Ok(false);
                    }
                }
            } else if !validate_step_blocks(blocks, step.is_tool_step())? {
                return Ok(false);
            }
        }
        Ok(true)
    }
    let mut cursor = 0;
    visit(history, &history.snapshot.workflow.steps, "", &mut cursor)?;
    if cursor != history.steps.len() {
        return Err(
            "pending step has later history; remove the edited result and everything after it"
                .into(),
        );
    }
    Ok(())
}

fn validate_step_blocks(blocks: &[Block], tool_step: bool) -> Result<bool, String> {
    let mut expects_input = true;
    let mut complete = false;
    for (position, block) in blocks.iter().enumerate() {
        let valid = match block {
            Block::Ask(text) => !tool_step && position == 0 && !text.trim().is_empty(),
            Block::Input(_) => {
                let pending_tool = position > 0
                    && matches!(&blocks[position - 1], Block::Output(text) if crate::tools::request(text).is_some());
                let valid = expects_input && !complete && (!pending_tool || tool_step);
                expects_input = false;
                valid
            }
            Block::Output(text) => {
                let valid = !expects_input && !complete && (tool_step || !text.trim().is_empty());
                expects_input = true;
                complete = tool_step
                    || (question(text).is_none() && crate::tools::request(text).is_none());
                valid
            }
            Block::Tree(_) | Block::Read(_) => {
                let name = if matches!(block, Block::Tree(_)) {
                    "TREE"
                } else {
                    "READ"
                };
                let valid = !tool_step
                    && position > 0
                    && matches!(&blocks[position - 1], Block::Output(request) if crate::tools::request(request).is_some_and(|(tool, _)| tool == name));
                expects_input = false;
                valid
            }
        };
        if !valid {
            return Err(
                "invalid conversation; remove the edited response and everything after it".into(),
            );
        }
    }
    Ok(complete)
}
pub fn continue_with(
    history: &mut History,
    mut execute: impl FnMut(&Invocation) -> Result<String, String>,
    input: &mut impl UserInput,
) -> Result<BTreeMap<String, String>, String> {
    validate_snapshot(&history.snapshot)?;
    validate_blocks(history)?;
    let mut outputs = BTreeMap::new();
    let steps = history.snapshot.workflow.steps.clone();
    Engine {
        history,
        execute: &mut execute,
        input,
        cursor: 0,
    }
    .run_steps(&steps, "", &mut outputs, &mut None)?;
    Ok(outputs)
}

struct Engine<'a, E, I> {
    history: &'a mut History,
    execute: &'a mut E,
    input: &'a mut I,
    cursor: usize,
}

impl<E, I> Engine<'_, E, I>
where
    E: FnMut(&Invocation) -> Result<String, String>,
    I: UserInput,
{
    fn run_steps(
        &mut self,
        steps: &[Step],
        prefix: &str,
        outputs: &mut BTreeMap<String, String>,
        locals: &mut Option<BTreeMap<String, String>>,
    ) -> Result<(), String> {
        for (position, step) in steps.iter().enumerate() {
            let id = format!("{prefix}{}", position + 1);
            let index = self.cursor;
            self.cursor += 1;
            if index == self.history.steps.len() {
                self.history.steps.push(Vec::new());
                self.history
                    .labels
                    .push(format!("Step {id} — {}", step.name()));
            }
            if step.tool.as_deref() == Some("LOOP") {
                if self.history.steps[index].is_empty() {
                    let argument = step.render_input(outputs, locals.as_ref())?;
                    loop_items(&argument)?;
                    self.history.steps[index].push(Block::Input(argument));
                    self.history.save()?;
                }
                let items = loop_items(self.history.steps[index][0].text())?;
                for (iteration, item) in items.into_iter().enumerate() {
                    let mut child_outputs = outputs.clone();
                    let mut child_locals = Some(BTreeMap::from([("item".into(), item)]));
                    self.run_steps(
                        &step.iter,
                        &format!("{id}.{}.", iteration + 1),
                        &mut child_outputs,
                        &mut child_locals,
                    )?;
                }
                continue;
            }
            let result = self.run_step(step, index, outputs, locals.as_ref())?;
            if let Some(name) = &step.output {
                if let Some(key) = loop_target(name) {
                    locals
                        .as_mut()
                        .ok_or("missing loop scope")?
                        .insert(key.into(), result);
                } else if let Some(values) = locals.as_mut() {
                    values.insert(name.clone(), result);
                } else {
                    outputs.insert(name.clone(), result);
                }
            }
        }
        Ok(())
    }

    fn run_step(
        &mut self,
        step: &Step,
        index: usize,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        let history = &mut self.history;
        if step.is_tool_step() {
            if history.steps[index].is_empty() {
                history.steps[index].push(Block::Input(step.render_input(outputs, locals)?));
                history.save()?;
            }
            if let Some(Block::Input(argument)) = history.steps[index].last() {
                let result = if let Some(tool) = &step.tool {
                    if tool == "WRITE" {
                        let path = step.render_path(outputs, locals)?;
                        crate::tools::write::write(
                            &history.snapshot.directory,
                            &path,
                            argument,
                            step.force(),
                        )?;
                        String::new()
                    } else {
                        crate::tools::execute_with_input(
                            tool,
                            argument,
                            &history.snapshot.directory,
                        )?
                    }
                } else {
                    let tool = step.custom_tool.as_deref().unwrap();
                    let arguments = crate::tools::custom::arguments(argument)?;
                    crate::tools::custom::execute(tool, &arguments, &history.snapshot.directory)?
                };
                print!("{result}");
                history.steps[index].push(Block::Output(result));
                history.save()?;
            }
            return Ok(history.steps[index].last().unwrap().text().to_owned());
        }
        let agent = history
            .snapshot
            .agents
            .iter()
            .find(|a| Some(&a.id) == step.agent.as_ref())
            .unwrap()
            .clone();
        let initial = step.render_input(outputs, locals)?;
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
                    let answer = self.input.ask(text)?;
                    history.steps[index].push(Block::Input(answer));
                    history.save()?;
                }
                Block::Output(ref text) => {
                    if let Some((tool, argument)) = crate::tools::request(text) {
                        let result = crate::tools::execute_with_input(
                            tool,
                            argument,
                            &history.snapshot.directory,
                        )?;
                        print!("{result}");
                        history.steps[index].push(if tool == "TREE" {
                            Block::Tree(result)
                        } else {
                            Block::Read(result)
                        });
                        history.save()?;
                    } else if let Some(ask) = question(text) {
                        if ask.is_empty() {
                            return Err(
                                "agent returned an empty ASK question; remove that output to retry"
                                    .into(),
                            );
                        }
                        let answer = self.input.ask(ask)?;
                        history.steps[index].push(Block::Input(answer));
                        history.save()?;
                    } else {
                        return Ok(text.clone());
                    }
                }
                Block::Input(_) | Block::Tree(_) | Block::Read(_) => {
                    let mut prompt = agent.instructions.clone();
                    prompt.push_str("\n\nAvailable tool: TREE. To list project files respecting Git ignores, respond with exactly TREE and nothing else. The tool result will be returned so you can continue your response.");
                    prompt.push_str("\nAvailable tool: READ. To read a UTF-8 file inside the execution directory, respond with exactly READ: <path> on one line, without quotes or code fences. Paths are relative to the execution directory. The file content will be returned so you can continue your response.");
                    if agent.ask.is_some() && !initial.is_empty() {
                        prompt.push_str(&format!("\n\nWorkflow input:\n{initial}"));
                    }
                    prompt.push_str("\n\nConversation (continue from the last user message):\n");
                    for block in &history.steps[index] {
                        let role = match block {
                            Block::Ask(_) => "Initial question",
                            Block::Input(_) => "User",
                            Block::Output(_) => "Assistant",
                            Block::Tree(_) => "Tool result (TREE)",
                            Block::Read(_) => "Tool result (READ)",
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
                    let response = (self.execute)(&invocation)?;
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
        fn init_git(&self) {
            let result = BashService::default()
                .execute_to(
                    &Invocation {
                        program: "git".into(),
                        arguments: vec!["init".into(), "--quiet".into()],
                        working_directory: self.0.clone(),
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
    fn tool_step_passes_named_output_and_resumes_without_rerunning_tree() {
        let project = Project::new();
        project.init_git();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: TREE\n  output: tree\n- agent: second\n  input: '{{ outputs.tree }}'\n  output: final\n").unwrap();
        assert!(
            run_with(&workflow, &project.0, |invocation| {
                let prompt = invocation.arguments.last().unwrap();
                assert!(prompt.contains("\"visible.txt\""));
                assert!(!prompt.contains("\"ignored.txt\""));
                Err("interrupted agent".into())
            })
            .is_err()
        );
        fs::remove_dir_all(project.0.join(".git")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        let outputs =
            continue_with(&mut history, |_| Ok("Done".into()), &mut answers(&[])).unwrap();
        assert!(outputs["tree"].contains("visible.txt"));
        assert_eq!(outputs["final"], "Done");
    }
    #[test]
    fn agent_tree_request_and_result_survive_interruptions() {
        let project = Project::new();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();
        // The request is saved even if TREE fails (no repository yet).
        assert!(run_with(&workflow, &project.0, |_| Ok("TREE\n".into())).is_err());
        project.init_git();
        let mut history = History::open(&project.log()).unwrap();
        assert!(
            continue_with(
                &mut history,
                |invocation| {
                    let prompt = invocation.arguments.last().unwrap();
                    assert!(prompt.contains("Tool result (TREE)"));
                    assert!(prompt.contains("\"visible.txt\""));
                    Err("interrupted after tool result".into())
                },
                &mut answers(&[])
            )
            .is_err()
        );
        assert!(matches!(history.steps[0].last(), Some(Block::Tree(_))));
        drop(history);
        fs::remove_dir_all(project.0.join(".git")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        let outputs = continue_with(
            &mut history,
            |_| Ok("ASK: Continue?".into()),
            &mut answers(&[]),
        );
        assert!(outputs.is_err());
        let outputs = continue_with(
            &mut history,
            |_| Ok("Final plan".into()),
            &mut answers(&["Yes"]),
        )
        .unwrap();
        assert_eq!(outputs["final"], "Final plan");
    }
    #[test]
    fn tree_only_workflow_needs_no_agents_and_embedded_tree_is_plain_text() {
        let project = Project::new();
        project.init_git();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- tool: TREE\n  output: tree\n").unwrap();
        fs::remove_dir_all(project.0.join(".agents")).unwrap();
        let outputs = run_with(&workflow, &project.0, |_| {
            panic!("tools do not invoke harnesses")
        })
        .unwrap();
        assert!(outputs["tree"].contains("visible.txt"));
        let project = Project::new();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();
        for response in ["Use TREE please", "```\nTREE\n```", "TREE: details", "tree"] {
            let outputs = run_with(&workflow, &project.0, |_| Ok(response.into())).unwrap();
            assert_eq!(outputs["final"], response);
        }
    }
    #[test]
    fn read_step_passes_verbatim_contents_and_restores_saved_result() {
        let project = Project::new();
        let content = "first\r\n==> READ\n<== OUTPUT\nlast\n\n";
        fs::write(project.0.join("a file.txt"), content).unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: READ\n  input: a file.txt\n  output: file\n- agent: second\n  input: '{{ outputs.file }}'\n  output: final\n").unwrap();
        let outputs = run_with(&workflow, &project.0, |invocation| {
            assert!(invocation.arguments.last().unwrap().contains(content));
            Ok("Done".into())
        })
        .unwrap();
        assert_eq!(outputs["file"], content);
        fs::remove_file(project.0.join("a file.txt")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        assert_eq!(
            outputs,
            continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap()
        );
    }
    #[test]
    fn read_request_recovers_failure_and_reuses_empty_file_result() {
        let project = Project::new();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();
        assert!(run_with(&workflow, &project.0, |_| Ok("READ: missing.txt".into())).is_err());
        fs::write(project.0.join("missing.txt"), "").unwrap();
        let mut history = History::open(&project.log()).unwrap();
        assert!(
            continue_with(
                &mut history,
                |invocation| {
                    assert!(
                        invocation
                            .arguments
                            .last()
                            .unwrap()
                            .contains("Tool result (READ)")
                    );
                    Err("interrupted".into())
                },
                &mut answers(&[])
            )
            .is_err()
        );
        drop(history);
        fs::remove_file(project.0.join("missing.txt")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        assert!(matches!(history.steps[0].last(), Some(Block::Read(text)) if text.is_empty()));
        assert_eq!(
            continue_with(&mut history, |_| Ok("Done".into()), &mut answers(&[])).unwrap()["final"],
            "Done"
        );
    }
    #[test]
    fn read_rejects_directories_binary_files_and_outside_symlinks() {
        let project = Project::new();
        assert!(crate::tools::read::read(&project.0, "").is_err());
        assert!(crate::tools::read::read(&project.0, ".agents").is_err());
        fs::write(project.0.join("binary"), [0xff]).unwrap();
        assert!(crate::tools::read::read(&project.0, "binary").is_err());
        let outside = Project::new();
        let path = outside.0.join(".agents/second.md");
        assert!(crate::tools::read::read(&project.0, path.to_str().unwrap()).is_err());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(path, project.0.join("external")).unwrap();
            assert!(crate::tools::read::read(&project.0, "external").is_err());
        }
    }
    #[test]
    fn custom_tool_passes_list_items_as_arguments_and_restores_its_output() {
        let project = Project::new();
        fs::create_dir(project.0.join("tools")).unwrap();
        fs::write(
            project.0.join("tools/combine.sh"),
            "printf '%s|%s|%s' \"$1\" \"$2\" \"$PWD\"\n",
        )
        .unwrap();
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- custom-tool: combine\n  input:\n  - spaces and 'quotes'\n  - '$(exit 19); `exit 20`'\n  output: combined\n- agent: second\n  input: '{{ outputs.combined }}'\n  output: final\n",
        )
        .unwrap();
        let outputs = run_with(&workflow, &project.0, |invocation| {
            let prompt = invocation.arguments.last().unwrap();
            assert!(prompt.contains("spaces and 'quotes'|$(exit 19); `exit 20`|"));
            assert!(prompt.contains(project.0.to_str().unwrap()));
            Ok("Done".into())
        })
        .unwrap();
        assert_eq!(
            outputs["combined"],
            format!(
                "spaces and 'quotes'|$(exit 19); `exit 20`|{}",
                project.0.display()
            )
        );
        fs::remove_file(project.0.join("tools/combine.sh")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        assert_eq!(
            outputs,
            continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap()
        );
    }
    #[test]
    fn write_step_creates_a_file_and_is_not_repeated_after_resume() {
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- tool: WRITE\n  path: generated.txt\n  input: generated content\n  output: written\n",
        )
        .unwrap();
        let outputs = run_with(&workflow, &project.0, |_| {
            panic!("WRITE does not invoke harnesses")
        })
        .unwrap();
        assert_eq!(outputs["written"], "");
        let target = project.0.join("generated.txt");
        assert_eq!(fs::read_to_string(&target).unwrap(), "generated content");
        fs::write(&target, "changed after completion").unwrap();
        let mut history = History::open(&project.log()).unwrap();
        assert_eq!(
            continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap(),
            outputs
        );
        assert_eq!(
            fs::read_to_string(target).unwrap(),
            "changed after completion"
        );
    }
    #[test]
    fn loop_reads_items_with_local_outputs_and_resumes_at_pending_iteration() {
        let project = Project::new();
        fs::write(project.0.join("a.txt"), "A").unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: '[\"a.txt\", \"b.txt\"]'\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: '{{ loop.content }}'\n  - agent: second\n    input: '{{ loop.item }}: {{ loop.content }}'\n    output: iteration_result\n- agent: second\n  input: finished\n  output: final\n").unwrap();
        let mut calls = 0;
        assert!(
            run_with(&workflow, &project.0, |invocation| {
                calls += 1;
                assert!(invocation.arguments.last().unwrap().contains("a.txt: A"));
                Ok("first done".into())
            })
            .is_err()
        );
        assert_eq!(calls, 1);
        let path = project.log();
        let log = fs::read_to_string(&path).unwrap();
        assert!(log.contains("Step 1.2.1 — READ"));
        fs::remove_file(project.0.join("a.txt")).unwrap();
        fs::write(project.0.join("b.txt"), "B").unwrap();
        let mut history = History::open(&path).unwrap();
        let mut calls = 0;
        let outputs = continue_with(
            &mut history,
            |invocation| {
                calls += 1;
                let prompt = invocation.arguments.last().unwrap();
                if calls == 1 {
                    assert!(prompt.contains("b.txt: B"));
                    assert!(!prompt.contains("a.txt: A"));
                } else {
                    assert!(prompt.contains("finished"));
                }
                Ok("done".into())
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(calls, 2);
        assert_eq!(outputs, BTreeMap::from([("final".into(), "done".into())]));
        drop(history);
        // Removing a result and subsequent records regenerates only that suffix.
        let source = fs::read_to_string(&path).unwrap();
        let start = source.find("Step 1.2.2 — second").unwrap();
        let cut = start + source[start..].find("<== OUTPUT").unwrap();
        fs::write(&path, &source[..cut]).unwrap();
        fs::remove_file(project.0.join("b.txt")).unwrap();
        let mut history = History::open(&path).unwrap();
        let mut calls = 0;
        continue_with(
            &mut history,
            |_| {
                calls += 1;
                Ok("replacement".into())
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(calls, 2);
    }
    #[test]
    fn nested_and_empty_loops_restore_outer_scope() {
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: []\n  iter:\n  - agent: second\n- tool: LOOP\n  input: [outer, next]\n  iter:\n  - agent: second\n    input: '{{ loop.item }}'\n    output: '{{ loop.saved }}'\n  - tool: LOOP\n    input: [inner]\n    iter:\n    - agent: second\n      input: '{{ loop.item }}'\n  - agent: second\n    input: '{{ loop.item }} / {{ loop.saved }}'\n").unwrap();
        let mut prompts = Vec::new();
        let outputs = run_with(&workflow, &project.0, |invocation| {
            prompts.push(invocation.arguments.last().unwrap().clone());
            Ok("saved".into())
        })
        .unwrap();
        assert!(outputs.is_empty());
        assert_eq!(prompts.len(), 6);
        for (prompt, value) in prompts.iter().zip([
            "outer",
            "inner",
            "outer / saved",
            "next",
            "inner",
            "next / saved",
        ]) {
            assert!(prompt.contains(&format!("User:\n{value}\n")));
        }
        let mut history = History::open(&project.log()).unwrap();
        continue_with(
            &mut history,
            |_| panic!("completed loops must not rerun"),
            &mut answers(&[]),
        )
        .unwrap();
    }
    #[test]
    fn loop_is_not_an_agent_tool_and_checks_nested_agents_early() {
        assert!(!crate::tools::supports("LOOP"));
        assert!(crate::tools::request("LOOP").is_none());
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- tool: LOOP\n  input: []\n  iter:\n  - agent: missing\n",
        )
        .unwrap();
        assert!(run_with(&workflow, &project.0, |_| panic!("must validate first")).is_err());
        assert!(!project.0.join("history").exists());
    }
    #[test]
    fn loop_rejects_non_array_output_before_running_body() {
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: items\n- tool: LOOP\n  input: '{{ outputs.items }}'\n  iter:\n  - agent: second\n    input: '{{ loop.item }}'\n").unwrap();
        let mut calls = 0;
        let error = run_with(&workflow, &project.0, |_| {
            calls += 1;
            Ok("[\"a\", 2]".into())
        })
        .unwrap_err();
        assert!(error.contains("array of strings"));
        assert_eq!(calls, 1);
    }
    #[test]
    fn loop_refuses_later_records_after_an_edited_pending_child() {
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [a, b]\n  iter:\n  - agent: second\n    input: '{{ loop.item }}'\n").unwrap();
        run_with(&workflow, &project.0, |_| Ok("Done".into())).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        history.steps[1].pop();
        assert!(
            continue_with(
                &mut history,
                |_| panic!("inconsistent history must not run"),
                &mut answers(&[])
            )
            .is_err()
        );
    }
    #[test]
    fn plain_loop_outputs_are_local_and_restored_on_resume() {
        let project = Project::new();
        fs::write(project.0.join("a.txt"), "A").unwrap();
        fs::write(project.0.join("b.txt"), "B").unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: content\n- tool: LOOP\n  input: [a.txt, b.txt]\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: content\n  - agent: second\n    input: '{{ outputs.content }} / {{ loop.item }} / {{ loop.content }}'\n    output: explain\n").unwrap();
        let mut calls = 0;
        assert!(
            run_with(&workflow, &project.0, |invocation| {
                calls += 1;
                if calls == 1 {
                    return Ok("outer".into());
                }
                assert!(
                    invocation
                        .arguments
                        .last()
                        .unwrap()
                        .contains("outer / a.txt / A")
                );
                Err("interrupted".into())
            })
            .is_err()
        );
        fs::remove_file(project.0.join("a.txt")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        let mut prompts = Vec::new();
        let outputs = continue_with(
            &mut history,
            |invocation| {
                prompts.push(invocation.arguments.last().unwrap().clone());
                Ok("explanation".into())
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(prompts.len(), 2);
        assert!(prompts[0].contains("outer / a.txt / A"));
        assert!(prompts[1].contains("outer / b.txt / B"));
        assert_eq!(
            outputs,
            BTreeMap::from([("content".into(), "outer".into())])
        );
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
