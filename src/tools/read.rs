//! READ: return the UTF-8 contents of a file inside the execution directory.
use std::{fs, io, path::Path};

pub fn read(directory: &Path, path: &str) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("READ requires a file path".into());
    }
    let root = directory
        .canonicalize()
        .map_err(|e| format!("READ cannot resolve directory: {e}"))?;
    let file = root
        .join(path)
        .canonicalize()
        .map_err(|e| format!("READ cannot resolve `{path}`: {e}"))?;
    if !file.starts_with(&root) {
        return Err("READ path must be inside the execution directory".into());
    }
    if !file.is_file() {
        return Err(format!("READ path is not a regular file: {path}"));
    }
    if read_ignored(&root, &file)? {
        return Err("AccessDenied".into());
    }
    fs::read_to_string(&file).map_err(|e| format!("READ cannot read `{path}` as UTF-8: {e}"))
}

/// Presents exact file content with stable, one-based line coordinates.
///
/// The header makes the prefix self-describing while each source line, including
/// its original line ending, remains after its `N | ` prefix.
pub fn enumerate(content: &str) -> String {
    let mut numbered = String::from("Line | Content\n");
    for (index, line) in content.split_inclusive('\n').enumerate() {
        numbered.push_str(&(index + 1).to_string());
        numbered.push_str(" | ");
        numbered.push_str(line);
    }
    numbered
}

/// Recovers exact source content from `enumerate` output saved in a transcript.
pub fn enumerated_content(numbered: &str) -> Result<String, String> {
    let body = numbered
        .strip_prefix("Line | Content\n")
        .ok_or("invalid enumerated READ result")?;
    let mut content = String::new();
    let mut expected = 1usize;
    let mut remaining = body;
    while !remaining.is_empty() {
        let line_end = remaining
            .find('\n')
            .map_or(remaining.len(), |index| index + 1);
        let line = &remaining[..line_end];
        let (number, source) = line
            .split_once(" | ")
            .ok_or("invalid enumerated READ result")?;
        if number.parse::<usize>().ok() != Some(expected) {
            return Err("invalid enumerated READ result".into());
        }
        content.push_str(source);
        remaining = &remaining[line_end..];
        expected += 1;
    }
    Ok(content)
}

fn read_ignored(root: &Path, file: &Path) -> Result<bool, String> {
    let source = match fs::read_to_string(root.join(".readignore")) {
        Ok(source) => source,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(_) => return Err("AccessDenied".into()),
    };
    let relative = file
        .strip_prefix(root)
        .expect("validated READ path is inside its root")
        .to_string_lossy()
        .replace('\\', "/");
    Ok(source.lines().map(str::trim).any(|pattern| {
        !pattern.is_empty() && !pattern.starts_with('#') && matches_ignore(pattern, &relative)
    }))
}

fn matches_ignore(pattern: &str, path: &str) -> bool {
    let pattern = pattern.trim_start_matches('/');
    let directory = pattern.strip_suffix('/').unwrap_or(pattern);
    let path_parts: Vec<_> = path.split('/').collect();
    let pattern_parts: Vec<_> = directory.split('/').collect();
    if pattern_parts.len() == 1 {
        return path_parts
            .iter()
            .any(|part| matches_component(pattern_parts[0], part));
    }
    if pattern.ends_with('/') {
        return matches_components(&pattern_parts, &path_parts, true);
    }
    matches_components(&pattern_parts, &path_parts, false)
}

fn matches_components(pattern: &[&str], path: &[&str], prefix: bool) -> bool {
    match (pattern, path) {
        ([], []) => true,
        ([], _) => prefix,
        (["**", rest @ ..], _) => {
            matches_components(rest, path, prefix)
                || (!path.is_empty() && matches_components(pattern, &path[1..], prefix))
        }
        ([part, rest @ ..], [path_part, path_rest @ ..]) => {
            matches_component(part, path_part) && matches_components(rest, path_rest, prefix)
        }
        _ => false,
    }
}

fn matches_component(pattern: &str, value: &str) -> bool {
    let pattern: Vec<_> = pattern.chars().collect();
    let value: Vec<_> = value.chars().collect();
    fn matches(pattern: &[char], value: &[char]) -> bool {
        match (pattern, value) {
            ([], []) => true,
            (['*', rest @ ..], _) => {
                matches(rest, value) || (!value.is_empty() && matches(pattern, &value[1..]))
            }
            (['?', rest @ ..], [_, value_rest @ ..]) => matches(rest, value_rest),
            ([expected, rest @ ..], [actual, value_rest @ ..]) if expected == actual => {
                matches(rest, value_rest)
            }
            _ => false,
        }
    }
    matches(&pattern, &value)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::*;

    fn directory() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "read-ignore-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&directory).unwrap();
        directory
    }

    #[test]
    fn returns_access_denied_for_paths_matching_readignore() {
        let directory = directory();
        fs::create_dir(directory.join("nested")).unwrap();
        fs::write(directory.join(".env"), "secret").unwrap();
        fs::write(directory.join("nested/.env"), "secret").unwrap();
        fs::write(directory.join("nested/token.pem"), "secret").unwrap();
        fs::write(directory.join("visible.txt"), "visible").unwrap();
        fs::write(directory.join(".readignore"), ".env\n*.pem\n").unwrap();

        assert_eq!(read(&directory, ".env"), Err("AccessDenied".into()));
        assert_eq!(
            read(&directory, "nested/../nested/.env"),
            Err("AccessDenied".into())
        );
        assert_eq!(
            read(&directory, "nested/token.pem"),
            Err("AccessDenied".into())
        );
        assert_eq!(read(&directory, "visible.txt"), Ok("visible".into()));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn blocks_a_directory_pattern_and_ignores_comments() {
        let directory = directory();
        fs::create_dir(directory.join("secrets")).unwrap();
        fs::write(directory.join("secrets/key"), "secret").unwrap();
        fs::write(directory.join("notes.txt"), "notes").unwrap();
        fs::write(
            directory.join(".readignore"),
            "# sensitive files\nsecrets/\n",
        )
        .unwrap();

        assert_eq!(read(&directory, "secrets/key"), Err("AccessDenied".into()));
        assert_eq!(read(&directory, "notes.txt"), Ok("notes".into()));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn enumerates_lines_without_losing_exact_content() {
        for content in ["", "one", "one\n", "one\r\ntwo\n\n"] {
            let numbered = enumerate(content);
            assert!(numbered.starts_with("Line | Content\n"));
            assert_eq!(enumerated_content(&numbered).unwrap(), content);
        }
        assert_eq!(enumerate("one\ntwo"), "Line | Content\n1 | one\n2 | two");
        assert!(enumerated_content("1 | one\n").is_err());
        assert!(enumerated_content("Line | Content\n2 | one\n").is_err());
    }
}
