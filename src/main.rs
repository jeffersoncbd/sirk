use std::env;
use std::io::{self, Write};
use std::path::Path;
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
        [] => interactive(),
        [command] if matches!(command.as_str(), "--newAgent" | "--new-agent") => {
            let path = new_harness::tools::new_agent::create(
                &env::current_dir().map_err(|e| e.to_string())?,
                &mut new_harness::input::TerminalInput,
            )?;
            println!("Agent created: {}", path.display());
            Ok(())
        }
        [command, path] if command == "run" => run_workflow(Path::new(path)),
        [command, path] if command == "resume" => resume_workflow(Path::new(path)),
        [command] if matches!(command.as_str(), "help" | "--help" | "-h") => {
            print_usage();
            Ok(())
        }
        _ => Err(format!("invalid command\n\n{}", usage())),
    }
}

fn interactive() -> Result<(), String> {
    println!("New Harness — workflow orchestrator");
    println!("Enter `run <workflow.yml>`, a workflow path, or `/help`.");
    let stdin = io::stdin();
    loop {
        print!("new-harness> ");
        io::stdout().flush().map_err(|error| error.to_string())?;
        let mut line = String::new();
        if stdin
            .read_line(&mut line)
            .map_err(|error| error.to_string())?
            == 0
        {
            return Ok(());
        }
        let line = line.trim();
        match line {
            "" => continue,
            "/quit" | "/exit" => return Ok(()),
            "/help" => print_usage(),
            _ => {
                if let Some(path) = line.strip_prefix("resume ") {
                    if let Err(error) = resume_workflow(Path::new(path)) {
                        eprintln!("error: {error}");
                    }
                    continue;
                }
                let path = line.strip_prefix("run ").unwrap_or(line);
                if let Err(error) = run_workflow(Path::new(path)) {
                    eprintln!("error: {error}");
                }
            }
        }
    }
}

fn run_workflow(path: &Path) -> Result<(), String> {
    let workflow = Workflow::from_file(path)?;
    let working_directory = env::current_dir()
        .and_then(|directory| directory.canonicalize())
        .map_err(|error| format!("could not resolve current working directory: {error}"))?;
    runner::run(&workflow, &working_directory)?;
    Ok(())
}

fn print_usage() {
    print!("{}", usage());
}

fn resume_workflow(path: &Path) -> Result<(), String> {
    runner::resume(path)?;
    Ok(())
}

fn usage() -> &'static str {
    "Usage:\n  new-harness\n  new-harness --newAgent\n  new-harness run <workflow.yml>\n  new-harness resume <history.log>\n\n--newAgent (alias --new-agent) creates an agent interactively.\nWith no arguments, opens an interactive workflow session.\n\nInteractive commands:\n  run <workflow.yml>  Start a workflow\n  <workflow.yml>      Start a workflow directly\n  resume <history.log> Resume an editable transcript\n  /help               Show this help\n  /quit               Leave the session\n\nDuring a question, /cancel cancels input. Workflow history can be resumed.\n"
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
    fn usage_distinguishes_cli_and_interactive_workflow_commands() {
        let text = usage();
        assert!(text.contains("new-harness run <workflow.yml>"));
        assert!(text.contains("  <workflow.yml>      Start a workflow directly"));
        assert!(!text.contains("Workflow commands:"));
    }
}
