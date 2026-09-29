use crate::history::History;

pub(super) fn write_request(history: &mut History, payload: &str) -> Result<String, String> {
    let mut request: serde_json::Value =
        serde_json::from_str(payload).map_err(|error| format!("invalid WRITE request: {error}"))?;
    let object = request
        .as_object_mut()
        .ok_or("invalid WRITE request: expected a JSON object")?;
    object.insert(
        "operation".into(),
        serde_json::Value::String("write".into()),
    );
    super::edit_request::edit_request(
        history,
        &serde_json::to_string(&request).map_err(|error| error.to_string())?,
    )
}
