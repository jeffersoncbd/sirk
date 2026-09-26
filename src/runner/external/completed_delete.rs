use super::{ExternalDeleteRequest, external_delete_request};
use crate::history::Block;

pub(in crate::runner) fn completed_external_delete(
    blocks: &[Block],
    request: &ExternalDeleteRequest,
) -> bool {
    blocks.windows(3).any(|window| {
        let [Block::Output(previous), Block::Input(_), Block::Delete(_)] = window else {
            return false;
        };
        external_delete_request(previous).is_some_and(|previous| previous.as_ref() == Ok(request))
    })
}
