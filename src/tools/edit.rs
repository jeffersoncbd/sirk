//! Workflow-only, version-checked edits with durable preparation and plain diffs.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{IsTerminal, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Insert,
    Delete,
    Replace,
    Prepend,
    Append,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub path: String,
    pub operation: Operation,
    pub line: Option<usize>,
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub version: Option<String>,
    pub input: String,
}

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

pub fn version(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}

/// Stored in the existing INPUT block. Preparation is committed before mutation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pending {
    pub request: Request,
    pub before: Option<String>,
    #[serde(default)]
    pub was_missing: bool,
}

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

    pub fn prepare(&mut self, directory: &Path) -> Result<(), String> {
        let allow_missing = matches!(
            self.request.operation,
            Operation::Append | Operation::Prepend
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

    pub fn diff(&self) -> Result<String, String> {
        let before = self.before.as_deref().ok_or("EDIT is not prepared")?;
        let after = self.request.apply_to(before)?;
        // Escape control characters in paths; file contents remain exact in the diff.
        let label = format!("{:?}", self.request.path);
        Ok(similar::TextDiff::from_lines(before, &after)
            .unified_diff()
            .context_radius(3)
            .header(
                if self.was_missing {
                    "/dev/null"
                } else {
                    &label
                },
                &label,
            )
            .to_string())
    }

    pub fn commit(&self, directory: &Path) -> Result<String, String> {
        self.validate()?;
        let before = self.before.as_deref().ok_or("EDIT is not prepared")?;
        let after = self.request.apply_to(before)?;
        let target = target(directory, &self.request.path, self.was_missing)?;
        let current = read_optional(&target)?;
        if current.as_deref() == Some(&after) {
            File::open(&target)
                .and_then(|f| f.sync_all())
                .map_err(|e| e.to_string())?;
            sync_parent(&target)?;
            return self.diff();
        }
        let expected = if self.was_missing { None } else { Some(before) };
        if current.as_deref() != expected {
            return Err(
                "EDIT conflict: file differs from both prepared and resulting content".into(),
            );
        }
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (temporary, mut file) = loop {
            let name = format!(
                ".new-harness-edit-{}-{}.tmp",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            );
            let temporary = target.parent().unwrap().join(name);
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)
            {
                Ok(file) => break (temporary, file),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => return Err(e.to_string()),
            }
        };
        let result = (|| -> Result<(), String> {
            if !self.was_missing {
                file.set_permissions(
                    fs::metadata(&target)
                        .map_err(|e| e.to_string())?
                        .permissions(),
                )
                .map_err(|e| e.to_string())?;
            }
            file.write_all(after.as_bytes())
                .and_then(|_| file.sync_all())
                .map_err(|e| e.to_string())?;
            // Detect ordinary intervening edits immediately before replacement.
            if target != self::target(directory, &self.request.path, self.was_missing)?
                || read_optional(&target)?.as_deref() != expected
            {
                return Err("EDIT conflict: file changed during preparation".into());
            }
            if self.was_missing {
                // Publish a complete file atomically without replacing a concurrently created target.
                fs::hard_link(&temporary, &target)
                    .map_err(|e| format!("EDIT cannot create target without overwriting: {e}"))?;
                fs::remove_file(&temporary).map_err(|e| e.to_string())?;
            } else {
                fs::rename(&temporary, &target).map_err(|e| e.to_string())?;
            }
            sync_parent(&target)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result?;
        self.diff()
    }
}

fn sync_parent(path: &Path) -> Result<(), String> {
    File::open(path.parent().unwrap())
        .and_then(|f| f.sync_all())
        .map_err(|e| e.to_string())
}

fn read_optional(path: &Path) -> Result<Option<String>, String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("EDIT cannot read UTF-8: {e}")),
    }
}

fn target(directory: &Path, path: &str, allow_missing: bool) -> Result<PathBuf, String> {
    let root = directory.canonicalize().map_err(|e| e.to_string())?;
    let requested = root.join(path);
    let metadata = match fs::symlink_metadata(&requested) {
        Ok(metadata) => metadata,
        Err(e) if allow_missing && e.kind() == std::io::ErrorKind::NotFound => {
            let parent = requested
                .parent()
                .ok_or("EDIT requires a parent directory")?
                .canonicalize()
                .map_err(|e| format!("EDIT parent directory must exist: {e}"))?;
            if !parent.starts_with(&root) {
                return Err("EDIT path must stay inside the execution directory".into());
            }
            return Ok(parent.join(requested.file_name().ok_or("EDIT requires a file name")?));
        }
        Err(e) => return Err(format!("EDIT cannot inspect path: {e}")),
    };
    if !metadata.file_type().is_file() {
        return Err("EDIT requires an existing regular file, not a symlink".into());
    }
    let resolved = requested.canonicalize().map_err(|e| e.to_string())?;
    if !resolved.starts_with(root) {
        return Err("EDIT path must stay inside the execution directory".into());
    }
    Ok(resolved)
}

pub fn display(diff: &str) {
    let color = std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    print!("{}", render(diff, color));
}

fn render(diff: &str, color: bool) -> String {
    let mut rendered = String::new();
    let mut in_hunk = false;
    for line in diff.split_inclusive('\n') {
        if line.starts_with("@@") {
            in_hunk = true;
        }
        let code = if in_hunk && line.starts_with('+') {
            "42"
        } else if in_hunk && line.starts_with('-') {
            "41"
        } else {
            ""
        };
        if color && !code.is_empty() {
            rendered.push_str(&format!("\x1b[{code}m"));
        }
        // Never execute control sequences originating in source files.
        for c in line.trim_end_matches('\n').chars() {
            if c.is_control() && c != '\n' && c != '\t' {
                rendered.extend(c.escape_default());
            } else {
                rendered.push(c);
            }
        }
        if color && !code.is_empty() {
            rendered.push_str("\x1b[0m");
        }
        if line.ends_with('\n') {
            rendered.push('\n');
        }
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(operation: Operation, before: &str, input: &str) -> Request {
        Request {
            path: "file.txt".into(),
            operation,
            line: None,
            start: None,
            end: None,
            version: Some(version(before)),
            input: input.into(),
        }
    }

    #[test]
    fn line_edits_preserve_bytes_and_handle_eof() {
        let before = "á\r\n}\r\nlast";
        let mut edit = request(Operation::Insert, before, "new\n");
        edit.line = Some(2);
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\nnew\n}\r\nlast");
        edit.line = Some(4);
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\n}\r\nlastnew\n");
        edit.line = Some(5);
        assert!(edit.apply_to(before).is_err());
        edit = request(Operation::Delete, before, "");
        edit.start = Some(2);
        edit.end = Some(2);
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\nlast");
        edit.operation = Operation::Replace;
        edit.end = Some(3);
        edit.input = "replacement".into();
        assert_eq!(edit.apply_to(before).unwrap(), "á\r\nreplacement");
        edit = request(Operation::Insert, "", "first");
        edit.line = Some(1);
        assert_eq!(edit.apply_to("").unwrap(), "first");
        edit = request(Operation::Insert, "a\n", "b\n");
        edit.line = Some(2);
        assert_eq!(edit.apply_to("a\n").unwrap(), "a\nb\n");
    }

    #[test]
    fn append_and_prepend_are_exact_and_versions_are_checked() {
        for (operation, expected) in [
            (Operation::Append, "oldnew"),
            (Operation::Prepend, "newold"),
        ] {
            let mut edit = request(operation, "old", "new");
            edit.version = None;
            assert_eq!(edit.apply_to("old").unwrap(), expected);
            assert_eq!(edit.apply_to("").unwrap(), "new");
        }
        let mut edit = request(Operation::Delete, "old", "");
        edit.start = Some(1);
        edit.end = Some(1);
        assert!(
            edit.apply_to("changed")
                .unwrap_err()
                .contains("version conflict")
        );
        assert_eq!(edit.apply_to("old").unwrap(), "");
        edit.version = None;
        assert!(edit.apply_to("old").is_err());
    }

    #[test]
    fn diff_and_color_rendering_preserve_plain_results() {
        let mut request = request(Operation::Replace, "a\nold\n", "new\n");
        request.start = Some(2);
        request.end = Some(2);
        let pending = Pending {
            request,
            before: Some("a\nold\n".into()),
            was_missing: false,
        };
        let diff = pending.diff().unwrap();
        assert!(diff.contains("-old\n+new\n"));
        assert_eq!(render(&diff, false), diff);
        assert!(render(&diff, true).contains("\x1b[41m-old\x1b[0m\n\x1b[42m+new\x1b[0m"));
        assert!(!render("\x1b[2J", false).contains('\x1b'));
        let mut pending = pending;
        pending.request.input = "new".into();
        assert!(
            pending
                .diff()
                .unwrap()
                .contains("No newline at end of file")
        );
    }
}
