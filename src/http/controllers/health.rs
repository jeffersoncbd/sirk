use super::super::{
    json_response::JsonResponse,
    schemas::responses::{ErrorResponse, HealthResponse, SuccessStatus},
};
#[utoipa::path(
    get,
    path = "/health",
    operation_id = "health",
    summary = "Check whether the service is available",
    responses(
        (status = 200, description = "The service accepted the health-check request.", body = HealthResponse, example = json!({"status": "ok"})),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = ErrorResponse, example = json!({"error": "Method not allowed"}))
    )
)]
pub(in crate::http) async fn health() -> JsonResponse<HealthResponse> {
    JsonResponse(HealthResponse {
        status: SuccessStatus::Ok,
    })
}
