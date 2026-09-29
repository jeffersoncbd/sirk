use super::super::{
    json_response::JsonResponse,
    schemas::{
        requests::DirectoryRequest,
        responses::{ErrorResponse, TreeResponse},
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
    path = "/v1/tree",
    operation_id = "tree",
    summary = "List files below a directory",
    description = "Returns sorted, unique relative UTF-8 paths for tracked and non-ignored untracked files below `directory`. Paths hidden by `.treeignore` are excluded. The requested directory must exist and be within a Git working tree visible to the server. `X-Sirk-Flow-Id` is required and must identify a flow created for the same directory.",
    params(("X-Sirk-Flow-Id" = String, Header, description = "Required identifier returned by POST /v1/flows for this directory.", example = "flow-18f-1234-0")),
    request_body(
        content = DirectoryRequest,
        description = "Server-visible Git directory.",
        content_type = "application/json",
        example = json!({"directory": "/workspace/project"})
    ),
    responses(
        (status = 200, description = "Files were listed successfully.", body = TreeResponse, example = json!({"paths": ["README.md", "src/lib.rs"]})),
        (status = 400, description = "The request body or X-Sirk-Flow-Id header is invalid, incomplete, unknown, or does not belong to the supplied directory.", body = ErrorResponse, example = json!({"error": "Missing or invalid X-Sirk-Flow-Id header"})),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = ErrorResponse, example = json!({"error": "Method not allowed"})),
        (status = 500, description = "The TREE operation could not be completed.", body = ErrorResponse, example = json!({"error": "An operation-specific error message."}))
    )
)]
pub(in crate::http) async fn tree(
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
    match tokio::task::spawn_blocking(move || crate::tools::tree::Tree::list(&request.directory))
        .await
    {
        Ok(Ok(tree)) => {
            let paths = tree
                .files
                .into_iter()
                .map(|path| {
                    path.into_os_string().into_string().map_err(|path| {
                        format!(
                            "TREE cannot represent a non-UTF-8 path in a JSON array: {}",
                            path.to_string_lossy()
                        )
                    })
                })
                .collect::<Result<_, _>>();
            match paths {
                Ok(paths) => JsonResponse(TreeResponse { paths }).into_response(),
                Err(error) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    JsonResponse(ErrorResponse { error }),
                )
                    .into_response(),
            }
        }
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse { error }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse {
                error: format!("TREE task failed: {error}"),
            }),
        )
            .into_response(),
    }
}
