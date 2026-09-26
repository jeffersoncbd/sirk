//! Structural validation of persisted workflow transcripts.

use super::{bootstrap::question, external::*};
use crate::{
    history::{Block, History},
    workflow::{Step, condition, loop_items},
};

pub(super) fn validate_blocks(history: &History) -> Result<(), String> {
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
            } else if step.tool.as_deref() == Some("ASK") {
                if !validate_ask_blocks(blocks)? {
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

fn validate_ask_blocks(blocks: &[Block]) -> Result<bool, String> {
    match blocks {
        [] => Ok(false),
        [Block::Ask(question)] if !question.trim().is_empty() => Ok(false),
        [Block::Ask(question), Block::Input(_)] if !question.trim().is_empty() => Ok(true),
        _ => Err("invalid ASK history; remove the edited result and everything after it".into()),
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
    let initial_question = match blocks.get(position) {
        Some(Block::Ask(text)) if !text.trim().is_empty() => {
            position += 1;
            true
        }
        Some(Block::Input(_)) => false,
        Some(_) => return Err("invalid conversation; expected initial input".into()),
        None => return Ok(false),
    };
    if matches!(blocks.get(position), Some(Block::Ask(_))) {
        return Err("invalid conversation; ASK can only start an agent turn".into());
    }
    if let Some(Block::Input(answer)) = blocks.get(position) {
        position += 1;
        if initial_question && !validate_user_tool_result(blocks, &mut position, answer)? {
            return Ok(false);
        }
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
            let Some(Block::Input(answer)) = blocks.get(position) else {
                return Ok(false);
            };
            position += 1;
            if !validate_user_tool_result(blocks, &mut position, answer)? {
                return Ok(false);
            }
            continue;
        }
        return Ok(position == blocks.len());
    }
    Ok(false)
}
