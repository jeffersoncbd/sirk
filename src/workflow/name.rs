use super::Step;

impl Step {
    pub fn name(&self) -> &str {
        self.agent
            .as_deref()
            .or(self.tool.as_deref())
            .or(self.custom_tool.as_deref())
            .unwrap_or("invalid")
    }
}
