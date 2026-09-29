use super::request::request;
use axum::body::to_bytes;
use std::path::Path;

pub(super) async fn flow(directory: &Path) -> String {
    let response = request(
        "POST",
        "/v1/flows",
        serde_json::json!({ "directory": directory }).to_string(),
        None,
    )
    .await;
    assert_eq!(response.status(), 201);
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice::<serde_json::Value>(&body).unwrap()["flowId"]
        .as_str()
        .unwrap()
        .to_owned()
}
