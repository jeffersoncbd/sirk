use super::{run::run, usage::usage, workflow_path::workflow_path};
use std::path::Path;

#[test]
fn rejects_an_invalid_command() {
    assert!(
        run(vec!["invalid".to_owned()])
            .unwrap_err()
            .contains("invalid command")
    );
}

#[test]
fn usage_lists_explicit_flow_commands() {
    let text = usage();
    assert!(text.contains("new-harness run <flow-name>"));
    assert!(text.contains("Flows resolve to flows/<flow-name>.yml."));
    assert!(!text.contains("Interactive commands:"));
    assert!(!text.contains("<flow-name>      Start a flow directly"));
}

#[test]
fn accepts_no_arguments_without_starting_a_prompt() {
    run(vec![]).unwrap();
}

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

#[test]
fn rejects_flow_paths_and_extensions() {
    for invalid in ["", "documentation.yml", "../documentation", "nested/flow"] {
        assert!(workflow_path(invalid).is_err(), "{invalid:?}");
    }
}
