use super::{Operation, Pending, read_optional::read_optional, target::target};
use std::path::Path;

impl Pending {
    pub fn prepare(&mut self, directory: &Path) -> Result<(), String> {
        let allow_missing = matches!(
            self.request.operation,
            Operation::Append | Operation::Prepend | Operation::Write
        );
        let target = target(directory, &self.request.path, allow_missing)?;
        let content = read_optional(&target)?;
        let before = content.clone().unwrap_or_default();
        if content.is_none() && !allow_missing {
            return Err("EDIT requires an existing file for line operations".into());
        }
        self.request.apply_to(&before)?;
        self.was_missing = content.is_none();
        self.before = Some(before);
        Ok(())
    }
}
