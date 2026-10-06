use super::{Block, Conversation, ConversationMessage, conversation::ConversationTool};

impl Conversation {
    pub(crate) fn replace_blocks(&mut self, blocks: &[Block]) {
        self.data.messages = blocks
            .iter()
            .filter_map(|block| match block {
                Block::Ask(_) => None,
                Block::Input(content) => Some(ConversationMessage::User {
                    content: content.clone(),
                }),
                Block::Output(content) => Some(ConversationMessage::Assistant {
                    content: content.clone(),
                }),
                Block::Tree(content) => Some(ConversationMessage::Tool {
                    name: ConversationTool::Tree,
                    content: content.clone(),
                }),
                Block::Read(content) => Some(ConversationMessage::Tool {
                    name: ConversationTool::Read,
                    content: content.clone(),
                }),
                Block::Edit(content) => Some(ConversationMessage::Tool {
                    name: ConversationTool::Edit,
                    content: content.clone(),
                }),
                Block::Write(content) => Some(ConversationMessage::Tool {
                    name: ConversationTool::Write,
                    content: content.clone(),
                }),
                Block::Delete(content) => Some(ConversationMessage::Tool {
                    name: ConversationTool::Delete,
                    content: content.clone(),
                }),
            })
            .collect();
    }
}
