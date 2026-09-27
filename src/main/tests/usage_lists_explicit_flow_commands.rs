use super::*;

#[test]
fn usage_lists_explicit_flow_commands() {
    let text = usage();
    assert!(text.starts_with("S.I.R.K."));
    assert!(text.contains("sirk run <flow-name>"));
    assert!(text.contains("sirk rpc"));
    assert!(text.contains("Flows resolve to flows/<flow-name>.yml."));
    assert!(!text.contains("Interactive commands:"));
    assert!(!text.contains("<flow-name>      Start a flow directly"));
}
