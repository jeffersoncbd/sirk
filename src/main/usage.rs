pub(super) fn usage() -> &'static str {
    "Usage:\n  new-harness --newAgent\n  new-harness run <flow-name>\n  new-harness resume <history.log>\n\nFlows resolve to flows/<flow-name>.yml.\n\n--newAgent (alias --new-agent) creates an agent interactively. During a question, /cancel cancels input. Workflow history can be resumed.\n"
}
