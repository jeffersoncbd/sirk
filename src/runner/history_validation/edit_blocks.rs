use crate::history::Block;

pub(super) fn validate_edit_blocks(blocks: &[Block]) -> Result<bool, String> {
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
