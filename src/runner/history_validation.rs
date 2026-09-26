//! Structural validation of persisted workflow transcripts.

#[path = "history_validation/agent_blocks.rs"]
mod agent_blocks;
#[path = "history_validation/ask_blocks.rs"]
mod ask_blocks;
#[path = "history_validation/edit_blocks.rs"]
mod edit_blocks;
#[path = "history_validation/step_blocks.rs"]
mod step_blocks;
#[path = "history_validation/visit.rs"]
mod visit;

use crate::history::History;
use visit::visit;

pub(super) fn validate_blocks(history: &History) -> Result<(), String> {
    let mut cursor = 0;
    visit(history, &history.snapshot.workflow.steps, "", &mut cursor)?;
    if cursor != history.steps.len() {
        return Err(
            "pending step has later history; remove the edited result and everything after it"
                .into(),
        );
    }
    Ok(())
}
