use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub(crate) struct Conversation {
    pub(super) path: PathBuf,
    pub(crate) data: ConversationData,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ConversationData {
    pub version: u32,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
    #[serde(rename = "flowId")]
    pub flow_id: String,
    pub agent: String,
    pub status: ConversationStatus,
    pub messages: Vec<ConversationMessage>,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ConversationStatus {
    Active,
    AwaitingUser,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "snake_case")]
pub(crate) enum ConversationMessage {
    User {
        content: String,
    },
    Assistant {
        content: String,
    },
    Tool {
        name: ConversationTool,
        content: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ConversationTool {
    Tree,
    Read,
    Edit,
    Write,
    Delete,
}
