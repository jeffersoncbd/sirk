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
    let mut conversation = conversation_id
        .as_deref()
        .map(|conversation_id| {
            history.open_conversation(&flow_id, &history.snapshot.agent.id, Some(conversation_id))
        })
        .transpose()?;
    if let Some(conversation) = conversation.as_ref() {
        if conversation.data.status != ConversationStatus::AwaitingUser {
            return Err(format!(
                "conversation `{}` is not awaiting user input",
                conversation.data.conversation_id
            ));
        }
        history.blocks = conversation.blocks();
    }
    let result = super::conversation::conversation(
        &mut history,
        &mut conversation,
        &flow_id,
        input,
        super::execute::execute,
    );
    let outcome = match result {
        Ok(outcome) => outcome,
        Err(error) => {
            if let Some(conversation) = conversation.as_mut() {
                conversation.replace_blocks(&history.blocks);
                conversation.data.status = ConversationStatus::Failed;
                conversation.save()?;
            }
            history.record_usage_summary()?;
            return Err(error);
        }
    };
    let conversation_id = if let Some(conversation) = conversation.as_mut() {
        conversation.replace_blocks(&history.blocks);
        conversation.data.status = match &outcome {
            super::outcome::AgentOutcome::Result(_) => ConversationStatus::Completed,
            super::outcome::AgentOutcome::Ask(_) => ConversationStatus::AwaitingUser,
        };
        conversation.save()?;
        Some(conversation.data.conversation_id.clone())
    } else {
        None
    };
    history.record_usage_summary()?;
    Ok(super::outcome::AgentExecution {
        conversation_id,
        outcome,
    })
}
