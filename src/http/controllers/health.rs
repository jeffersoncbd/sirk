use super::super::types::JsonResponse;
use serde_json::json;

pub(in crate::http) async fn health() -> JsonResponse {
    JsonResponse(json!({ "status": "ok" }))
}
