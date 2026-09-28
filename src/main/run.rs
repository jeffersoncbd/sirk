use super::{create_agent, http, print_usage, usage};

pub(super) fn run(arguments: Vec<String>) -> Result<(), String> {
    match arguments.as_slice() {
        [] => {
            print_usage::print_usage();
            Ok(())
        }
        [command] if matches!(command.as_str(), "--newAgent" | "--new-agent") => {
            create_agent::create()
        }
        [command] if command == "http" => http::http(None),
        [command, address] if command == "http" => http::http(Some(address)),
        [command] if matches!(command.as_str(), "help" | "--help" | "-h") => {
            print_usage::print_usage();
            Ok(())
        }
        _ => Err(format!("invalid command\n\n{}", usage::usage())),
    }
}
