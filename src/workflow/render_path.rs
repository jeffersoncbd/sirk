use super::{Step, render_scoped};
use std::collections::BTreeMap;

impl Step {
    pub fn render_path(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        let path = self.path.as_deref().ok_or("tool requires a path")?;
        render_scoped(path, outputs, locals)
    }
}
