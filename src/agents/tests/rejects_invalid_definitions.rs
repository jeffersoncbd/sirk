use super::super::Agent;
use std::path::Path;

#[test]
fn rejects_malformed_definitions_and_paths() {
    for source in [
        "No header",
        "---\nadapter: codex",
        "---\nadapter: codex\n---",
        "---\nadapter: codex\nunknown: true\n---\nReview",
        "---\nadapter: codex\nwrite: true\n---\nReview",
        "---\nadapter: codex\njson: true\n---\nReview",
        "---\nadapter: codex\nASK_TOOL: deny\n---\nReview",
        "---\nadapter: codex\nREAD_TOOL: deny\n---\nReview",
        "---\nadapter: codex\nEDIT_TOOL: deny\n---\nReview",
        "---\nadapter: codex\nDELETE_TOOL: deny\n---\nReview",
        "---\nadapter: codex\nDELETE_WITHOUT_CONFIRM: allow\n---\nReview",
        "---\nadapter: codex\nTREE_TOOL: deny\n---\nReview",
        "---\nadapter: codex\ncall_prefix: [docker, '']\n---\nReview",
    ] {
        assert!(Agent::parse("reviewer", source).is_err());
    }
    for id in ["", "../outside", "/tmp/agent", "agent.md"] {
        assert!(Agent::load(Path::new(".agents"), id).is_err());
    }
}
