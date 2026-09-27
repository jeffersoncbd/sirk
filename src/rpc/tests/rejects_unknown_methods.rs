use super::super::{handle::handle, types::Request};
use serde_json::json;

#[test]
fn rejects_unknown_methods() {
    let response = handle(
        Request {
            jsonrpc: "2.0".to_owned(),
            id: Some(json!(7)),
            method: "tools.read".to_owned(),
            params: json!({}),
        },
        std::path::Path::new("."),
    );
    let error = response.error.unwrap();
    assert_eq!(error.code, -32601);
    assert_eq!(error.message, "Method not found");
}
