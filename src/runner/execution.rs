//! Step execution and conversation driving.

use crate::{
    adapters,
    harness::RunRequest,
    history::{Block, History},
    input::UserInput,
    services::Invocation,
    workflow::{Step, condition, loop_items, loop_target},
};
use std::collections::BTreeMap;

use super::{
    bootstrap::{question, validate_snapshot},
    external::*,
    history_validation::validate_blocks,
};

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
            let version_content = if step.tool.as_deref() == Some("READ") && step.enumerate() {
                crate::tools::read::enumerated_content(&result)?
            } else {
                result.clone()
            };
            let version = crate::tools::edit::version(&version_content);
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
        if step.tool.as_deref() == Some("ASK") {
            if history.steps[index].is_empty() {
                history.steps[index].push(Block::Ask(step.render_input(outputs, locals)?));
                history.save()?;
            }
            if let Some(Block::Input(answer)) = history.steps[index].last() {
                return Ok(answer.clone());
            }
            let Block::Ask(question) = history.steps[index].last().unwrap() else {
                return Err("invalid ASK history".into());
            };
            let answer = self.input.ask(question)?;
            history.steps[index].push(Block::Input(answer.clone()));
            history.save()?;
            return Ok(answer);
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
                        let result = crate::tools::execute_with_input(
                            tool,
                            argument,
                            &history.snapshot.directory,
                        )?;
                        if tool == "READ" && step.enumerate() {
                            crate::tools::read::enumerate(&result)
                        } else {
                            result
                        }
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
                Block::Input(_) if user_tool_request(&history.steps[index]).is_some() => {
                    let (tool, argument) = user_tool_request(&history.steps[index])
                        .expect("matched user tool request");
                    let tool = tool.to_owned();
                    let argument = argument.to_owned();
                    let result = crate::tools::execute_with_input(
                        &tool,
                        &argument,
                        &history.snapshot.directory,
                    )?;
                    print!("{result}");
                    history.steps[index].push(tool_result(&tool, result));
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
                        if tool == "TREE" && !agent.tree_tool {
                            return Err("agent requested TREE_TOOL without permission".into());
                        }
                        let result = crate::tools::execute_with_input(
                            tool,
                            argument,
                            &history.snapshot.directory,
                        )?;
                        let result = if tool == "READ" && agent.edit_tool {
                            crate::tools::read::enumerate(&result)
                        } else {
                            result
                        };
                        print!("{result}");
                        history.steps[index].push(tool_result(tool, result));
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
                    if agent.tree_tool {
                        prompt.push_str("\n\nAvailable tool: TREE. To list project files respecting Git ignores, respond with exactly TREE and nothing else. The tool result will be returned so you can continue your response.");
                    }
                    prompt.push_str("\nAvailable tool: READ. To read a UTF-8 file inside the execution directory, respond with exactly READ: <path> on one line, without quotes or code fences. Paths are relative to the execution directory. The file content will be returned so you can continue your response.");
                    if agent.edit_tool {
                        prompt.push_str("\n\nExternal tool: EDIT. This tool is executed after your response; do not try to use an internal tool. To request exactly one file edit, respond only with `EDIT:` followed by a JSON object, without code fences or any other text. The object must contain `path`, `operation`, and `input`. `operation` is one of `insert`, `delete`, `replace`, `prepend`, or `append`. For `insert`, also provide positive integer `line`. For `delete` and `replace`, also provide positive integer `start` and `end` (inclusive). `delete` requires an empty `input`. `prepend` and `append` take no coordinates. Paths are relative to the execution directory. Do not include `version`; the external tool verifies the current document before applying the edit. Before a coordinate-based edit, request READ for the target; its result is numbered. After a successful edit, you receive its diff and may request another edit or provide your final answer. A Tool result (EDIT) beginning with `EDIT failed:` means no change was made; correct the request and try again. Before requesting another edit, inspect every Tool result (EDIT) in the conversation. Never repeat a request that was already applied; if the diff completes the work, provide your final answer. Request edits only when necessary.");
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
                        .map_err(|e| e.to_string())?
                        .with_prefix(&agent.call_prefix);
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
