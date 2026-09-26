use super::Step;

impl Step {
    pub fn is_tool_step(&self) -> bool {
        self.tool.is_some() || self.custom_tool.is_some()
    }
}
