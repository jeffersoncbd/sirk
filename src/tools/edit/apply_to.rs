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
        if self.operation == Operation::Write {
            return Ok(self.input.clone());
        }
        if let (Operation::Replace, Some(old_string), Some(new_string)) = (
            self.operation,
            self.old_string.as_ref(),
            self.new_string.as_ref(),
        ) {
            let ending = if before.contains("\r\n") {
                "\r\n"
            } else {
                "\n"
            };
            let old = old_string.replace("\r\n", "\n").replace('\n', ending);
            let new = new_string.replace("\r\n", "\n").replace('\n', ending);
            let matches = before.match_indices(&old).count();
            if matches == 0 {
                return Err("EDIT oldString was not found in the file".into());
            }
            if matches > 1 && !self.replace_all {
                return Err("EDIT found multiple matches for oldString; provide more surrounding text or set replaceAll to true".into());
            }
            return Ok(if self.replace_all {
                before.replace(&old, &new)
            } else {
                before.replacen(&old, &new, 1)
            });
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
            Operation::Write => unreachable!("WRITE returns before line processing"),
        };
        Ok(format!(
            "{}{}{}",
            &before[..start],
            self.input,
            &before[end..]
        ))
    }
}
