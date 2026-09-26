use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use new_harness::runner;
use new_harness::workflow::Workflow;

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [] => {
            print_usage();
            Ok(())
        }
        [command] if matches!(command.as_str(), "--newAgent" | "--new-agent") => {
            let path = new_harness::tools::new_agent::create(
                &env::current_dir().map_err(|e| e.to_string())?,
                &mut new_harness::input::TerminalInput,
            )?;
            println!("Agent created: {}", path.display());
            Ok(())
        }
        [command, name] if command == "run" => run_workflow(name),
        [command, path] if command == "resume" => resume_workflow(Path::new(path)),
        [command] if matches!(command.as_str(), "help" | "--help" | "-h") => {
            print_usage();
            Ok(())
        }
        _ => Err(format!("invalid command\n\n{}", usage())),
    }
}

fn run_workflow(name: &str) -> Result<(), String> {
    let path = workflow_path(name)?;
    let workflow = Workflow::from_file(&path)?;
    let working_directory = env::current_dir()
        .and_then(|directory| directory.canonicalize())
        .map_err(|error| format!("could not resolve current working directory: {error}"))?;
    runner::run(&workflow, &working_directory)?;
    Ok(())
}

fn workflow_path(name: &str) -> Result<PathBuf, String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        return Err(format!(
            "invalid flow name `{name}`; use letters, digits, `_` or `-`"
        ));
    }
    Ok(Path::new("flows").join(format!("{name}.yml")))
}

fn print_usage() {
    print!("{}", usage());
}

fn resume_workflow(path: &Path) -> Result<(), String> {
    runner::resume(path)?;
    Ok(())
}

fn usage() -> &'static str {
    "Usage:\n  new-harness --newAgent\n  new-harness run <flow-name>\n  new-harness resume <history.log>\n\nFlows resolve to flows/<flow-name>.yml.\n\n--newAgent (alias --new-agent) creates an agent interactively. During a question, /cancel cancels input. Workflow history can be resumed.\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_an_invalid_command() {
        assert!(
            run(vec!["invalid".to_owned()])
                .unwrap_err()
                .contains("invalid command")
        );
    }

    #[test]
    fn usage_lists_explicit_flow_commands() {
        let text = usage();
        assert!(text.contains("new-harness run <flow-name>"));
        assert!(text.contains("Flows resolve to flows/<flow-name>.yml."));
        assert!(!text.contains("Interactive commands:"));
        assert!(!text.contains("<flow-name>      Start a flow directly"));
    }

    #[test]
    fn accepts_no_arguments_without_starting_a_prompt() {
        run(vec![]).unwrap();
    }

    #[test]
    fn resolves_flow_names_inside_the_flows_directory() {
        assert_eq!(
            workflow_path("documentation").unwrap(),
            Path::new("flows/documentation.yml")
        );
        assert_eq!(
            workflow_path("release_notes-2").unwrap(),
            Path::new("flows/release_notes-2.yml")
        );
    }

    #[test]
    fn rejects_flow_paths_and_extensions() {
        for invalid in ["", "documentation.yml", "../documentation", "nested/flow"] {
            assert!(workflow_path(invalid).is_err(), "{invalid:?}");
        }
    }
}
