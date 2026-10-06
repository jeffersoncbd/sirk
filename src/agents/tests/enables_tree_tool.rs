use super::super::Agent;

#[test]
fn enables_tree_only_when_explicitly_allowed() {
    let agent = Agent::parse(
        "explorer",
        "---\nadapter: codex\nTREE_TOOL: allow\n---\nInspect the project.",
    )
    .unwrap();
    assert!(agent.tree_tool);
    assert!(
        serde_yaml::to_string(&agent)
            .unwrap()
            .contains("TREE_TOOL: true")
    );
}
