use super::{Operation, Pending};

impl Pending {
    pub fn validate(&self) -> Result<(), String> {
        self.request.validate()?;
        if self.was_missing
            && (self.before.as_deref() != Some("")
                || !matches!(
                    self.request.operation,
                    Operation::Append | Operation::Prepend
                ))
        {
            return Err("invalid EDIT creation record".into());
        }
        if let Some(before) = &self.before {
            self.request.apply_to(before)?;
        }
        Ok(())
    }
}
