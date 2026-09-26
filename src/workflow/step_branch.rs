use super::Step;

impl Step {
    pub fn branch(&self, selected: bool) -> (&str, &[Step]) {
        if selected {
            ("true", &self.is_true)
        } else {
            ("false", &self.is_false)
        }
    }
}
