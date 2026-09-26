use std::path::Path;

pub(in crate::runner) fn prepare_external_edit(
    mut request: crate::tools::edit::Request,
    directory: &Path,
) -> Result<crate::tools::edit::Pending, String> {
    let before = crate::tools::read::read(directory, &request.path)?;
    let line_count = before.split_inclusive('\n').count();
    match request.operation {
        crate::tools::edit::Operation::Insert
            if request.line.is_some_and(|line| line > line_count + 1) =>
        {
            return Err(format!(
                "EDIT insert line is beyond EOF; `{}` has {line_count} lines",
                request.path
            ));
        }
        crate::tools::edit::Operation::Delete | crate::tools::edit::Operation::Replace
            if request.end.is_some_and(|end| end > line_count) =>
        {
            return Err(format!(
                "EDIT range is beyond EOF; `{}` has {line_count} lines",
                request.path
            ));
        }
        _ => (),
    }
    request.version = Some(crate::tools::edit::version(&before));
    let pending = crate::tools::edit::Pending {
        request,
        before: Some(before),
        was_missing: false,
    };
    pending.validate()?;
    Ok(pending)
}
