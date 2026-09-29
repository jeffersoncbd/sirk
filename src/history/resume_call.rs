pub(super) struct ResumeCall {
    pub adapter: String,
    pub calls: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub has_usage: bool,
}
