use super::super::Agent;

#[test]
fn normalizes_models_from_markdown_and_saved_snapshots() {
    let agent = Agent::parse(
        "planner",
        "---\nadapter: codex\nmodel: ' GPT-6-Astra '\n---\nPlan.",
    )
    .unwrap();
    assert_eq!(agent.model.as_deref(), Some("gpt-6-astra"));
    let mut snapshot = serde_yaml::to_string(&agent).unwrap();
    snapshot = snapshot.replace("gpt-6-astra", "GPT-6-Astra");
    let restored: Agent = serde_yaml::from_str(&snapshot).unwrap();
    assert_eq!(restored.model.as_deref(), Some("gpt-6-astra"));
}
