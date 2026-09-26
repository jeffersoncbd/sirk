use super::{StepInput, render_scoped};
use std::collections::BTreeMap;

impl StepInput {
    pub(super) fn render(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        match self {
            Self::Bool(value) => Ok(value.to_string()),
            Self::Text(text) => render_scoped(text, outputs, locals),
            Self::Array(items) => items
                .iter()
                .map(|item| render_scoped(item, outputs, locals))
                .collect::<Result<Vec<_>, _>>()
                .and_then(|items| serde_json::to_string(&items).map_err(|error| error.to_string())),
        }
    }
}
