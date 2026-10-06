use super::super::Agent;

#[test]
fn reads_metadata_and_markdown_with_crlf() {
    let agent = Agent::parse(
        "reviewer",
        "---\r\nadapter: codex\r\nmodel: custom\r\n---\r\n# Review\r\n\r\nBe careful.\r\n",
    )
    .unwrap();
    assert_eq!(agent.id, "reviewer");
    assert_eq!(agent.instructions, "# Review\n\nBe careful.");
    assert_eq!(agent.model.as_deref(), Some("custom"));
    assert!(agent.call_prefix.is_empty());
    assert!(!agent.tree_tool);
    assert!(!agent.json);
    assert!(!agent.ask_tool);
    assert!(!agent.edit_tool);
    assert!(!agent.delete_tool);
}
