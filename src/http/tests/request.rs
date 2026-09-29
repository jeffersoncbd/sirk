use super::super::routes::routes;
use axum::{
    body::Body,
    http::{Request, Response, header},
};
use tower::ServiceExt;

pub(super) async fn request(
    method: &str,
    path: &str,
    body: String,
    flow_id: Option<&str>,
) -> Response<Body> {
    let mut request = Request::builder()
        .method(method)
        .uri(path)
        .header(header::CONTENT_TYPE, "application/json");
    if let Some(flow_id) = flow_id {
        request = request.header("X-Sirk-Flow-Id", flow_id);
    }
    routes()
        .0
        .oneshot(request.body(Body::from(body)).unwrap())
        .await
        .unwrap()
}
