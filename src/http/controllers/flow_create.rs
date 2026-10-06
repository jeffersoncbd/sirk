use super::super::{
    json_response::JsonResponse,
    schemas::{
        requests::DirectoryRequest,
        responses::{ErrorResponse, FlowResponse},
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
    path = "/v1/flows",
    operation_id = "createFlow",
    summary = "Create a flow transcript",
    description = "Creates `history/<flow-id>/` in `directory` with an empty `flow.log`, `usage.log`, and `conversations/` directory, then returns the flow identifier. Include the returned value as the required `X-Sirk-Flow-Id` header on every later agent, TREE, and Git request for this execution. Agent calls append complete prompts and model responses to `flow.log`, aggregate provider usage in `usage.log`, and persist resumable user, assistant, and tool messages under `conversations/`.",
    request_body(
        content = DirectoryRequest,
        description = "Server-visible directory where the flow transcript is stored.",
        content_type = "application/json",
        example = json!({"directory": "/workspace/project"})
    ),
    responses(
        (status = 201, description = "The flow transcript was created.", body = FlowResponse, example = json!({"flowId": "flow-18f-1234-0"})),
        (status = 400, description = "The request body is invalid, incomplete, or contains an unknown field.", body = ErrorResponse, example = json!({"error": "Invalid request"})),
        (status = 500, description = "The flow transcript could not be created.", body = ErrorResponse, example = json!({"error": "An operation-specific error message."}))
    )
)]
pub(in crate::http) async fn flow_create(
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
    match tokio::task::spawn_blocking(move || crate::history::new_flow(&request.directory)).await {
        Ok(Ok(flow_id)) => {
            (StatusCode::CREATED, JsonResponse(FlowResponse { flow_id })).into_response()
        }
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse { error }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse {
                error: format!("Flow task failed: {error}"),
            }),
        )
            .into_response(),
    }
}
