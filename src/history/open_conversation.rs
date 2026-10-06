use super::{
    Conversation, ConversationMessage, ConversationStatus, History, conversation::ConversationData,
    conversation_id::conversation_id, valid_conversation_id,
};
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

impl History {
    pub(crate) fn open_conversation(
        &self,
        flow_id: &str,
        agent: &str,
        requested_id: Option<&str>,
    ) -> Result<Conversation, String> {
        let directory = self
            .path
            .parent()
            .ok_or("missing flow directory")?
            .join("conversations");
        if let Some(conversation_id) = requested_id {
            if !valid_conversation_id(conversation_id) {
                return Err("invalid conversation ID".to_owned());
            }
            let path = directory.join(format!("{conversation_id}.json"));
            let content = fs::read(&path).map_err(|error| {
                format!("could not load conversation `{conversation_id}`: {error}")
            })?;
            let data: ConversationData =
                serde_json::from_slice(&content).map_err(|error| error.to_string())?;
            if data.version != 1
                || data.conversation_id != conversation_id
                || data.flow_id != flow_id
                || data.agent != agent
            {
                return Err(format!(
                    "conversation `{conversation_id}` does not belong to this flow and agent"
                ));
            }
            return Ok(Conversation { path, data });
        }
        loop {
            let conversation_id = conversation_id()?;
            let path = directory.join(format!("{conversation_id}.json"));
            let data = ConversationData {
                version: 1,
                conversation_id,
                flow_id: flow_id.to_owned(),
                agent: agent.to_owned(),
                status: ConversationStatus::Active,
                messages: Vec::<ConversationMessage>::new(),
            };
            let mut content =
                serde_json::to_vec_pretty(&data).map_err(|error| error.to_string())?;
            content.push(b'\n');
            match OpenOptions::new().write(true).create_new(true).open(&path) {
                Ok(mut file) => {
                    file.write_all(&content)
                        .and_then(|_| file.sync_all())
                        .map_err(|error| error.to_string())?;
                    fs::File::open(&directory)
                        .and_then(|file| file.sync_all())
                        .map_err(|error| error.to_string())?;
                    return Ok(Conversation { path, data });
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.to_string()),
            }
        }
    }
}
