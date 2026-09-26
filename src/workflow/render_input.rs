use std::collections::BTreeMap;

pub fn render_input(template: &str, outputs: &BTreeMap<String, String>) -> Result<String, String> {
    super::render_scoped(template, outputs, None)
}
