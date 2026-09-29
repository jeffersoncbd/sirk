use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "S.I.R.K. HTTP API",
        version = "0.1.0",
        description = "HTTP interface for the S.I.R.K. coding-agent service. The service has no authentication or TLS. Bind it only to a trusted network. Paths supplied in `directory` identify directories as seen by the host running S.I.R.K.; they are not resolved relative to the client. Requests to an unknown path return `404` with a JSON error. A method other than the documented method returns `405` with a JSON error."
    ),
    servers((
        url = "http://127.0.0.1:8080",
        description = "Default address used by `sirk http`."
    ))
)]
pub(in crate::http) struct ApiDoc;

pub fn document() -> utoipa::openapi::OpenApi {
    super::routes::routes().1
}
