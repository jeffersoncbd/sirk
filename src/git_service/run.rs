use crate::services::{BashService, Invocation};
use std::{io, path::Path};

pub(super) fn run(directory: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let result = BashService::default()
        .execute_bytes_to(
            &Invocation {
                program: "git".to_owned(),
                arguments: arguments
                    .iter()
                    .map(|argument| (*argument).to_owned())
                    .collect(),
                working_directory: directory.to_owned(),
                environment: Default::default(),
            },
            &mut io::sink(),
        )
        .map_err(|error| format!("could not run Git: {error}"))?;
    if result.status.success() {
        Ok(result.stdout)
    } else {
        Err(format!("Git exited with {}", result.status))
    }
}
