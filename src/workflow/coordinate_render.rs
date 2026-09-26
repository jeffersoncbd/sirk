use super::{
    EditCoordinate, coordinate_error::coordinate_error, positive_coordinate::positive_coordinate,
    render_scoped,
};
use std::collections::BTreeMap;

impl EditCoordinate {
    pub(super) fn render(
        &self,
        name: &str,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<usize, String> {
        let rendered = match self {
            Self::Number(value) => return positive_coordinate(name, *value),
            Self::Template(template) => render_scoped(template, outputs, locals)?,
        };
        let value = rendered
            .trim()
            .parse::<usize>()
            .map_err(|_| coordinate_error(name))?;
        positive_coordinate(name, value)
    }
}
