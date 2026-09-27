pub(super) fn usage() -> &'static str {
    "S.I.R.K. — coding-agent workflows\n\nUsage:\n  sirk --newAgent\n  sirk run <flow-name>\n  sirk resume <history.log>\n\nFlows resolve to flows/<flow-name>.yml.\n\n--newAgent (alias --new-agent) creates an agent interactively. During a question, /cancel cancels input. Workflow history can be resumed.\n"
}
