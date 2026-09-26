use super::ExternalDeleteRequest;

pub(in crate::runner) fn external_delete_request(
    text: &str,
) -> Option<Result<ExternalDeleteRequest, String>> {
    let payload = text.trim().strip_prefix("DELETE:")?.trim();
    Some(
        serde_json::from_str(payload)
            .map_err(|error| format!("invalid DELETE_TOOL request: {error}")),
    )
}
