use super::{EditCoordinate, Step, render_scoped};
use std::collections::BTreeMap;

impl Step {
    pub(super) fn edit_request_with(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
        validating: bool,
    ) -> Result<crate::tools::edit::Request, String> {
        let coordinate = |value: Option<&EditCoordinate>, name: &str| {
            value
                .map(|value| {
                    if validating {
                        value.render_for_validation(name, outputs, locals)
                    } else {
                        value.render(name, outputs, locals)
                    }
                })
                .transpose()
        };
        Ok(crate::tools::edit::Request {
            path: self.render_path(outputs, locals)?,
            operation: self.operation.ok_or("EDIT requires operation")?,
            line: coordinate(self.line.as_ref(), "line")?,
            start: coordinate(self.start.as_ref(), "start")?,
            end: coordinate(self.end.as_ref(), "end")?,
            version: self
                .version
                .as_deref()
                .map(|v| render_scoped(v, outputs, locals))
                .transpose()?,
            input: self.render_input(outputs, locals)?,
        })
    }
}
