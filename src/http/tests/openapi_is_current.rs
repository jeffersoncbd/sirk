use super::super::openapi_document;

#[test]
fn openapi_is_current() {
    let generated = openapi_document().to_yaml().unwrap();
    assert_eq!(generated, include_str!("../../../openapi.yaml"));
}
