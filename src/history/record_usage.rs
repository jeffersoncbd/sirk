use super::History;
use crate::harness::TokenUsage;
use std::{fs::OpenOptions, io::Write};

impl History {
    pub fn record_usage(&self, adapter: &str, usage: &TokenUsage) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .map_err(|error| error.to_string())?;
        file.write_all(b"==> USAGE\n")
            .and_then(|_| file.write_all(format!("adapter: {adapter}\n").as_bytes()))
            .and_then(|_| {
                file.write_all(format!("input_tokens: {}\n", usage.input_tokens).as_bytes())
            })
            .and_then(|_| {
                file.write_all(format!("output_tokens: {}\n\n", usage.output_tokens).as_bytes())
            })
            .and_then(|_| file.sync_all())
            .map_err(|error| error.to_string())?;
        std::fs::File::open(self.path.parent().ok_or("missing flow directory")?)
            .and_then(|file| file.sync_all())
            .map_err(|error| error.to_string())
    }
}
