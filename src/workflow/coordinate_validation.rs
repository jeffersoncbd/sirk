use super::{EditCoordinate, render_scoped};
use std::collections::BTreeMap;

impl EditCoordinate {
    pub(super) fn render_for_validation(
        &self,
        name: &str,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<usize, String> {
        if let Self::Template(template) = self
            && template.contains("{{")
        {
            render_scoped(template, outputs, locals)?;
            return Ok(if name == "end" { usize::MAX } else { 1 });
        }
        self.render(name, outputs, locals)
    }
}
