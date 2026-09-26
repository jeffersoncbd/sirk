use crate::history::Block;

pub(in crate::runner) fn validate_user_tool_result(
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
