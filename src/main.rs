#[path = "main/create_agent.rs"]
mod create_agent;
#[path = "main/http.rs"]
mod http;
#[path = "main/print_usage.rs"]
mod print_usage;
#[path = "main/resume.rs"]
mod resume;
#[path = "main/rpc.rs"]
mod rpc;
#[path = "main/run.rs"]
mod run;
#[path = "main/run_workflow.rs"]
mod run_workflow;
#[path = "main/usage.rs"]
mod usage;
#[path = "main/workflow_path.rs"]
mod workflow_path;

use std::{env, process::ExitCode};

fn main() -> ExitCode {
    match run::run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
#[path = "main/tests.rs"]
mod tests;
