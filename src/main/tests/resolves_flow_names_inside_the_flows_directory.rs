use super::*;

#[test]
fn resolves_flow_names_inside_the_flows_directory() {
    assert_eq!(
        workflow_path("documentation").unwrap(),
        Path::new("flows/documentation.yml")
    );
    assert_eq!(
        workflow_path("release_notes-2").unwrap(),
        Path::new("flows/release_notes-2.yml")
    );
}
