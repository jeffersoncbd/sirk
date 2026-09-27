use super::{run::run, usage::usage, workflow_path::workflow_path};
use std::path::Path;

#[path = "tests/accepts_no_arguments_without_starting_a_prompt.rs"]
mod accepts_no_arguments_without_starting_a_prompt;
#[path = "tests/rejects_an_invalid_command.rs"]
mod rejects_an_invalid_command;
#[path = "tests/rejects_flow_paths_and_extensions.rs"]
mod rejects_flow_paths_and_extensions;
#[path = "tests/resolves_flow_names_inside_the_flows_directory.rs"]
mod resolves_flow_names_inside_the_flows_directory;
#[path = "tests/usage_lists_explicit_flow_commands.rs"]
mod usage_lists_explicit_flow_commands;
