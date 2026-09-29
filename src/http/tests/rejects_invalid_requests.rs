use super::request::request;
use axum::{body::to_bytes, http::header};

#[tokio::test]
async fn rejects_invalid_requests() {
    assert_eq!(request("GET", "/health", String::new()).await.status(), 200);
    let openapi = request("GET", "/openapi.yaml", String::new()).await;
    assert_eq!(openapi.status(), 200);
    assert_eq!(
        openapi.headers()[header::CONTENT_TYPE],
        "text/plain; charset=utf-8"
    );
    let openapi = to_bytes(openapi.into_body(), usize::MAX).await.unwrap();
    assert_eq!(openapi.as_ref(), include_bytes!("../../../openapi.yaml"));
    let swagger = request("GET", "/swagger", String::new()).await;
    assert_eq!(swagger.status(), 200);
    assert_eq!(
        swagger.headers()[header::CONTENT_TYPE],
        "text/html; charset=utf-8"
    );
    let swagger = to_bytes(swagger.into_body(), usize::MAX).await.unwrap();
    assert!(String::from_utf8_lossy(&swagger).contains("/openapi.yaml"));
    assert_eq!(
        request("GET", "/missing", String::new()).await.status(),
        404
    );
    assert_eq!(
        request("GET", "/v1/agent/run", String::new())
            .await
            .status(),
        405
    );
    assert_eq!(
        request("GET", "/v1/git/status", String::new())
            .await
            .status(),
        405
    );
    assert_eq!(
        request("GET", "/v1/tree", String::new()).await.status(),
        405
    );
    assert_eq!(
        request("POST", "/v1/agent/run", "{}".to_owned())
            .await
            .status(),
        400
    );
    assert_eq!(
        request("POST", "/v1/git/status", "{}".to_owned())
            .await
            .status(),
        400
    );
    assert_eq!(
        request("POST", "/v1/tree", "{}".to_owned()).await.status(),
        400
    );
    assert_eq!(
        request("POST", "/v1/git/add", "{}".to_owned())
            .await
            .status(),
        400
    );
}
