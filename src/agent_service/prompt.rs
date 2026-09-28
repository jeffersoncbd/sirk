use crate::history::{Block, History};

pub(super) fn prompt(history: &History) -> String {
    let agent = &history.snapshot.agent;
    let mut prompt = agent.instructions.clone();
    if agent.tree_tool {
        prompt.push_str("\n\nAvailable tool: TREE. To list project files respecting Git ignores, respond with exactly TREE and nothing else. The tool result will be returned so you can continue your response.");
    }
    prompt.push_str("\nAvailable tool: READ. To read a UTF-8 file inside the execution directory, respond with exactly READ: <path> on one line, without quotes or code fences. Paths are relative to the execution directory. The file content will be returned so you can continue your response.");
    if agent.edit_tool {
        prompt.push_str("\n\nExternal tool: EDIT. This tool is executed after your response; do not try to use an internal tool. To request exactly one file edit, respond only with `EDIT:` followed by a JSON object, without code fences or any other text. The object must contain `path`, `operation`, and `input`. `operation` is one of `insert`, `delete`, `replace`, `prepend`, or `append`. For `insert`, also provide positive integer `line`. For `delete` and `replace`, also provide positive integer `start` and `end` (inclusive). `delete` requires an empty `input`. `prepend` and `append` take no coordinates. Paths are relative to the execution directory. Do not include `version`; the external tool verifies the current document before applying the edit. Before a coordinate-based edit, request READ for the target; its result is numbered. After a successful edit, you receive its diff and may request another edit or provide your final answer. A Tool result (EDIT) beginning with `EDIT failed:` means no change was made; correct the request and try again. Before requesting another edit, inspect every Tool result (EDIT) in the conversation. Never repeat a request that was already applied; if the diff completes the work, provide your final answer. Request edits only when necessary.");
    }
    if agent.delete_tool {
        prompt.push_str("\n\nExternal tool: DELETE. To request deletion of exactly one regular file, respond only with `DELETE:` followed by a JSON object containing `path`, without code fences or other text. You may include `force: true` only when DELETE_WITHOUT_CONFIRM is allowed; otherwise it is rejected. After execution, inspect the Tool result (DELETE) before responding.");
    }
    prompt.push_str("\n\nConversation (continue from the last user message):\n");
    for (index, block) in history.blocks.iter().enumerate() {
        if matches!(block, Block::Input(_))
            && index > 0
            && matches!(history.blocks[index - 1], Block::Output(ref text) if text.trim().starts_with("EDIT:") || text.trim().starts_with("DELETE:"))
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
    prompt
}
