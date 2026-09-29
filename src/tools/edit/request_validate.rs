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
            Operation::Delete => {
                self.line.is_none()
                    && matches!((self.start, self.end), (Some(s), Some(e)) if s > 0 && e >= s)
            }
            Operation::Replace if self.old_string.is_some() || self.new_string.is_some() => {
                self.line.is_none()
                    && self.start.is_none()
                    && self.end.is_none()
                    && self
                        .old_string
                        .as_ref()
                        .is_some_and(|text| !text.is_empty())
                    && self.new_string.is_some()
                    && self.version.is_none()
            }
            Operation::Replace => {
                self.line.is_none()
                    && matches!((self.start, self.end), (Some(s), Some(e)) if s > 0 && e >= s)
            }
            Operation::Prepend | Operation::Append | Operation::Write => {
                self.line.is_none() && self.start.is_none() && self.end.is_none()
            }
        };
        if !coordinates {
            return Err("EDIT has invalid coordinates for its operation (lines start at 1)".into());
        }
        if self.operation == Operation::Delete && !self.input.is_empty() {
            return Err("EDIT delete does not accept nonempty input".into());
        }
        if self.operation == Operation::Write
            && (self.old_string.is_some() || self.new_string.is_some() || self.replace_all)
        {
            return Err("WRITE only accepts filePath and content".into());
        }
        if self.operation == Operation::Replace
            && self.old_string.is_some()
            && !self.input.is_empty()
        {
            return Err("EDIT string replacement does not accept content".into());
        }
        if self.operation == Operation::Replace
            && self.old_string.is_some()
            && self.old_string == self.new_string
        {
            return Err("EDIT oldString and newString must differ".into());
        }
        if !matches!(
            self.operation,
            Operation::Append | Operation::Prepend | Operation::Write
        ) && !(self.operation == Operation::Replace && self.old_string.is_some())
            && self.version.is_none()
        {
            return Err("EDIT by line requires a SHA-256 version of the current file".into());
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
