use super::Block;

pub(super) fn finish(
    active: &mut Option<(&str, Vec<String>)>,
    steps: &mut [Vec<Block>],
) -> Result<(), String> {
    if let Some((marker, mut lines)) = active.take() {
        if lines.last().is_some_and(String::is_empty) {
            lines.pop();
        }
        let value = lines.join("\n");
        let block = match marker {
            "==> ASK" => Block::Ask(value),
            "==> INPUT" => Block::Input(value),
            "==> TREE" => Block::Tree(value),
            "==> READ" => Block::Read(value),
            "==> EDIT" => Block::Edit(value),
            "==> DELETE" => Block::Delete(value),
            _ => Block::Output(value),
        };
        steps
            .last_mut()
            .ok_or("content before a step header")?
            .push(block);
    }
    Ok(())
}
