//! Sequential conversations reconstructed from an editable transcript.
use crate::{
    adapters,
    agents::Agent,
    harness::RunRequest,
    history::{Block, History, Snapshot},
    input::{TerminalInput, UserInput},
    services::{BashService, Invocation},
    workflow::{Step, Workflow, condition, loop_items, loop_target},
};
use serde::{Deserialize, Serialize};
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

const DUPLICATE_EDIT_RESULT: &str = "No changes applied: this EDIT request was already completed. Do not repeat it; provide a final response or a different edit.";
const EDIT_FAILURE_PREFIX: &str = "EDIT failed: ";
const DUPLICATE_DELETE_RESULT: &str = "No changes applied: this DELETE request was already completed. Do not repeat it; provide a final response or a different request.";
const DELETE_FAILURE_PREFIX: &str = "DELETE failed: ";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ExternalDeleteRequest {
    path: String,
    #[serde(default)]
    force: bool,
}

fn external_edit_request(text: &str) -> Option<Result<crate::tools::edit::Request, String>> {
    let payload = text.trim().strip_prefix("EDIT:")?.trim();
    Some(
        serde_json::from_str(payload)
            .map_err(|error| format!("invalid EDIT_TOOL request: {error}"))
            .and_then(|request: crate::tools::edit::Request| {
                if request.version.is_some() {
                    return Err("EDIT_TOOL must not include `version`".into());
                }
                Ok(request)
            }),
    )
}

fn prepare_external_edit(
    mut request: crate::tools::edit::Request,
    directory: &Path,
) -> Result<crate::tools::edit::Pending, String> {
    let before = crate::tools::read::read(directory, &request.path)?;
    let line_count = before.split_inclusive('\n').count();
    match request.operation {
        crate::tools::edit::Operation::Insert
            if request.line.is_some_and(|line| line > line_count + 1) =>
        {
            return Err(format!(
                "EDIT insert line is beyond EOF; `{}` has {line_count} lines",
                request.path
            ));
        }
        crate::tools::edit::Operation::Delete | crate::tools::edit::Operation::Replace
            if request.end.is_some_and(|end| end > line_count) =>
        {
            return Err(format!(
                "EDIT range is beyond EOF; `{}` has {line_count} lines",
                request.path
            ));
        }
        _ => (),
    }
    request.version = Some(crate::tools::edit::version(&before));
    let pending = crate::tools::edit::Pending {
        request,
        before: Some(before),
        was_missing: false,
    };
    pending.validate()?;
    Ok(pending)
}

fn external_edit_pending(blocks: &[Block]) -> Option<Result<crate::tools::edit::Pending, String>> {
    let [.., Block::Output(request), Block::Input(pending)] = blocks else {
        return None;
    };
    if let Err(error) = external_edit_request(request)? {
        return Some(Err(error));
    }
    Some(
        serde_json::from_str(pending)
            .map_err(|error| format!("invalid EDIT_TOOL history: {error}")),
    )
}

fn completed_external_edit(blocks: &[Block], request: &crate::tools::edit::Request) -> bool {
    blocks.windows(3).any(|window| {
        let [Block::Output(previous), Block::Input(_), Block::Edit(_)] = window else {
            return false;
        };
        external_edit_request(previous).is_some_and(|previous| previous.as_ref() == Ok(request))
    })
}

fn external_delete_request(text: &str) -> Option<Result<ExternalDeleteRequest, String>> {
    let payload = text.trim().strip_prefix("DELETE:")?.trim();
    Some(
        serde_json::from_str(payload)
            .map_err(|error| format!("invalid DELETE_TOOL request: {error}")),
    )
}

fn external_delete_pending(blocks: &[Block]) -> Option<Result<ExternalDeleteRequest, String>> {
    let [.., Block::Output(request), Block::Input(pending)] = blocks else {
        return None;
    };
    let request = external_delete_request(request)?;
    Some(request.and_then(|request| {
        let pending: ExternalDeleteRequest = serde_json::from_str(pending)
            .map_err(|error| format!("invalid DELETE_TOOL history: {error}"))?;
        (pending == request)
            .then_some(pending)
            .ok_or_else(|| "DELETE_TOOL history does not match its request".into())
    }))
}

fn completed_external_delete(blocks: &[Block], request: &ExternalDeleteRequest) -> bool {
    blocks.windows(3).any(|window| {
        let [Block::Output(previous), Block::Input(_), Block::Delete(_)] = window else {
            return false;
        };
        external_delete_request(previous).is_some_and(|previous| previous.as_ref() == Ok(request))
    })
}
fn all_steps(steps: &[Step]) -> Vec<&Step> {
    let mut result = Vec::new();
    for step in steps {
        result.push(step);
        result.extend(all_steps(&step.iter));
        result.extend(all_steps(&step.is_true));
        result.extend(all_steps(&step.is_false));
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
            } else if step.tool.as_deref() == Some("IF") {
                if blocks.is_empty() {
                    return Ok(false);
                }
                let [Block::Input(input)] = blocks.as_slice() else {
                    return Err("IF history must contain only its condition".into());
                };
                let (branch, body) = step.branch(condition(input)?);
                if !visit(history, body, &format!("{id}.{branch}."), cursor)? {
                    return Ok(false);
                }
            } else if step.tool.as_deref() == Some("EDIT") {
                if !validate_edit_blocks(blocks)? {
                    return Ok(false);
                }
            } else if let Some(id) = &step.agent {
                let agent = history
                    .snapshot
                    .agents
                    .iter()
                    .find(|agent| &agent.id == id)
                    .ok_or("missing agent configuration")?;
                if !validate_agent_blocks(
                    blocks,
                    agent.edit_tool,
                    agent.delete_tool,
                    agent.delete_without_confirm,
                )? {
                    return Ok(false);
                }
            } else if !validate_step_blocks(blocks, true)? {
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

fn validate_edit_blocks(blocks: &[Block]) -> Result<bool, String> {
    if blocks.is_empty() {
        return Ok(false);
    }
    let Block::Input(text) = &blocks[0] else {
        return Err("EDIT history requires prepared input".into());
    };
    let pending: crate::tools::edit::Pending =
        serde_json::from_str(text).map_err(|e| format!("invalid EDIT history: {e}"))?;
    pending.validate()?;
    match &blocks[1..] {
        [] => Ok(false),
        [Block::Output(diff)] if pending.before.is_some() && *diff == pending.diff()? => Ok(true),
        _ => Err("invalid EDIT result; remove the edited result and later records".into()),
    }
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
            Block::Tree(_) | Block::Read(_) | Block::Edit(_) => {
                let name = if matches!(block, Block::Tree(_)) {
                    "TREE"
                } else if matches!(block, Block::Read(_)) {
                    "READ"
                } else {
                    "EDIT"
                };
                let valid = !tool_step
                    && position > 0
                    && matches!(&blocks[position - 1], Block::Output(request) if crate::tools::request(request).is_some_and(|(tool, _)| tool == name));
                expects_input = false;
                valid
            }
            Block::Delete(_) => false,
        };
        if !valid {
            return Err(
                "invalid conversation; remove the edited response and everything after it".into(),
            );
        }
    }
    Ok(complete)
}

fn validate_agent_blocks(
    blocks: &[Block],
    edit_tool: bool,
    delete_tool: bool,
    delete_without_confirm: bool,
) -> Result<bool, String> {
    let mut position = 0;
    match blocks.get(position) {
        Some(Block::Ask(text)) if !text.trim().is_empty() => position += 1,
        Some(Block::Input(_)) => (),
        Some(_) => return Err("invalid conversation; expected initial input".into()),
        None => return Ok(false),
    }
    if matches!(blocks.get(position), Some(Block::Ask(_))) {
        return Err("invalid conversation; ASK can only start an agent turn".into());
    }
    if matches!(blocks.get(position), Some(Block::Input(_))) {
        position += 1;
    } else if position != 0 {
        return Ok(false);
    }
    while position < blocks.len() {
        let Block::Output(output) = &blocks[position] else {
            return Err("invalid conversation; expected agent output".into());
        };
        position += 1;
        if let Some((tool, _)) = crate::tools::request(output) {
            let expected = if tool == "TREE" { "TREE" } else { "READ" };
            let Some(result) = blocks.get(position) else {
                return Ok(false);
            };
            if !matches!(
                (expected, result),
                ("TREE", Block::Tree(_)) | ("READ", Block::Read(_))
            ) {
                return Err("invalid conversation; tool result does not match request".into());
            }
            position += 1;
            continue;
        }
        if let Some(request) = external_edit_request(output) {
            if !edit_tool {
                return Err("agent requested EDIT_TOOL without permission".into());
            }
            let request = request?;
            if matches!(blocks.get(position), Some(Block::Edit(result)) if result == DUPLICATE_EDIT_RESULT)
            {
                if !completed_external_edit(&blocks[..position - 1], &request) {
                    return Err("invalid duplicate EDIT_TOOL result".into());
                }
                position += 1;
                continue;
            }
            if matches!(blocks.get(position), Some(Block::Edit(result)) if result.starts_with(EDIT_FAILURE_PREFIX))
            {
                position += 1;
                continue;
            }
            let Some(Block::Input(pending)) = blocks.get(position) else {
                return Ok(false);
            };
            let pending: crate::tools::edit::Pending = serde_json::from_str(pending)
                .map_err(|error| format!("invalid EDIT_TOOL history: {error}"))?;
            pending.validate()?;
            position += 1;
            let Some(Block::Edit(diff)) = blocks.get(position) else {
                return Ok(false);
            };
            if *diff != pending.diff()? {
                return Err(
                    "invalid EDIT_TOOL result; remove the response and later records".into(),
                );
            }
            position += 1;
            continue;
        }
        if let Some(request) = external_delete_request(output) {
            if !delete_tool {
                return Err("agent requested DELETE_TOOL without permission".into());
            }
            let request = request?;
            if matches!(blocks.get(position), Some(Block::Delete(result)) if result == DUPLICATE_DELETE_RESULT)
            {
                if !completed_external_delete(&blocks[..position - 1], &request) {
                    return Err("invalid duplicate DELETE_TOOL result".into());
                }
                position += 1;
                continue;
            }
            if matches!(blocks.get(position), Some(Block::Delete(result)) if result.starts_with(DELETE_FAILURE_PREFIX))
            {
                position += 1;
                continue;
            }
            if request.force && !delete_without_confirm {
                return Err(
                    "DELETE_TOOL force request lacks DELETE_WITHOUT_CONFIRM permission".into(),
                );
            }
            let Some(Block::Input(pending)) = blocks.get(position) else {
                return Ok(false);
            };
            let pending: ExternalDeleteRequest = serde_json::from_str(pending)
                .map_err(|error| format!("invalid DELETE_TOOL history: {error}"))?;
            if pending != request {
                return Err("DELETE_TOOL history does not match its request".into());
            }
            position += 1;
            if !matches!(blocks.get(position), Some(Block::Delete(_))) {
                return Ok(false);
            }
            position += 1;
            continue;
        }
        if let Some(ask) = question(output) {
            if ask.is_empty() {
                return Err("agent returned an empty ASK question".into());
            }
            if !matches!(blocks.get(position), Some(Block::Input(_))) {
                return Ok(false);
            }
            position += 1;
            continue;
        }
        return Ok(position == blocks.len());
    }
    Ok(false)
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
            if step.tool.as_deref() == Some("IF") {
                if self.history.steps[index].is_empty() {
                    let argument = step.render_input(outputs, locals.as_ref())?;
                    condition(&argument)?;
                    self.history.steps[index].push(Block::Input(argument));
                    self.history.save()?;
                }
                let selected = condition(self.history.steps[index][0].text())?;
                let (branch, body) = step.branch(selected);
                self.run_steps(body, &format!("{id}.{branch}."), outputs, locals)?;
                continue;
            }
            let result = self.run_step(step, index, outputs, locals.as_ref())?;
            let version = crate::tools::edit::version(&result);
            for (name, value) in step
                .output
                .iter()
                .map(|name| (name, &result))
                .chain(step.version_output.iter().map(|name| (name, &version)))
            {
                if let Some(key) = loop_target(name) {
                    locals
                        .as_mut()
                        .ok_or("missing loop scope")?
                        .insert(key.into(), value.clone());
                } else if let Some(values) = locals.as_mut() {
                    values.insert(name.clone(), value.clone());
                } else {
                    outputs.insert(name.clone(), value.clone());
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
        if step.tool.as_deref() == Some("EDIT") {
            if history.steps[index].is_empty() {
                let pending = crate::tools::edit::Pending {
                    request: step.edit_request(outputs, locals)?,
                    before: None,
                    was_missing: false,
                };
                pending.validate()?;
                history.steps[index].push(Block::Input(
                    serde_json::to_string(&pending).map_err(|e| e.to_string())?,
                ));
                history.save()?;
            }
            if let Some(Block::Output(result)) = history.steps[index].last() {
                return Ok(result.clone());
            }
            let mut pending: crate::tools::edit::Pending =
                serde_json::from_str(history.steps[index][0].text()).map_err(|e| e.to_string())?;
            if pending.before.is_none() {
                pending.prepare(&history.snapshot.directory)?;
                history.steps[index][0] =
                    Block::Input(serde_json::to_string(&pending).map_err(|e| e.to_string())?);
                history.save()?;
            }
            let diff = pending.commit(&history.snapshot.directory)?;
            history.steps[index].push(Block::Output(diff.clone()));
            history.save()?;
            crate::tools::edit::display(&diff);
            return Ok(diff);
        }
        if step.tool.as_deref() == Some("AWAIT") {
            if history.steps[index].is_empty() {
                history.steps[index].push(Block::Input(String::new()));
                history.save()?;
            }
            if matches!(history.steps[index].last(), Some(Block::Input(_))) {
                self.input
                    .await_confirmation("Awaiting confirmation. Press Enter to continue...")?;
                history.steps[index].push(Block::Output(String::new()));
                history.save()?;
            }
            return Ok(String::new());
        }
        if step.is_tool_step() {
            if history.steps[index].is_empty() {
                history.steps[index].push(Block::Input(step.render_input(outputs, locals)?));
                history.save()?;
            }
            if let Some(Block::Input(argument)) = history.steps[index].last() {
                let result = if let Some(tool) = &step.tool {
                    if tool == "WRITE" {
                        let path = step.render_path(outputs, locals)?;
                        crate::tools::write::write_with_options(
                            &history.snapshot.directory,
                            &path,
                            argument,
                            step.force(),
                            step.skip.unwrap_or(false),
                        )?;
                        String::new()
                    } else if tool == "DELETE" {
                        let path = step.render_path(outputs, locals)?;
                        if !step.force() {
                            self.input.await_confirmation(&format!(
                                "Delete `{path}`? Press Enter to confirm, or /cancel to cancel."
                            ))?;
                        }
                        crate::tools::delete::delete(&history.snapshot.directory, &path)?;
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
                    if let Some(request) = external_edit_request(text) {
                        if !agent.edit_tool {
                            return Err("agent requested EDIT_TOOL without permission".into());
                        }
                        let request = request?;
                        if completed_external_edit(&history.steps[index], &request) {
                            history.steps[index].push(Block::Edit(DUPLICATE_EDIT_RESULT.into()));
                        } else {
                            match prepare_external_edit(request, &history.snapshot.directory) {
                                Ok(pending) => history.steps[index].push(Block::Input(
                                    serde_json::to_string(&pending)
                                        .map_err(|error| error.to_string())?,
                                )),
                                Err(error) => history.steps[index].push(Block::Edit(format!(
                                    "{EDIT_FAILURE_PREFIX}{error}. Correct the request and try again."
                                ))),
                            }
                        }
                        history.save()?;
                    } else if let Some(request) = external_delete_request(text) {
                        if !agent.delete_tool {
                            return Err("agent requested DELETE_TOOL without permission".into());
                        }
                        let request = request?;
                        if request.force && !agent.delete_without_confirm {
                            history.steps[index].push(Block::Delete(format!(
                                "{DELETE_FAILURE_PREFIX}`force: true` requires DELETE_WITHOUT_CONFIRM: allow. Correct the request and try again."
                            )));
                        } else if completed_external_delete(&history.steps[index], &request) {
                            history.steps[index]
                                .push(Block::Delete(DUPLICATE_DELETE_RESULT.into()));
                        } else {
                            history.steps[index].push(Block::Input(
                                serde_json::to_string(&request)
                                    .map_err(|error| error.to_string())?,
                            ));
                        }
                        history.save()?;
                    } else if let Some((tool, argument)) = crate::tools::request(text) {
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
                Block::Input(_) if external_edit_pending(&history.steps[index]).is_some() => {
                    let pending = external_edit_pending(&history.steps[index])
                        .expect("matched external edit pending")?;
                    let diff = pending.commit(&history.snapshot.directory)?;
                    history.steps[index].push(Block::Edit(diff.clone()));
                    history.save()?;
                    crate::tools::edit::display(&diff);
                }
                Block::Input(_) if external_delete_pending(&history.steps[index]).is_some() => {
                    let request = external_delete_pending(&history.steps[index])
                        .expect("matched external delete pending")?;
                    if !request.force {
                        self.input.await_confirmation(&format!(
                            "Delete `{}`? Press Enter to confirm, or /cancel to cancel.",
                            request.path
                        ))?;
                    }
                    let result = match crate::tools::delete::delete(
                        &history.snapshot.directory,
                        &request.path,
                    ) {
                        Ok(()) => String::new(),
                        Err(error) => format!(
                            "{DELETE_FAILURE_PREFIX}{error}. Correct the request and try again."
                        ),
                    };
                    history.steps[index].push(Block::Delete(result));
                    history.save()?;
                }
                Block::Input(_)
                | Block::Tree(_)
                | Block::Read(_)
                | Block::Edit(_)
                | Block::Delete(_) => {
                    let mut prompt = agent.instructions.clone();
                    prompt.push_str("\n\nAvailable tool: TREE. To list project files respecting Git ignores, respond with exactly TREE and nothing else. The tool result will be returned so you can continue your response.");
                    prompt.push_str("\nAvailable tool: READ. To read a UTF-8 file inside the execution directory, respond with exactly READ: <path> on one line, without quotes or code fences. Paths are relative to the execution directory. The file content will be returned so you can continue your response.");
                    if agent.edit_tool {
                        prompt.push_str("\n\nExternal tool: EDIT. This tool is executed after your response; do not try to use an internal tool. To request exactly one file edit, respond only with `EDIT:` followed by a JSON object, without code fences or any other text. The object must contain `path`, `operation`, and `input`. `operation` is one of `insert`, `delete`, `replace`, `prepend`, or `append`. For `insert`, also provide positive integer `line`. For `delete` and `replace`, also provide positive integer `start` and `end` (inclusive). `delete` requires an empty `input`. `prepend` and `append` take no coordinates. Paths are relative to the execution directory. Do not include `version`; the external tool verifies the current document before applying the edit. After a successful edit, you receive its diff and may request another edit or provide your final answer. A Tool result (EDIT) beginning with `EDIT failed:` means no change was made; correct the request and try again. Before requesting another edit, inspect every Tool result (EDIT) in the conversation. Never repeat a request that was already applied; if the diff completes the work, provide your final answer. Request edits only when necessary.");
                    }
                    if agent.delete_tool {
                        prompt.push_str("\n\nExternal tool: DELETE. To request deletion of exactly one regular file, respond only with `DELETE:` followed by a JSON object containing `path`, without code fences or other text. Paths are relative to the execution directory. The user confirms each deletion before it runs. You may include `force: true` only when DELETE_WITHOUT_CONFIRM is allowed; otherwise it is rejected. After execution, inspect the Tool result (DELETE) before responding.");
                    }
                    if agent.ask.is_some() && !initial.is_empty() {
                        prompt.push_str(&format!("\n\nWorkflow input:\n{initial}"));
                    }
                    prompt.push_str("\n\nConversation (continue from the last user message):\n");
                    for (position, block) in history.steps[index].iter().enumerate() {
                        if matches!(block, Block::Input(_))
                            && position > 0
                            && (external_edit_request(history.steps[index][position - 1].text())
                                .is_some()
                                || external_delete_request(
                                    history.steps[index][position - 1].text(),
                                )
                                .is_some())
                        {
                            continue;
                        }
                        let role = match block {
                            Block::Ask(_) => "Initial question",
                            Block::Input(_) => "User",
                            Block::Output(_) => "Assistant",
                            Block::Tree(_) => "Tool result (TREE)",
                            Block::Read(_) => "Tool result (READ)",
                            Block::Edit(_) => "Tool result (EDIT)",
                            Block::Delete(_) => "Tool result (DELETE)",
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
                    let response = adapter
                        .response((self.execute)(&invocation)?)
                        .map_err(|e| e.to_string())?;
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
    fn agent_edit_tool_applies_one_external_edit_and_resumes_without_repeating_it() {
        let project = Project::new();
        fs::write(
            project.0.join(".agents/second.md"),
            "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
        )
        .unwrap();
        fs::write(project.0.join("document.md"), "before\nobsolete\nafter\n").unwrap();
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
        )
        .unwrap();
        let mut calls = 0;
        assert!(run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            if calls == 1 {
                assert!(prompt.contains("External tool: EDIT"));
                Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":2,\"end\":2,\"input\":\"current\\n\"}".into())
            } else {
                assert!(prompt.contains("Tool result (EDIT)"));
                assert!(prompt.contains("+current"));
                Err("interrupted after edit".into())
            }
        })
        .is_err());
        assert_eq!(
            fs::read_to_string(project.0.join("document.md")).unwrap(),
            "before\ncurrent\nafter\n"
        );
        let mut history = History::open(&project.log()).unwrap();
        let output = continue_with(
            &mut history,
            |invocation| {
                assert!(invocation.arguments.last().unwrap().contains("+current"));
                Ok("Updated only the obsolete line.".into())
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(output["result"], "Updated only the obsolete line.");
        assert_eq!(
            fs::read_to_string(project.0.join("document.md")).unwrap(),
            "before\ncurrent\nafter\n"
        );
    }

    #[test]
    fn agent_edit_tool_ignores_a_repeated_completed_request() {
        let project = Project::new();
        fs::write(
            project.0.join(".agents/second.md"),
            "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
        )
        .unwrap();
        fs::write(project.0.join("document.md"), "before\nobsolete\nafter\n").unwrap();
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
        )
        .unwrap();
        let request = "EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":2,\"end\":2,\"input\":\"current\\n\"}";
        let mut calls = 0;
        let output = run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            match calls {
                1 | 2 => Ok(request.into()),
                3 => {
                    assert!(
                        invocation
                            .arguments
                            .last()
                            .unwrap()
                            .contains(DUPLICATE_EDIT_RESULT)
                    );
                    Ok("Updated only the obsolete line.".into())
                }
                _ => panic!("unexpected agent invocation"),
            }
        })
        .unwrap();
        assert_eq!(output["result"], "Updated only the obsolete line.");
        assert_eq!(
            fs::read_to_string(project.0.join("document.md")).unwrap(),
            "before\ncurrent\nafter\n"
        );
        let mut history = History::open(&project.log()).unwrap();
        continue_with(
            &mut history,
            |_| panic!("completed agent turn must not run again"),
            &mut answers(&[]),
        )
        .unwrap();
    }

    #[test]
    fn agent_edit_tool_returns_invalid_coordinates_for_correction() {
        let project = Project::new();
        fs::write(
            project.0.join(".agents/second.md"),
            "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
        )
        .unwrap();
        fs::write(project.0.join("document.md"), "before\nobsolete\n").unwrap();
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
        )
        .unwrap();
        let mut calls = 0;
        let output = run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            match calls {
                1 => Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":1,\"end\":3,\"input\":\"current\\n\"}".into()),
                2 => {
                    let prompt = invocation.arguments.last().unwrap();
                    assert!(prompt.contains(EDIT_FAILURE_PREFIX));
                    assert!(prompt.contains("range is beyond EOF"));
                    assert!(prompt.contains("`document.md` has 2 lines"));
                    Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":1,\"end\":2,\"input\":\"current\\n\"}".into())
                }
                3 => Ok("Updated the document.".into()),
                _ => panic!("unexpected agent invocation"),
            }
        })
        .unwrap();
        assert_eq!(output["result"], "Updated the document.");
        assert_eq!(
            fs::read_to_string(project.0.join("document.md")).unwrap(),
            "current\n"
        );
        let mut history = History::open(&project.log()).unwrap();
        continue_with(
            &mut history,
            |_| panic!("completed agent turn must not run again"),
            &mut answers(&[]),
        )
        .unwrap();
    }

    #[test]
    fn await_pauses_once_and_is_complete_after_resume() {
        let project = Project::new();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- tool: AWAIT\n  output: confirmed\n")
                .unwrap();
        let output = run_interactive_with(
            &workflow,
            &project.0,
            |_| panic!("AWAIT does not invoke an agent"),
            &mut answers(&[""]),
        )
        .unwrap();
        assert_eq!(output["confirmed"], "");
        let mut history = History::open(&project.log()).unwrap();
        let output = continue_with(
            &mut history,
            |_| panic!("completed AWAIT does not invoke an agent"),
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(output["confirmed"], "");
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
    fn read_version_and_edit_work_in_loop_scopes_and_resume() {
        let project = Project::new();
        fs::create_dir(project.0.join("tools")).unwrap();
        fs::write(project.0.join("tools/line.sh"), "printf ' 4\\n'\n").unwrap();
        for name in ["a", "b"] {
            fs::write(project.0.join(name), "fn a() {\n}\nfn b() {\n}\n").unwrap();
        }
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [a, b]\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: source\n    version-output: revision\n  - custom-tool: line\n    input: []\n    output: target\n  - tool: EDIT\n    path: '{{ loop.item }}'\n    operation: replace\n    start: '{{ loop.target }}'\n    end: '{{ loop.target }}'\n    version: '{{ loop.revision }}'\n    input: \"  // literal {{ loop.source }}\\n}\\n\"\n    output: diff\n").unwrap();
        // Inserted source contains no templates; repeated braces select only line 4.
        let result = run_with(&workflow, &project.0, |_| panic!("no model")).unwrap();
        assert!(result.is_empty());
        for name in ["a", "b"] {
            let content = fs::read_to_string(project.0.join(name)).unwrap();
            assert!(content.starts_with("fn a() {\n}\nfn b() {\n  // literal"));
        }
        fs::remove_file(project.0.join("tools/line.sh")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        fs::remove_file(project.0.join("a")).unwrap();
        assert!(
            continue_with(&mut history, |_| panic!("completed"), &mut answers(&[]))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn edit_rejects_stale_read_version_and_preserves_file() {
        let project = Project::new();
        fs::write(project.0.join("file"), "original\n").unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: READ\n  input: file\n  version-output: revision\n- agent: second\n- tool: EDIT\n  path: file\n  operation: delete\n  start: 1\n  end: 1\n  version: '{{ outputs.revision }}'\n").unwrap();
        let error = run_with(&workflow, &project.0, |_| {
            fs::write(project.0.join("file"), "external change\n").unwrap();
            Ok("Done".into())
        })
        .unwrap_err();
        assert!(error.contains("version conflict"));
        assert_eq!(
            fs::read_to_string(project.0.join("file")).unwrap(),
            "external change\n"
        );
    }

    #[test]
    fn prepared_edits_recover_before_and_after_commit_without_duplication() {
        use crate::tools::edit::{Operation, Pending, Request};
        for operation in [Operation::Append, Operation::Prepend] {
            for already_written in [false, true] {
                let project = Project::new();
                fs::write(project.0.join("file"), "original\n").unwrap();
                let op = if operation == Operation::Append {
                    "append"
                } else {
                    "prepend"
                };
                let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: file\n  operation: {op}\n  input: \"entry\\n\"\n  output: diff\n")).unwrap();
                let mut pending = Pending {
                    request: Request {
                        path: "file".into(),
                        operation,
                        line: None,
                        start: None,
                        end: None,
                        version: None,
                        input: "entry\n".into(),
                    },
                    before: None,
                    was_missing: false,
                };
                pending.prepare(&project.0).unwrap();
                let expected = pending.request.apply_to("original\n").unwrap();
                let mut history = History::create(Snapshot {
                    directory: project.0.clone(),
                    workflow,
                    agents: vec![],
                })
                .unwrap();
                history.labels.push("Step 1 — EDIT".into());
                history
                    .steps
                    .push(vec![Block::Input(serde_json::to_string(&pending).unwrap())]);
                history.save().unwrap();
                if already_written {
                    pending.commit(&project.0).unwrap();
                }
                let path = history.path.clone();
                drop(history);
                let mut history = History::open(&path).unwrap();
                let outputs =
                    continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap();
                assert_eq!(outputs["diff"], pending.diff().unwrap());
                assert_eq!(
                    fs::read_to_string(project.0.join("file")).unwrap(),
                    expected
                );
                assert_eq!(
                    continue_with(&mut history, |_| panic!("completed"), &mut answers(&[]))
                        .unwrap(),
                    outputs
                );
                assert_eq!(
                    fs::read_to_string(project.0.join("file")).unwrap(),
                    expected
                );
                // A pending operation followed by later history is rejected before applying.
                history.steps[0].pop();
                history.steps.push(vec![Block::Input("later".into())]);
                assert!(
                    continue_with(&mut history, |_| panic!("invalid"), &mut answers(&[])).is_err()
                );
                history.steps.pop();
                fs::write(project.0.join("file"), "unrelated").unwrap();
                assert!(
                    continue_with(&mut history, |_| panic!("conflict"), &mut answers(&[]))
                        .unwrap_err()
                        .contains("conflict")
                );
                assert_eq!(
                    fs::read_to_string(project.0.join("file")).unwrap(),
                    "unrelated"
                );
                history.steps[0].push(Block::Output("forged diff".into()));
                assert!(
                    continue_with(&mut history, |_| panic!("invalid"), &mut answers(&[]))
                        .unwrap_err()
                        .contains("invalid EDIT result")
                );
            }
        }
    }

    #[test]
    fn edit_confines_paths_and_preserves_permissions() {
        use crate::tools::edit::{Operation, Pending, Request};
        let project = Project::new();
        let outside = Project::new();
        fs::write(project.0.join("file"), "before").unwrap();
        fs::write(project.0.join("binary"), [255]).unwrap();
        fs::write(outside.0.join("file"), "outside").unwrap();
        let mut pending = Pending {
            request: Request {
                path: "file".into(),
                operation: Operation::Append,
                line: None,
                start: None,
                end: None,
                version: None,
                input: "after".into(),
            },
            before: None,
            was_missing: false,
        };
        for path in [
            ".agents".into(),
            "binary".into(),
            outside.0.join("file").to_str().unwrap().into(),
        ] {
            pending.request.path = path;
            assert!(pending.prepare(&project.0).is_err());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::{PermissionsExt, symlink};
            symlink(project.0.join("file"), project.0.join("link")).unwrap();
            symlink(&outside.0, project.0.join("external")).unwrap();
            for path in ["link", "external/file"] {
                pending.request.path = path.into();
                assert!(pending.prepare(&project.0).is_err());
            }
            fs::set_permissions(project.0.join("file"), fs::Permissions::from_mode(0o751)).unwrap();
            pending.request.path = "file".into();
            pending.prepare(&project.0).unwrap();
            pending.commit(&project.0).unwrap();
            assert_eq!(
                fs::metadata(project.0.join("file"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o751
            );
        }
        assert!(crate::tools::request("EDIT: file").is_none());
    }
    #[test]
    fn edit_creates_missing_files_and_recovers_creation() {
        for operation in ["append", "prepend"] {
            for content in ["entry\n", ""] {
                let project = Project::new();
                let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: new.txt\n  operation: {operation}\n  input: {}\n  output: diff\n", serde_json::to_string(content).unwrap())).unwrap();
                let outputs = run_with(&workflow, &project.0, |_| panic!("no model")).unwrap();
                assert_eq!(
                    fs::read_to_string(project.0.join("new.txt")).unwrap(),
                    content
                );
                if !content.is_empty() {
                    assert!(outputs["diff"].contains("--- /dev/null"));
                }
                let mut history = History::open(&project.log()).unwrap();
                // Simulate a crash after publication but before saving OUTPUT.
                history.steps[0].pop();
                history.save().unwrap();
                let path = history.path.clone();
                drop(history);
                let mut history = History::open(&path).unwrap();
                assert_eq!(
                    continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap(),
                    outputs
                );
                assert_eq!(
                    fs::read_to_string(project.0.join("new.txt")).unwrap(),
                    content
                );
                // Simulate a prepared creation which has not reached the filesystem yet.
                history.steps[0].pop();
                fs::remove_file(project.0.join("new.txt")).unwrap();
                continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap();
                assert_eq!(
                    fs::read_to_string(project.0.join("new.txt")).unwrap(),
                    content
                );
                // A different file created in between must not be replaced.
                history.steps[0].pop();
                fs::write(project.0.join("new.txt"), "other writer").unwrap();
                assert!(
                    continue_with(&mut history, |_| panic!("conflict"), &mut answers(&[]))
                        .unwrap_err()
                        .contains("conflict")
                );
                assert_eq!(
                    fs::read_to_string(project.0.join("new.txt")).unwrap(),
                    "other writer"
                );
            }
        }
    }

    #[test]
    fn missing_edit_targets_remain_confined_and_line_edits_require_files() {
        let project = Project::new();
        let outside = Project::new();
        for path in [
            "missing-parent/new.txt".to_owned(),
            outside.0.join("new.txt").to_str().unwrap().to_owned(),
        ] {
            let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: {}\n  operation: append\n  input: text\n", serde_json::to_string(&path).unwrap())).unwrap();
            assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
        }
        for operation in ["insert", "delete", "replace"] {
            let coordinates = if operation == "insert" {
                "line: 1"
            } else {
                "start: 1\n  end: 1"
            };
            let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: missing.txt\n  operation: {operation}\n  {coordinates}\n  version: '{}'\n", crate::tools::edit::version(""))).unwrap();
            assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
            assert!(!project.0.join("missing.txt").exists());
        }
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(outside.0.join("absent"), project.0.join("dangling"))
                .unwrap();
            let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: EDIT\n  path: dangling\n  operation: append\n  input: text\n").unwrap();
            assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
            assert!(!outside.0.join("absent").exists());
        }
    }
    #[test]
    fn skipped_write_completes_and_resume_does_not_recreate_it() {
        let project = Project::new();
        fs::write(project.0.join("file"), "original").unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: WRITE\n  path: file\n  input: replacement\n  skip: true\n  output: skipped\n- tool: WRITE\n  path: next\n  input: continued\n").unwrap();
        let outputs = run_with(&workflow, &project.0, |_| panic!("no model")).unwrap();
        assert_eq!(outputs["skipped"], "");
        assert_eq!(
            fs::read_to_string(project.0.join("file")).unwrap(),
            "original"
        );
        assert_eq!(
            fs::read_to_string(project.0.join("next")).unwrap(),
            "continued"
        );
        fs::remove_file(project.0.join("file")).unwrap();
        let mut history = History::open(&project.log()).unwrap();
        assert_eq!(
            continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap(),
            outputs
        );
        assert!(!project.0.join("file").exists());
    }

    #[test]
    fn delete_requires_confirmation_unless_forced() {
        let project = Project::new();
        fs::write(project.0.join("obsolete"), "content").unwrap();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- tool: DELETE\n  path: obsolete\n").unwrap();
        assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
        assert!(project.0.join("obsolete").exists());
        run_interactive_with(
            &workflow,
            &project.0,
            |_| panic!("no model"),
            &mut answers(&[""]),
        )
        .unwrap();
        assert!(!project.0.join("obsolete").exists());

        fs::write(project.0.join("forced"), "content").unwrap();
        let forced: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- tool: DELETE\n  path: forced\n  force: true\n",
        )
        .unwrap();
        run_with(&forced, &project.0, |_| panic!("no model")).unwrap();
        assert!(!project.0.join("forced").exists());
    }

    #[test]
    fn agent_delete_requires_permission_and_reports_its_result() {
        let project = Project::new();
        fs::write(
            project.0.join(".agents/cleaner.md"),
            "---\nadapter: codex\nDELETE_TOOL: allow\n---\nClean generated files.",
        )
        .unwrap();
        fs::write(project.0.join("obsolete"), "content").unwrap();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- agent: cleaner\n  output: result\n")
                .unwrap();
        let mut calls = 0;
        let outputs = run_interactive_with(
            &workflow,
            &project.0,
            |invocation| {
                calls += 1;
                let prompt = invocation.arguments.last().unwrap();
                match calls {
                    1 => {
                        assert!(prompt.contains("External tool: DELETE"));
                        Ok("DELETE: {\"path\":\"obsolete\"}".into())
                    }
                    2 => {
                        assert!(prompt.contains("Tool result (DELETE)"));
                        Ok("Deleted the obsolete file.".into())
                    }
                    _ => panic!("unexpected model call"),
                }
            },
            &mut answers(&[""]),
        )
        .unwrap();
        assert_eq!(outputs["result"], "Deleted the obsolete file.");
        assert!(!project.0.join("obsolete").exists());
    }

    #[test]
    fn agent_delete_force_requires_the_separate_permission() {
        let project = Project::new();
        fs::write(
            project.0.join(".agents/cleaner.md"),
            "---\nadapter: codex\nDELETE_TOOL: allow\nDELETE_WITHOUT_CONFIRM: allow\n---\nClean generated files.",
        )
        .unwrap();
        fs::write(project.0.join("obsolete"), "content").unwrap();
        let workflow: Workflow =
            serde_yaml::from_str("version: 1\nsteps:\n- agent: cleaner\n  output: result\n")
                .unwrap();
        let mut calls = 0;
        let outputs = run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            match calls {
                1 => Ok("DELETE: {\"path\":\"obsolete\",\"force\":true}".into()),
                2 => {
                    assert!(
                        invocation
                            .arguments
                            .last()
                            .unwrap()
                            .contains("Tool result (DELETE)")
                    );
                    Ok("Deleted without confirmation.".into())
                }
                _ => panic!("unexpected model call"),
            }
        })
        .unwrap();
        assert_eq!(outputs["result"], "Deleted without confirmation.");
        assert!(!project.0.join("obsolete").exists());
    }
    #[test]
    fn if_reuses_docs_or_generates_them_and_restores_loop_outputs() {
        let project = Project::new();
        fs::create_dir(project.0.join("tools")).unwrap();
        fs::write(
            project.0.join("tools/file-exists.sh"),
            include_str!("../tools/file-exists.sh"),
        )
        .unwrap();
        fs::write(project.0.join("cached"), "existing documentation").unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [cached, missing]\n  iter:\n  - custom-tool: file-exists\n    input: ['{{ loop.item }}']\n    output: exists\n  - tool: IF\n    input: '{{ loop.exists }}'\n    is_true:\n    - tool: READ\n      input: '{{ loop.item }}'\n      output: explain\n    is_false:\n    - agent: second\n      input: generate\n      output: explain\n    - tool: WRITE\n      path: '{{ loop.item }}'\n      input: '{{ loop.explain }}'\n  - tool: EDIT\n    path: index\n    operation: append\n    input: '{{ loop.explain }}'\n").unwrap();
        let mut calls = 0;
        let outputs = run_with(&workflow, &project.0, |_| {
            calls += 1;
            Ok("generated documentation".into())
        })
        .unwrap();
        assert_eq!(calls, 1);
        assert!(outputs.is_empty());
        assert_eq!(
            fs::read_to_string(project.0.join("index")).unwrap(),
            "existing documentationgenerated documentation"
        );
        let mut history = History::open(&project.log()).unwrap();
        assert!(
            history
                .labels
                .iter()
                .any(|s| s == "Step 1.1.2.true.1 — READ")
        );
        assert!(
            history
                .labels
                .iter()
                .any(|s| s == "Step 1.2.2.false.1 — second")
        );
        fs::remove_file(project.0.join("cached")).unwrap();
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap();
        assert_eq!(
            fs::read_to_string(project.0.join("index")).unwrap(),
            "existing documentationgenerated documentation"
        );
    }

    #[test]
    fn if_recovers_pending_nested_branch_and_rejects_edited_conditions() {
        let project = Project::new();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: selected\n- tool: IF\n  input: '{{ outputs.selected }}'\n  is_true:\n  - tool: LOOP\n    input: [a, b]\n    iter:\n    - agent: second\n      input: '{{ loop.item }}'\n  is_false:\n  - tool: WRITE\n    path: wrong-branch\n    input: wrong\n- tool: WRITE\n  path: final\n  input: done\n").unwrap();
        let mut calls = 0;
        assert!(
            run_with(&workflow, &project.0, |_| {
                calls += 1;
                match calls {
                    1 => Ok("true\n".into()),
                    2 => Ok("first".into()),
                    _ => Err("interrupted".into()),
                }
            })
            .is_err()
        );
        let mut history = History::open(&project.log()).unwrap();
        assert!(!project.0.join("wrong-branch").exists());
        let original = history.steps[1][0].clone();
        history.steps[1][0] = Block::Input("false".into());
        assert!(
            continue_with(
                &mut history,
                |_| panic!("must validate first"),
                &mut answers(&[])
            )
            .is_err()
        );
        assert!(!project.0.join("wrong-branch").exists());
        history.steps[1][0] = original;
        let mut resumed = 0;
        continue_with(
            &mut history,
            |invocation| {
                resumed += 1;
                assert!(invocation.arguments.last().unwrap().contains("b"));
                Ok("second".into())
            },
            &mut answers(&[]),
        )
        .unwrap();
        assert_eq!(resumed, 1);
        assert_eq!(fs::read_to_string(project.0.join("final")).unwrap(), "done");
        // Truncating a branch result while retaining later records must fail preflight.
        history.steps[3].pop();
        assert!(continue_with(&mut history, |_| panic!("invalid"), &mut answers(&[])).is_err());
    }

    #[test]
    fn if_validates_unselected_agents_and_handles_empty_selection() {
        let project = Project::new();
        let invalid: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: IF\n  input: true\n  is_true:\n  - tool: TREE\n  is_false:\n  - agent: missing\n").unwrap();
        assert!(run_with(&invalid, &project.0, |_| panic!("validate first")).is_err());
        assert!(!project.0.join("history").exists());
        let empty: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: IF\n  input: false\n  is_true:\n  - tool: READ\n    input: missing\n- tool: WRITE\n  path: done\n  input: ok\n").unwrap();
        run_with(&empty, &project.0, |_| panic!("no model")).unwrap();
        assert_eq!(fs::read_to_string(project.0.join("done")).unwrap(), "ok");
        let mut history = History::open(&project.log()).unwrap();
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap();
        assert!(crate::tools::request("IF: true").is_none());
        assert!(!crate::tools::supports("IF"));
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
