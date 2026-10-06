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
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
#[utoipa::path(
    post,
    path = "/v1/agent/run",
    operation_id = "runAgent",
    summary = "Run a named agent",
    description = "Loads the agent definition from `.agents/<agent>.md` below the supplied directory and runs it with the literal `input` string. When an agent with `ASK_TOOL: allow` returns an `ask`, the response includes `conversationId`; send it with the user's answer to restore the prior messages. Completed standalone runs return `result` without a `conversationId`. Template-like content in `input`, including `{{ ... }}`, is passed through unchanged. `X-Sirk-Flow-Id` is required and must identify a flow created for the same directory; the complete prompt and each model response are appended to that flow's transcript.",
    params(("X-Sirk-Flow-Id" = String, Header, description = "Required identifier returned by POST /v1/flows for this directory.", example = "flow-18f-1234-0")),
    request_body(
        content = AgentRunRequest,
        description = "Agent execution request.",
        content_type = "application/json",
        examples(
            ("new" = (summary = "Run an agent", value = json!({"directory": "/workspace/project", "agent": "code-explainer", "input": "Explain src/lib.rs."}))),
            ("continue" = (summary = "Answer an agent question", value = json!({"directory": "/workspace/project", "agent": "profile-interviewer", "input": "Ana", "conversationId": "conversation-18f-1234-0"})))
        )
    ),
    responses(
        (status = 200, description = "The agent completed or requested user input.", body = AgentRunResponse, examples(
            ("result" = (summary = "Completed standalone run", value = json!({"result": "The module exposes the public S.I.R.K. API."}))),
            ("ask" = (summary = "Question for the user", value = json!({"conversationId": "conversation-18f-1234-0", "ask": "What is your name?"})))
        )),
        (status = 400, description = "The request body or X-Sirk-Flow-Id header is invalid, incomplete, unknown, or does not belong to the supplied directory.", body = ErrorResponse, example = json!({"error": "Missing or invalid X-Sirk-Flow-Id header"})),
        (status = 405, description = "The endpoint does not accept the HTTP method used.", body = ErrorResponse, example = json!({"error": "Method not allowed"})),
        (status = 500, description = "The requested agent operation could not be completed.", body = ErrorResponse, example = json!({"error": "An operation-specific error message."}))
    )
)]
pub(in crate::http) async fn agent_run(
    headers: HeaderMap,
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
    match tokio::task::spawn_blocking(move || {
        crate::agent_service::run(
            &request.directory,
            request.agent,
            request.input,
            flow_id,
            request.conversation_id,
        )
    })
    .await
    {
        Ok(Ok(crate::agent_service::AgentExecution {
            conversation_id,
            outcome: crate::agent_service::AgentOutcome::Result(result),
        })) => JsonResponse(AgentRunResponse {
            conversation_id,
            result: Some(result),
            ask: None,
        })
        .into_response(),
        Ok(Ok(crate::agent_service::AgentExecution {
            conversation_id,
            outcome: crate::agent_service::AgentOutcome::Ask(ask),
        })) => JsonResponse(AgentRunResponse {
            conversation_id,
            result: None,
            ask: Some(ask),
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
                error: format!("Agent task failed: {error}"),
            }),
        )
            .into_response(),
    }
}
