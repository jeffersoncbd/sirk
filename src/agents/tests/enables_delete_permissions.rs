use super::super::Agent;

#[test]
fn enables_delete_permissions_only_when_explicitly_allowed() {
    let agent = Agent::parse(
        "cleaner",
        "---\nadapter: codex\nDELETE_TOOL: allow\nDELETE_WITHOUT_CONFIRM: allow\n---\nClean generated files.",
    )
    .unwrap();
    assert!(agent.delete_tool);
    assert!(agent.delete_without_confirm);
}
