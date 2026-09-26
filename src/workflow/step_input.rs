use super::Step;
use std::collections::BTreeMap;

impl Step {
    pub fn render_input(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        self.input.render(outputs, locals)
    }
}
