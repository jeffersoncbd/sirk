use super::super::question::question;
use crate::history::Block;

pub(super) fn validate_step_blocks(blocks: &[Block], tool_step: bool) -> Result<bool, String> {
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
