use super::{
    agent_blocks::validate_agent_blocks, ask_blocks::validate_ask_blocks,
    edit_blocks::validate_edit_blocks, step_blocks::validate_step_blocks,
};
use crate::{
    history::{Block, History},
    workflow::{Step, condition, loop_items},
};

pub(super) fn visit(
    history: &History,
    steps: &[Step],
    prefix: &str,
    cursor: &mut usize,
) -> Result<bool, String> {
    for (position, step) in steps.iter().enumerate() {
        let id = format!("{prefix}{}", position + 1);
        let Some(blocks) = history.steps.get(*cursor) else {
            return Ok(false);
        };
        let label = format!("Step {id} — {}", step.name());
        if history
            .labels
            .get(*cursor)
            .is_some_and(|saved| saved != &label)
        {
            return Err(format!(
                "unexpected history step; expected {label}; remove later blocks after editing"
            ));
        }
        *cursor += 1;
        if step.tool.as_deref() == Some("LOOP") {
            if blocks.is_empty() {
                return Ok(false);
            }
            let [Block::Input(input)] = blocks.as_slice() else {
                return Err("LOOP history must contain only its input array".into());
            };
            for (iteration, _) in loop_items(input)?.iter().enumerate() {
                if !visit(
                    history,
                    &step.iter,
                    &format!("{id}.{}.", iteration + 1),
                    cursor,
                )? {
                    return Ok(false);
                }
            }
        } else if step.tool.as_deref() == Some("IF") {
            if blocks.is_empty() {
                return Ok(false);
            }
            let [Block::Input(input)] = blocks.as_slice() else {
                return Err("IF history must contain only its condition".into());
            };
            let (branch, body) = step.branch(condition(input)?);
            if !visit(history, body, &format!("{id}.{branch}."), cursor)? {
                return Ok(false);
            }
        } else if step.tool.as_deref() == Some("EDIT") {
            if !validate_edit_blocks(blocks)? {
                return Ok(false);
            }
        } else if step.tool.as_deref() == Some("ASK") {
            if !validate_ask_blocks(blocks)? {
                return Ok(false);
            }
        } else if let Some(id) = &step.agent {
            let agent = history
                .snapshot
                .agents
                .iter()
                .find(|agent| &agent.id == id)
                .ok_or("missing agent configuration")?;
            if !validate_agent_blocks(
                blocks,
                agent.edit_tool,
                agent.delete_tool,
                agent.delete_without_confirm,
            )? {
                return Ok(false);
            }
        } else if !validate_step_blocks(blocks, true)? {
            return Ok(false);
        }
    }
    Ok(true)
}
