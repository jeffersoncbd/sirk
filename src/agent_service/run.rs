use crate::{
    adapters,
    agents::Agent,
    history::{ConversationStatus, History, Snapshot},
};
use std::path::Path;

pub(crate) fn run(
    directory: &Path,
    agent: String,
    input: String,
    flow_id: String,
    conversation_id: Option<String>,
) -> Result<super::outcome::AgentExecution, String> {
    let directory = directory
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let agent = Agent::load(&directory.join(".agents"), &agent)?;
    adapters::resolve(&agent.adapter)
        .ok_or_else(|| format!("unknown adapter `{}`", agent.adapter))?;
    let mut history = History::create(Snapshot { directory, agent }, &flow_id)?;
    let mut conversation = history.open_conversation(
        &flow_id,
        &history.snapshot.agent.id,
        conversation_id.as_deref(),
    )?;
    if conversation_id.is_some() && conversation.data.status != ConversationStatus::AwaitingUser {
        return Err(format!(
            "conversation `{}` is not awaiting user input",
            conversation.data.conversation_id
        ));
    }
    history.blocks = conversation.blocks();
    conversation.data.status = ConversationStatus::Active;
    let result = super::conversation::conversation(
        &mut history,
        &mut conversation,
        input,
        super::execute::execute,
    );
    let outcome = match result {
        Ok(outcome) => outcome,
        Err(error) => {
            conversation.replace_blocks(&history.blocks);
            conversation.data.status = ConversationStatus::Failed;
            conversation.save()?;
            history.record_usage_summary()?;
            return Err(error);
        }
    };
    conversation.replace_blocks(&history.blocks);
    conversation.data.status = match &outcome {
        super::outcome::AgentOutcome::Result(_) => ConversationStatus::Completed,
        super::outcome::AgentOutcome::Ask(_) => ConversationStatus::AwaitingUser,
    };
    conversation.save()?;
    history.record_usage_summary()?;
    Ok(super::outcome::AgentExecution {
        conversation_id: conversation.data.conversation_id,
        outcome,
    })
}
