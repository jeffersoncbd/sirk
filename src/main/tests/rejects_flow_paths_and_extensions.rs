use super::*;

#[test]
fn rejects_flow_paths_and_extensions() {
    for invalid in ["", "documentation.yml", "../documentation", "nested/flow"] {
        assert!(workflow_path(invalid).is_err(), "{invalid:?}");
    }
}
