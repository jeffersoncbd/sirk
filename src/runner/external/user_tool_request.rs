use super::super::question::question;
use crate::history::Block;

pub(in crate::runner) fn user_tool_request(blocks: &[Block]) -> Option<(&str, &str)> {
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
