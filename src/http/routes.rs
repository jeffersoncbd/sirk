use super::controllers::{
    agent_run, git_add, git_status, health, method_not_allowed, not_found, openapi, swagger, tree,
};
use super::spec::ApiDoc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

pub(super) fn routes() -> (Router, utoipa::openapi::OpenApi) {
    let mut document = ApiDoc::openapi();
    document.info.license = None;
    let (router, document) = OpenApiRouter::with_openapi(document)
        .routes(utoipa_axum::routes!(health::health)) // GET /health
        .routes(utoipa_axum::routes!(openapi::openapi)) // GET /openapi.yaml
        .routes(utoipa_axum::routes!(swagger::swagger)) // GET /swagger
        .routes(utoipa_axum::routes!(agent_run::agent_run)) // POST /v1/agent/run
        .routes(utoipa_axum::routes!(tree::tree)) // POST /v1/tree
        .routes(utoipa_axum::routes!(git_status::git_status)) // POST /v1/git/status
        .routes(utoipa_axum::routes!(git_add::git_add)) // POST /v1/git/add
        .split_for_parts();
    let router = router
        .fallback(not_found::not_found)
        .method_not_allowed_fallback(method_not_allowed::method_not_allowed);
    (router, document)
}
