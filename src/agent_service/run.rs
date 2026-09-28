use crate::{
    runner::run_silent_with,
    workflow::{Step, StepInput, Workflow},
};
use std::path::Path;

pub(crate) fn run(directory: &Path, agent: String, input: String) -> Result<String, String> {
    let workflow = Workflow {
        version: 1,
        steps: vec![Step {
            agent: Some(agent),
            tool: None,
            custom_tool: None,
            input: StepInput::Text(input),
            path: None,
            force: None,
            skip: None,
            operation: None,
            line: None,
            start: None,
            end: None,
            version: None,
            version_output: None,
            enumerate: None,
            output: Some("result".to_owned()),
            iter: Vec::new(),
            is_true: Vec::new(),
            is_false: Vec::new(),
        }],
    };
    run_silent_with(&workflow, directory, super::execute::execute)?
        .remove("result")
        .ok_or_else(|| "agent execution did not produce a result".to_owned())
}
