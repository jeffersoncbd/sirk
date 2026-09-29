use super::{json_response::JsonResponse, schemas::responses::ErrorResponse};
use axum::{
    http::HeaderMap,
    response::{IntoResponse, Response},
};

pub(super) fn flow_id(headers: &HeaderMap) -> Result<String, Box<Response>> {
    headers
        .get("X-Sirk-Flow-Id")
        .and_then(|value| value.to_str().ok())
        .filter(|value| crate::history::valid_flow_id(value))
        .map(str::to_owned)
        .ok_or_else(|| {
            Box::new(
                JsonResponse(ErrorResponse {
                    error: "Missing or invalid X-Sirk-Flow-Id header".to_owned(),
                })
                .into_response(),
            )
        })
}
