use super::super::{
    json_response::JsonResponse,
    schemas::{
        requests::DirectoryRequest,
        responses::{ErrorResponse, StatusResponse, SuccessStatus},
    },
};
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
#[utoipa::path(
    post,
    path = "/v1/git/add",
    operation_id = "gitAdd",
    summary = "Stage all changes below a directory",
    description = "Runs Git add for every change below `directory`, including deletions. The requested directory must exist and be within a Git working tree visible to the server.",
    request_body(
        content = DirectoryRequest,
        description = "Server-visible Git directory.",
        content_type = "application/json",
        example = json!({"directory": "/workspace/project"})
    ),
    responses(
        (status = 200, description = "All changes below the requested directory were staged.", body = StatusResponse, example = json!({"status": "ok"})),
        (status = 400, description = "The request body is invalid, incomplete, or contains an unknown field.", body = ErrorResponse, example = json!({"error": "Invalid request"})),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = ErrorResponse, example = json!({"error": "Method not allowed"})),
        (status = 500, description = "The Git add operation could not be completed.", body = ErrorResponse, example = json!({"error": "An operation-specific error message."}))
    )
)]
pub(in crate::http) async fn git_add(
    payload: Result<Json<DirectoryRequest>, JsonRejection>,
) -> Response {
    let Json(request) = match payload {
        Ok(request) => request,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                JsonResponse(ErrorResponse {
                    error: "Invalid request".to_owned(),
                }),
            )
                .into_response();
        }
    };
    match tokio::task::spawn_blocking(move || crate::git_service::add(&request.directory)).await {
        Ok(Ok(())) => JsonResponse(StatusResponse {
            status: SuccessStatus::Ok,
        })
        .into_response(),
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse { error }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse {
                error: format!("Git add task failed: {error}"),
            }),
        )
            .into_response(),
    }
}
