use super::external_edit_request;
use crate::history::Block;

pub(in crate::runner) fn completed_external_edit(
    blocks: &[Block],
    request: &crate::tools::edit::Request,
) -> bool {
    blocks.windows(3).any(|window| {
        let [Block::Output(previous), Block::Input(_), Block::Edit(_)] = window else {
            return false;
        };
        external_edit_request(previous).is_some_and(|previous| previous.as_ref() == Ok(request))
    })
}
