use super::super::Agent;

#[test]
fn enables_ask_tool_only_when_explicitly_allowed() {
    let agent = Agent::parse(
        "interviewer",
        "---\nadapter: codex\nASK_TOOL: allow\n---\nAsk one question.",
    )
    .unwrap();
    assert!(agent.ask_tool);
    assert!(
        serde_yaml::to_string(&agent)
            .unwrap()
            .contains("ASK_TOOL: true")
    );
}
