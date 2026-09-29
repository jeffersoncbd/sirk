use super::super::{
    json_response::JsonResponse,
    schemas::{
        requests::AgentRunRequest,
        responses::{AgentRunResponse, ErrorResponse},
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
    path = "/v1/agent/run",
    operation_id = "runAgent",
    summary = "Run a named agent",
    description = "Loads the agent definition from `.agents/<agent>.md` below the supplied directory and runs it with the literal `input` string. Template-like content in `input`, including `{{ ... }}`, is passed through unchanged. The result is the agent's final response after any enabled tool requests have been handled.",
    request_body(
        content = AgentRunRequest,
        description = "Agent execution request.",
        content_type = "application/json",
        example = json!({"directory": "/workspace/project", "agent": "code-explainer", "input": "Explain src/lib.rs."})
    ),
    responses(
        (status = 200, description = "The agent completed successfully.", body = AgentRunResponse, example = json!({"result": "The module exposes the public S.I.R.K. API."})),
        (status = 400, description = "The request body is invalid, incomplete, or contains an unknown field.", body = ErrorResponse, example = json!({"error": "Invalid request"})),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = ErrorResponse, example = json!({"error": "Method not allowed"})),
        (status = 500, description = "The requested agent operation could not be completed.", body = ErrorResponse, example = json!({"error": "An operation-specific error message."}))
    )
)]
pub(in crate::http) async fn agent_run(
    payload: Result<Json<AgentRunRequest>, JsonRejection>,
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
    match tokio::task::spawn_blocking(move || {
        crate::agent_service::run(&request.directory, request.agent, request.input)
    })
    .await
    {
        Ok(Ok(result)) => JsonResponse(AgentRunResponse { result }).into_response(),
        Ok(Err(error)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse { error }),
        )
            .into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            JsonResponse(ErrorResponse {
                error: format!("Agent task failed: {error}"),
            }),
        )
            .into_response(),
    }
}
