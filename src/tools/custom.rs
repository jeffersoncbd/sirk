//! CUSTOM-TOOL: execute a project-local Bash script with positional arguments.
use crate::services::{BashService, Invocation};
use std::path::Path;

pub fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

pub fn arguments(input: &str) -> Result<Vec<String>, String> {
    serde_json::from_str(input)
        .map_err(|error| format!("CUSTOM-TOOL requires a list of string arguments: {error}"))
}

pub fn execute(name: &str, arguments: &[String], directory: &Path) -> Result<String, String> {
    if !valid_name(name) {
        return Err(format!(
            "invalid custom tool name `{name}`; use letters, digits, `_` or `-`"
        ));
    }
    let root = directory
        .canonicalize()
        .map_err(|error| format!("CUSTOM-TOOL cannot resolve execution directory: {error}"))?;
    let tools_directory = root
        .join("tools")
        .canonicalize()
        .map_err(|error| format!("CUSTOM-TOOL cannot resolve the tools directory: {error}"))?;
    if !tools_directory.starts_with(&root) {
        return Err("CUSTOM-TOOL tools directory must stay inside the execution directory".into());
    }
    let requested = tools_directory.join(format!("{name}.sh"));
    let script = requested.canonicalize().map_err(|error| {
        format!(
            "CUSTOM-TOOL cannot resolve script `{}`: {error}",
            requested.display()
        )
    })?;
    if !script.starts_with(&tools_directory) {
        return Err("CUSTOM-TOOL script must stay inside the tools directory".into());
    }
    if !script.is_file() {
        return Err(format!(
            "CUSTOM-TOOL script is not a regular file: {}",
            script.display()
        ));
    }
    let script = script
        .to_str()
        .ok_or("CUSTOM-TOOL script path is not valid UTF-8")?;
    let invocation = Invocation {
        program: "bash".into(),
        arguments: std::iter::once(script.to_owned())
            .chain(arguments.iter().cloned())
            .collect(),
        working_directory: root,
        environment: Default::default(),
    };
    let result = BashService::default()
        .execute_to(&invocation, &mut Vec::new())
        .map_err(|error| format!("CUSTOM-TOOL `{name}` could not execute: {error}"))?;
    if !result.status.success() {
        return Err(format!(
            "CUSTOM-TOOL `{name}` exited with {}; incomplete output was not committed",
            result.status
        ));
    }
    Ok(result.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temporary_project() -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "custom-tool-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(directory.join("tools")).unwrap();
        directory
    }

    #[test]
    fn passes_arguments_literally_and_captures_stdout() {
        let project = temporary_project();
        fs::write(
            project.join("tools/inspect.sh"),
            "printf '%s\\n' \"$#\" \"$1\" \"$2\"\n",
        )
        .unwrap();
        let output = execute(
            "inspect",
            &["spaces and 'quotes'".into(), "$(exit 19); `exit 20`".into()],
            &project,
        )
        .unwrap();
        assert_eq!(output, "2\nspaces and 'quotes'\n$(exit 19); `exit 20`\n");
        fs::remove_dir_all(project).unwrap();
    }

    #[test]
    fn reports_missing_scripts_and_unsuccessful_exits() {
        let project = temporary_project();
        assert!(execute("missing", &[], &project).is_err());
        fs::write(project.join("tools/fail.sh"), "printf partial; exit 7\n").unwrap();
        let error = execute("fail", &[], &project).unwrap_err();
        assert!(error.contains("exit status: 7"));
        fs::remove_dir_all(project).unwrap();
    }
}
