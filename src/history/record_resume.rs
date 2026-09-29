use super::History;
use std::{fs::OpenOptions, io::Write};

impl History {
    pub fn record_resume(&self) -> Result<(), String> {
        if self.resume_calls.is_empty() {
            return Ok(());
        }
        let text = std::fs::read_to_string(&self.resume_path)
            .or_else(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    Ok(String::new())
                } else {
                    Err(error)
                }
            })
            .map_err(|error| error.to_string())?;
        let mut lines = text.lines().map(str::to_owned).collect::<Vec<_>>();
        for call in &self.resume_calls {
            let prefix = format!("{}: ", call.adapter);
            let header = lines.iter().position(|line| line.starts_with(&prefix));
            let (index, previous_calls, previous_input_tokens, previous_output_tokens, had_usage) =
                match header {
                    Some(index) => {
                        let line = &lines[index];
                        let previous_calls = line
                            .strip_prefix(&prefix)
                            .and_then(|value| value.split_once(" calls"))
                            .and_then(|(value, _)| value.parse::<u64>().ok())
                            .unwrap_or(0);
                        let previous_input_tokens = line
                            .split(" - total_input_tokens: ")
                            .nth(1)
                            .and_then(|value| value.split_whitespace().next())
                            .and_then(|value| value.parse::<u64>().ok())
                            .unwrap_or(0);
                        let previous_output_tokens = line
                            .split(" - total_output_tokens: ")
                            .nth(1)
                            .and_then(|value| value.parse::<u64>().ok())
                            .unwrap_or(0);
                        (
                            index,
                            previous_calls,
                            previous_input_tokens,
                            previous_output_tokens,
                            line.contains(" - total_input_tokens: "),
                        )
                    }
                    None => {
                        lines.push(String::new());
                        (lines.len() - 1, 0, 0, 0, false)
                    }
                };
            let calls = previous_calls + call.calls;
            let has_usage = had_usage || call.has_usage;
            lines[index] = if has_usage {
                format!(
                    "{}: {calls} calls - total_input_tokens: {} - total_output_tokens: {}",
                    call.adapter,
                    previous_input_tokens + call.input_tokens,
                    previous_output_tokens + call.output_tokens,
                )
            } else {
                format!("{}: {calls} calls", call.adapter)
            };
            let mut agent_index = index + 1;
            while agent_index < lines.len() && lines[agent_index].starts_with("   - ") {
                agent_index += 1;
            }
            let model = self.snapshot.agent.model.as_deref().unwrap_or("default");
            lines.insert(
                agent_index,
                format!("   - {} ({model})", self.snapshot.agent.id),
            );
        }
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.resume_path)
            .map_err(|error| error.to_string())?;
        file.write_all(lines.join("\n").trim().as_bytes())
            .and_then(|_| file.write_all(b"\n"))
            .and_then(|_| file.sync_all())
            .map_err(|error| error.to_string())?;
        std::fs::File::open(self.resume_path.parent().ok_or("missing flow directory")?)
            .and_then(|file| file.sync_all())
            .map_err(|error| error.to_string())
    }
}
