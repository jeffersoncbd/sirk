use super::super::routes::routes;
use axum::{
    body::Body,
    http::{Request, Response, header},
};
use tower::ServiceExt;

pub(super) async fn request(method: &str, path: &str, body: String) -> Response<Body> {
    routes()
        .0
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap()
}
