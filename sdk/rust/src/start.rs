use crate::{Error, Sirk};
use std::{
    ffi::OsStr,
    io::BufReader,
    path::Path,
    process::{Command, Stdio},
};

impl Sirk {
    pub fn start(
        executable: impl AsRef<OsStr>,
        directory: impl AsRef<Path>,
    ) -> Result<Self, Error> {
        let mut child = Command::new(executable)
            .arg("rpc")
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let input = child
            .stdin
            .take()
            .ok_or_else(|| Error::Protocol("CLI stdin was not piped".to_owned()))?;
        let output = child
            .stdout
            .take()
            .ok_or_else(|| Error::Protocol("CLI stdout was not piped".to_owned()))?;
        Ok(Self {
            child,
            input: Some(input),
            output: BufReader::new(output),
            next_id: 1,
        })
    }
}
