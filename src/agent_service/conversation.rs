use crate::{
    adapters,
    harness::RunRequest,
    history::{Block, History},
    services::Invocation,
    tools,
};

pub(super) fn conversation(
    history: &mut History,
    input: String,
    mut execute: impl FnMut(&Invocation) -> Result<String, String>,
) -> Result<String, String> {
    if let Some(question) = &history.snapshot.agent.ask {
        history.blocks.push(Block::Ask(question.clone()));
        return Err("user input is unavailable in this mode".into());
    }
    history.blocks.push(Block::Input(input));
    let agent = history.snapshot.agent.clone();
    let adapter = adapters::resolve(&agent.adapter).ok_or("missing agent adapter")?;
    loop {
        let prompt = super::prompt::prompt(history);
        history.record_input(&prompt)?;
        let invocation = adapter
            .invocation(&RunRequest {
                prompt,
                working_directory: history.snapshot.directory.clone(),
                model: agent.model.clone(),
                event_stream: false,
            })
            .map_err(|error| error.to_string())?
            .with_prefix(&agent.call_prefix);
        let response = adapter
            .response(execute(&invocation)?)
            .map_err(|error| error.to_string())?;
        history.record_output(&response)?;
        if response.trim().is_empty() {
            return Err("agent returned an empty response; input remains pending".into());
        }
        history.blocks.push(Block::Output(response.clone()));
        if let Some(payload) = response.trim().strip_prefix("EDIT:") {
            if !agent.edit_tool {
                return Err("agent requested EDIT_TOOL without permission".into());
            }
            let result = super::edit_request::edit_request(history, payload.trim())?;
            history.blocks.push(Block::Edit(result));
        } else if let Some(payload) = response.trim().strip_prefix("WRITE:") {
            if !agent.edit_tool {
                return Err("agent requested EDIT_TOOL without permission".into());
            }
            let result = super::write_request::write_request(history, payload.trim())?;
            history.blocks.push(Block::Write(result));
        } else if let Some(payload) = response.trim().strip_prefix("DELETE:") {
            if !agent.delete_tool {
                return Err("agent requested DELETE_TOOL without permission".into());
            }
            let result = super::delete_request::delete_request(history, payload.trim())?;
            history.blocks.push(Block::Delete(result));
        } else if let Some((tool, argument)) = tools::request(&response) {
            if tool == "TREE" && !agent.tree_tool {
                return Err("agent requested TREE_TOOL without permission".into());
            }
            let result = if tool == "READ" {
                let page = tools::read::page(&history.snapshot.directory, argument)?;
                if agent.edit_tool {
                    tools::read::enumerate_from(&page.content, page.offset)
                } else {
                    page.content
                }
            } else {
                tools::execute_with_input(tool, argument, &history.snapshot.directory)?
            };
            history.blocks.push(if tool == "TREE" {
                Block::Tree(result)
            } else {
                Block::Read(result)
            });
        } else if response.trim_start().starts_with("ASK:") {
            return Err("user input is unavailable in this mode".into());
        } else {
            return Ok(response);
        }
    }
}
