use super::{Operation, Request, version};

impl Request {
    pub fn apply_to(&self, before: &str) -> Result<String, String> {
        self.validate()?;
        if self
            .version
            .as_ref()
            .is_some_and(|v| !v.eq_ignore_ascii_case(&version(before)))
        {
            return Err("EDIT version conflict; READ the current file before editing".into());
        }
        let lines: Vec<&str> = before.split_inclusive('\n').collect();
        let offset = |line: usize| lines[..line].iter().map(|s| s.len()).sum::<usize>();
        let (start, end) = match self.operation {
            Operation::Append => (before.len(), before.len()),
            Operation::Prepend => (0, 0),
            Operation::Insert => {
                let line = self.line.unwrap();
                if line - 1 > lines.len() {
                    return Err("EDIT insert line is beyond EOF".into());
                }
                (offset(line - 1), offset(line - 1))
            }
            Operation::Delete | Operation::Replace => {
                let (start, end) = (self.start.unwrap(), self.end.unwrap());
                if end > lines.len() {
                    return Err("EDIT range is beyond EOF".into());
                }
                (offset(start - 1), offset(end))
            }
        };
        Ok(format!(
            "{}{}{}",
            &before[..start],
            self.input,
            &before[end..]
        ))
    }
}
