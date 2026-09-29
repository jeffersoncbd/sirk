use crate::history::{Block, History};

pub(super) fn prompt(history: &History) -> String {
    let agent = &history.snapshot.agent;
    let mut prompt = agent.instructions.clone();
    if agent.tree_tool {
        prompt.push_str("\n\nAvailable tool: TREE. To list project files respecting Git ignores, respond with exactly TREE and nothing else. The tool result will be returned so you can continue your response.");
    }
    prompt.push_str("\nAvailable tool: READ. To read a UTF-8 file inside the execution directory, respond with exactly `READ: <path>` on one line. To read a page, respond with `READ: {\"path\":\"<path>\",\"offset\":<first line>,\"limit\":<line count>}`. Paths are relative to the execution directory; offset and limit are positive, one-based line values. The file content will be returned so you can continue your response.");
    if agent.edit_tool {
        prompt.push_str("\n\nExternal tools: EDIT and WRITE. Respond only with `EDIT:` followed by one JSON object containing `filePath`, `oldString`, `newString`, and optional `replaceAll`; the exact old text must occur once unless `replaceAll` is true. Respond only with `WRITE:` followed by one JSON object containing `filePath` and `content` to create or replace a whole UTF-8 file. Paths are relative to the execution directory. Both tools respect .readignore, accept only regular files inside the execution directory, return a plain diff, and never run commands. Use READ before editing an existing file. The legacy line-based EDIT form remains available for existing agents. Do not include `version`; S.I.R.K. verifies the current document before applying the change.");
    }
    if agent.delete_tool {
        prompt.push_str("\n\nExternal tool: DELETE. To request deletion of exactly one regular file, respond only with `DELETE:` followed by a JSON object containing `path`, without code fences or other text. You may include `force: true` only when DELETE_WITHOUT_CONFIRM is allowed; otherwise it is rejected. After execution, inspect the Tool result (DELETE) before responding.");
    }
    prompt.push_str("\n\nConversation (continue from the last user message):\n");
    for (index, block) in history.blocks.iter().enumerate() {
        if matches!(block, Block::Input(_))
            && index > 0
            && matches!(history.blocks[index - 1], Block::Output(ref text) if text.trim().starts_with("EDIT:") || text.trim().starts_with("WRITE:") || text.trim().starts_with("DELETE:"))
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
            Block::Write(_) => "Tool result (WRITE)",
            Block::Delete(_) => "Tool result (DELETE)",
        };
        prompt.push_str(&format!("\n{role}:\n{}\n", block.text()));
    }
    prompt
}
