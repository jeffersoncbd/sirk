use crate::services::{BashService, Invocation};
use std::{io, path::Path, process::ExitStatus};

pub(super) struct GitOutput {
    pub(super) status: ExitStatus,
    pub(super) stdout: Vec<u8>,
}

pub(super) fn git(root: &Path, arguments: &[&str]) -> Result<GitOutput, String> {
    let result = BashService::default()
        .execute_bytes_to(
            &Invocation {
                program: "git".into(),
                arguments: arguments.iter().map(|value| (*value).into()).collect(),
                working_directory: root.to_owned(),
                environment: Default::default(),
            },
            &mut io::sink(),
        )
        .map_err(|error| format!("GIT-STATUS-TREE could not execute Git: {error}"))?;
    Ok(GitOutput {
        status: result.status,
        stdout: result.stdout,
    })
}
