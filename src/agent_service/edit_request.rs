use crate::{
    history::{Block, History},
    tools::{
        edit::{Pending, Request},
        read,
    },
};

pub(super) fn edit_request(history: &mut History, payload: &str) -> Result<String, String> {
    let mut request: Request = serde_json::from_str(payload)
        .map_err(|error| format!("invalid EDIT_TOOL request: {error}"))?;
    if request.version.is_some() {
        return Err("EDIT_TOOL must not include `version`".into());
    }
    if history.blocks.windows(3).any(|window| {
        matches!(window, [Block::Output(previous), Block::Input(_), Block::Edit(_)]
            if previous.trim().strip_prefix("EDIT:")
                .and_then(|text| serde_json::from_str::<Request>(text.trim()).ok())
                .is_some_and(|old| old == request))
    }) {
        return Ok("No changes applied: this EDIT request was already completed. Do not repeat it; provide a final response or a different edit.".into());
    }
    if request.operation == crate::tools::edit::Operation::Write {
        if let Err(error) = read::allowed(&history.snapshot.directory, &request.path) {
            return Ok(format!(
                "WRITE failed: {error}. Correct the request and try again."
            ));
        }
    } else {
        let before = match read::read(&history.snapshot.directory, &request.path) {
            Ok(before) => before,
            Err(error) => {
                return Ok(format!(
                    "EDIT failed: {error}. Correct the request and try again."
                ));
            }
        };
        if request.old_string.is_none()
            && !matches!(
                request.operation,
                crate::tools::edit::Operation::Append | crate::tools::edit::Operation::Prepend
            )
        {
            request.version = Some(crate::tools::edit::version(&before));
        }
    }
    let mut pending = Pending {
        request,
        before: None,
        was_missing: false,
    };
    if let Err(error) = pending.prepare(&history.snapshot.directory) {
        return Ok(format!(
            "EDIT failed: {error}. Correct the request and try again."
        ));
    }
    history.blocks.push(Block::Input(
        serde_json::to_string(&pending).map_err(|error| error.to_string())?,
    ));
    history.save()?;
    pending.commit(&history.snapshot.directory)
}
