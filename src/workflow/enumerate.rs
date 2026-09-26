use super::Step;

impl Step {
    pub fn enumerate(&self) -> bool {
        self.enumerate.unwrap_or(false)
    }
}
