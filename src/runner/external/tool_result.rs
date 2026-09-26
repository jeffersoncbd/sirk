use crate::history::Block;

pub(in crate::runner) fn tool_result(tool: &str, result: String) -> Block {
    if tool == "TREE" {
        Block::Tree(result)
    } else {
        Block::Read(result)
    }
}
