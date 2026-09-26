use super::Step;

impl Step {
    pub fn force(&self) -> bool {
        self.force.unwrap_or(false)
    }
}
