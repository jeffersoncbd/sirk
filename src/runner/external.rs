//! Parsing and recovery helpers for agent-requested tools.

use crate::history::Block;
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::bootstrap::question;

pub(super) fn user_tool_request(blocks: &[Block]) -> Option<(&str, &str)> {
    let [.., previous, Block::Input(answer)] = blocks else {
        return None;
    };
    if matches!(previous, Block::Ask(_))
        || matches!(previous, Block::Output(output) if question(output).is_some())
    {
        crate::tools::request(answer)
    } else {
        None
    }
}

pub(super) fn tool_result(tool: &str, result: String) -> Block {
    if tool == "TREE" {
        Block::Tree(result)
    } else {
        Block::Read(result)
    }
}

pub(super) fn validate_user_tool_result(
    blocks: &[Block],
    position: &mut usize,
    answer: &str,
) -> Result<bool, String> {
    let Some((tool, _)) = crate::tools::request(answer) else {
        return Ok(true);
    };
    let expected = if tool == "TREE" { "TREE" } else { "READ" };
    let Some(result) = blocks.get(*position) else {
        return Ok(false);
    };
    if !matches!(
        (expected, result),
        ("TREE", Block::Tree(_)) | ("READ", Block::Read(_))
    ) {
        return Err("invalid conversation; tool result does not match user request".into());
    }
    *position += 1;
    Ok(true)
}

pub(crate) const DUPLICATE_EDIT_RESULT: &str = "No changes applied: this EDIT request was already completed. Do not repeat it; provide a final response or a different edit.";
pub(crate) const EDIT_FAILURE_PREFIX: &str = "EDIT failed: ";
pub(super) const DUPLICATE_DELETE_RESULT: &str = "No changes applied: this DELETE request was already completed. Do not repeat it; provide a final response or a different request.";
pub(super) const DELETE_FAILURE_PREFIX: &str = "DELETE failed: ";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(super) struct ExternalDeleteRequest {
    pub(super) path: String,
    #[serde(default)]
    pub(super) force: bool,
}

pub(super) fn external_edit_request(
    text: &str,
) -> Option<Result<crate::tools::edit::Request, String>> {
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

pub(super) fn prepare_external_edit(
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

pub(super) fn external_edit_pending(
    blocks: &[Block],
) -> Option<Result<crate::tools::edit::Pending, String>> {
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

pub(super) fn completed_external_edit(
    blocks: &[Block],
    request: &crate::tools::edit::Request,
) -> bool {
    blocks.windows(3).any(|window| {
        let [Block::Output(previous), Block::Input(_), Block::Edit(_)] = window else {
            return false;
        };
        external_edit_request(previous).is_some_and(|previous| previous.as_ref() == Ok(request))
    })
}

pub(super) fn external_delete_request(text: &str) -> Option<Result<ExternalDeleteRequest, String>> {
    let payload = text.trim().strip_prefix("DELETE:")?.trim();
    Some(
        serde_json::from_str(payload)
            .map_err(|error| format!("invalid DELETE_TOOL request: {error}")),
    )
}

pub(super) fn external_delete_pending(
    blocks: &[Block],
) -> Option<Result<ExternalDeleteRequest, String>> {
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

pub(super) fn completed_external_delete(blocks: &[Block], request: &ExternalDeleteRequest) -> bool {
    blocks.windows(3).any(|window| {
        let [Block::Output(previous), Block::Input(_), Block::Delete(_)] = window else {
            return false;
        };
        external_delete_request(previous).is_some_and(|previous| previous.as_ref() == Ok(request))
    })
}
