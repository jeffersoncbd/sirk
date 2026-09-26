use super::external_edit_request;
use crate::history::Block;

pub(in crate::runner) fn external_edit_pending(
    blocks: &[Block],
) -> Option<Result<crate::tools::edit::Pending, String>> {
    let [.., Block::Output(request), Block::Input(pending)] = blocks else {
        return None;
    };
    if let Err(error) = external_edit_request(request)? {
        return Some(Err(error));
    }
    Some(
        serde_json::from_str(pending)
            .map_err(|error| format!("invalid EDIT_TOOL history: {error}")),
    )
}
