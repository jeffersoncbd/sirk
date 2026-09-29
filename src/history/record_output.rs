use super::History;
use std::{fs::OpenOptions, io::Write};

impl History {
    pub fn record_output(&self, output: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .map_err(|error| error.to_string())?;
        file.write_all(b"<== OUTPUT\n")
            .and_then(|_| file.write_all(output.as_bytes()))
            .and_then(|_| file.write_all(b"\n\n"))
            .and_then(|_| file.sync_all())
            .map_err(|error| error.to_string())?;
        std::fs::File::open(self.path.parent().ok_or("missing flow directory")?)
            .and_then(|file| file.sync_all())
            .map_err(|error| error.to_string())
    }
}
