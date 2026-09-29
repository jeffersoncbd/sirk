use crate::history::{Block, History};
use serde::Deserialize;

pub(super) fn delete_request(history: &mut History, payload: &str) -> Result<String, String> {
    #[derive(Deserialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    struct Request {
        path: String,
        #[serde(default)]
        force: bool,
    }
    let request: Request = serde_json::from_str(payload)
        .map_err(|error| format!("invalid DELETE_TOOL request: {error}"))?;
    if request.force && !history.snapshot.agent.delete_without_confirm {
        return Ok("DELETE failed: `force: true` requires DELETE_WITHOUT_CONFIRM: allow. Correct the request and try again.".into());
    }
    if history.blocks.windows(3).any(|window| {
        matches!(window, [Block::Output(previous), Block::Input(_), Block::Delete(_)]
            if previous.trim().strip_prefix("DELETE:")
                .and_then(|text| serde_json::from_str::<Request>(text.trim()).ok())
                .is_some_and(|old| old == request))
    }) {
        return Ok("No changes applied: this DELETE request was already completed. Do not repeat it; provide a final response or a different request.".into());
    }
    history.blocks.push(Block::Input(payload.to_owned()));
    if !request.force {
        return Err("user input is unavailable in this mode".into());
    }
    match crate::tools::delete::delete(&history.snapshot.directory, &request.path) {
        Ok(()) => Ok(String::new()),
        Err(error) => Ok(format!(
            "DELETE failed: {error}. Correct the request and try again."
        )),
    }
}
