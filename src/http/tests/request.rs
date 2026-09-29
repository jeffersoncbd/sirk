use super::super::routes::routes;
use axum::{
    body::Body,
    http::{Request, Response},
};
use tower::ServiceExt;

pub(super) async fn request(method: &str, path: &str, body: String) -> Response<Body> {
    routes()
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap()
}
