use super::super::{external::*, question::question};
use crate::history::Block;

pub(super) fn validate_agent_blocks(
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
