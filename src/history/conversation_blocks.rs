use super::{Block, Conversation, ConversationMessage, conversation::ConversationTool};

impl Conversation {
    pub(crate) fn blocks(&self) -> Vec<Block> {
        self.data
            .messages
            .iter()
            .map(|message| match message {
                ConversationMessage::User { content } => Block::Input(content.clone()),
                ConversationMessage::Assistant { content } => Block::Output(content.clone()),
                ConversationMessage::Tool { name, content } => match name {
                    ConversationTool::Tree => Block::Tree(content.clone()),
                    ConversationTool::Read => Block::Read(content.clone()),
                    ConversationTool::Edit => Block::Edit(content.clone()),
                    ConversationTool::Write => Block::Write(content.clone()),
                    ConversationTool::Delete => Block::Delete(content.clone()),
                },
            })
            .collect()
    }
}
