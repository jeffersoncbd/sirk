use std::env;

pub(super) fn run_workflow(name: &str) -> Result<(), String> {
    let path = super::workflow_path::workflow_path(name)?;
    let workflow = new_harness::workflow::Workflow::from_file(&path)?;
    let working_directory = env::current_dir()
        .and_then(|directory| directory.canonicalize())
        .map_err(|error| format!("could not resolve current working directory: {error}"))?;
    new_harness::runner::run(&workflow, &working_directory)?;
    Ok(())
}
