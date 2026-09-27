pub(super) fn usage() -> &'static str {
    "S.I.R.K. — coding-agent workflows\n\nUsage:\n  sirk --newAgent\n  sirk run <flow-name>\n  sirk resume <history.log>\n  sirk rpc\n\nFlows resolve to flows/<flow-name>.yml.\nThe rpc command serves line-delimited JSON-RPC 2.0 for language SDKs.\n\n--newAgent (alias --new-agent) creates an agent interactively. During a question, /cancel cancels input. Workflow history can be resumed.\n"
}
