use super::{ExternalDeleteRequest, external_delete_request};
use crate::history::Block;

pub(in crate::runner) fn external_delete_pending(
    blocks: &[Block],
) -> Option<Result<ExternalDeleteRequest, String>> {
    let [.., Block::Output(request), Block::Input(pending)] = blocks else {
        return None;
    };
    let request = external_delete_request(request)?;
    Some(request.and_then(|request| {
        let pending: ExternalDeleteRequest = serde_json::from_str(pending)
            .map_err(|error| format!("invalid DELETE_TOOL history: {error}"))?;
        (pending == request)
            .then_some(pending)
            .ok_or_else(|| "DELETE_TOOL history does not match its request".into())
    }))
}
