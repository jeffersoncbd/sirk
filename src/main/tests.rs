use super::{run::run, usage::usage};

#[path = "tests/accepts_no_arguments_without_starting_a_prompt.rs"]
mod accepts_no_arguments_without_starting_a_prompt;
#[path = "tests/rejects_an_invalid_command.rs"]
mod rejects_an_invalid_command;
#[path = "tests/rejects_removed_workflow_commands.rs"]
mod rejects_removed_workflow_commands;
#[path = "tests/usage_lists_service_commands.rs"]
mod usage_lists_service_commands;
