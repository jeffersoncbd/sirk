use super::controllers::{
    agent_run, git_add, git_status, health, method_not_allowed, not_found, openapi, swagger,
};
use axum::{
    Router,
    routing::{MethodFilter, on},
};

pub(super) fn routes() -> Router {
    Router::new()
        .route("/health", on(MethodFilter::GET, health::health))
        .route("/openapi.yaml", on(MethodFilter::GET, openapi::openapi))
        .route("/swagger", on(MethodFilter::GET, swagger::swagger))
        .route(
            "/v1/agent/run",
            on(MethodFilter::POST, agent_run::agent_run),
        )
        .route(
            "/v1/git/status",
            on(MethodFilter::POST, git_status::git_status),
        )
        .route("/v1/git/add", on(MethodFilter::POST, git_add::git_add))
        .fallback(not_found::not_found)
        .method_not_allowed_fallback(method_not_allowed::method_not_allowed)
}
