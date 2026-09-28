use super::super::handle::handle;

#[test]
fn rejects_invalid_requests() {
    assert_eq!(handle("GET", "/health", "").status, 200);
    assert_eq!(handle("GET", "/missing", "").status, 404);
    assert_eq!(handle("GET", "/v1/agent/run", "").status, 405);
    assert_eq!(handle("POST", "/v1/agent/run", "{}").status, 400);
}
