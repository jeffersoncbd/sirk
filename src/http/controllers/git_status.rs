use super::super::{
    json_response::JsonResponse,
    schemas::{
        requests::DirectoryRequest,
        responses::{ErrorResponse, GitStatusResponse},
    },
};
use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
#[utoipa::path(
    post,
    path = "/v1/git/status",
    operation_id = "gitStatus",
    summary = "List changed paths below a directory",
    description = "Returns sorted, unique relative UTF-8 paths for modified, untracked, and deleted regular files in the Git project below `directory`. Paths hidden by `.treeignore` are excluded. The requested directory must exist and be within a Git working tree visible to the server. `X-Sirk-Flow-Id` is required and must identify a flow created for the same directory.",
    params(("X-Sirk-Flow-Id" = String, Header, description = "Required identifier returned by POST /v1/flows for this directory.", example = "flow-18f-1234-0")),
    request_body(
        content = DirectoryRequest,
        description = "Server-visible Git directory.",
        content_type = "application/json",
        example = json!({"directory": "/workspace/project"})
    ),
    responses(
        (status = 200, description = "Changed paths were listed successfully.", body = GitStatusResponse, example = json!({"paths": ["src/lib.rs", "README.md"]})),
        (status = 400, description = "The request body or X-Sirk-Flow-Id header is invalid, incomplete, unknown, or does not belong to the supplied directory.", body = ErrorResponse, example = json!({"error": "Missing or invalid X-Sirk-Flow-Id header"})),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = ErrorResponse, example = json!({"error": "Method not allowed"})),
        (status = 500, description = "The Git status operation could not be completed.", body = ErrorResponse, example = json!({"error": "An operation-specific error message."}))
    )
)]
pub(in crate::http) async fn git_status(
    headers: HeaderMap,
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
    let flow_id = match super::super::flow_id::flow_id(&headers) {
        Ok(flow_id) => flow_id,
        Err(response) => return *response,
    };
    if let Err(error) = crate::history::validate_flow(&request.directory, &flow_id) {
        return (
            StatusCode::BAD_REQUEST,
            JsonResponse(ErrorResponse { error }),
        )
            .into_response();
    }
    match tokio::task::spawn_blocking(move || crate::git_service::status(&request.directory)).await
    {
        Ok(Ok(paths)) => JsonResponse(GitStatusResponse { paths }).into_response(),
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse { error }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse {
                error: format!("Git status task failed: {error}"),
            }),
        )
            .into_response(),
    }
}
