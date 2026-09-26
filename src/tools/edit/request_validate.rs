use super::{Operation, Request};

impl Request {
    pub fn validate(&self) -> Result<(), String> {
        if self.path.trim().is_empty() {
            return Err("EDIT requires a path".into());
        }
        let coordinates = match self.operation {
            Operation::Insert => {
                self.line.is_some_and(|n| n > 0) && self.start.is_none() && self.end.is_none()
            }
            Operation::Delete | Operation::Replace => {
                self.line.is_none()
                    && matches!((self.start, self.end), (Some(s), Some(e)) if s > 0 && e >= s)
            }
            Operation::Prepend | Operation::Append => {
                self.line.is_none() && self.start.is_none() && self.end.is_none()
            }
        };
        if !coordinates {
            return Err("EDIT has invalid coordinates for its operation (lines start at 1)".into());
        }
        if self.operation == Operation::Delete && !self.input.is_empty() {
            return Err("EDIT delete does not accept nonempty input".into());
        }
        if !matches!(self.operation, Operation::Append | Operation::Prepend)
            && self.version.is_none()
        {
            return Err("EDIT by line requires version from a previous READ version-output".into());
        }
        if self
            .version
            .as_ref()
            .is_some_and(|v| v.len() != 64 || !v.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            return Err("EDIT version must be a SHA-256 hex digest".into());
        }
        Ok(())
    }
}
