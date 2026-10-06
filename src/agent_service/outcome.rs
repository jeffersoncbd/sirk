pub(crate) enum AgentOutcome {
    Result(String),
    Ask(String),
}

pub(crate) struct AgentExecution {
    pub conversation_id: String,
    pub outcome: AgentOutcome,
}
