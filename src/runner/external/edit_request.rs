pub(in crate::runner) fn external_edit_request(
    text: &str,
) -> Option<Result<crate::tools::edit::Request, String>> {
    let payload = text.trim().strip_prefix("EDIT:")?.trim();
    Some(
        serde_json::from_str(payload)
            .map_err(|error| format!("invalid EDIT_TOOL request: {error}"))
            .and_then(|request: crate::tools::edit::Request| {
                if request.version.is_some() {
                    return Err("EDIT_TOOL must not include `version`".into());
                }
                Ok(request)
            }),
    )
}
