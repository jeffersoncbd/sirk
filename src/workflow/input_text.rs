use super::StepInput;

impl StepInput {
    pub(super) fn text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            Self::Array(_) | Self::Bool(_) => None,
        }
    }
}
