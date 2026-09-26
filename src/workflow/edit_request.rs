use super::Step;
use std::collections::BTreeMap;

impl Step {
    pub fn edit_request(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<crate::tools::edit::Request, String> {
        self.edit_request_with(outputs, locals, false)
    }
}
