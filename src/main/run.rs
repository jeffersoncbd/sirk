use super::{create_agent, print_usage, resume, rpc, run_workflow, usage};
use std::path::Path;

pub(super) fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [] => {
            print_usage::print_usage();
            Ok(())
        }
        [command] if matches!(command.as_str(), "--newAgent" | "--new-agent") => {
            create_agent::create()
        }
        [command] if command == "rpc" => rpc::rpc(),
        [command, name] if command == "run" => run_workflow::run_workflow(name),
        [command, path] if command == "resume" => resume::resume(Path::new(path)),
        [command] if matches!(command.as_str(), "help" | "--help" | "-h") => {
            print_usage::print_usage();
            Ok(())
        }
        _ => Err(format!("invalid command\n\n{}", usage::usage())),
    }
}
