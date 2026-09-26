use crate::history::Block;

pub(super) fn validate_ask_blocks(blocks: &[Block]) -> Result<bool, String> {
    match blocks {
        [] => Ok(false),
        [Block::Ask(question)] if !question.trim().is_empty() => Ok(false),
        [Block::Ask(question), Block::Input(_)] if !question.trim().is_empty() => Ok(true),
        _ => Err("invalid ASK history; remove the edited result and everything after it".into()),
    }
}
