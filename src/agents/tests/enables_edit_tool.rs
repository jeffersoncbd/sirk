use super::super::Agent;

#[test]
fn enables_the_external_edit_tool_only_when_allowed() {
    let agent = Agent::parse(
        "editor",
        "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate documentation.",
    )
    .unwrap();
    assert!(agent.edit_tool);
    assert!(
        serde_yaml::to_string(&agent)
            .unwrap()
            .contains("EDIT_TOOL: true")
    );
}
