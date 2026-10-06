use super::super::Agent;

#[test]
fn enables_read_tool_when_explicitly_allowed() {
    let agent = Agent::parse(
        "reader",
        "---\nadapter: codex\nREAD_TOOL: allow\n---\nRead only what is needed.",
    )
    .unwrap();
    assert!(agent.read_tool);
    assert!(
        serde_yaml::to_string(&agent)
            .unwrap()
            .contains("READ_TOOL: true")
    );
}
