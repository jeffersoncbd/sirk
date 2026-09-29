use super::super::handle::handle;

#[test]
fn rejects_invalid_requests() {
    assert_eq!(handle("GET", "/health", "").status, 200);
    let openapi = handle("GET", "/openapi.yaml", "");
    assert_eq!(openapi.status, 200);
    assert_eq!(openapi.content_type, "text/plain; charset=utf-8");
    assert_eq!(openapi.body, include_str!("../../../openapi.yaml"));
    let swagger = handle("GET", "/swagger", "");
    assert_eq!(swagger.status, 200);
    assert_eq!(swagger.content_type, "text/html; charset=utf-8");
    assert!(swagger.body.contains("/openapi.yaml"));
    assert_eq!(handle("GET", "/missing", "").status, 404);
    assert_eq!(handle("GET", "/v1/agent/run", "").status, 405);
    assert_eq!(handle("GET", "/v1/git/status", "").status, 405);
    assert_eq!(handle("POST", "/v1/agent/run", "{}").status, 400);
    assert_eq!(handle("POST", "/v1/git/status", "{}").status, 400);
    assert_eq!(handle("POST", "/v1/git/add", "{}").status, 400);
}
