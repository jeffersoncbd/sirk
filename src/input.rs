use std::io::{self, Write};

/// Shared interaction boundary for initial questions and agent clarification.
pub trait UserInput {
    fn ask(&mut self, question: &str) -> Result<String, String>;
}

pub struct TerminalInput;

impl UserInput for TerminalInput {
    fn ask(&mut self, question: &str) -> Result<String, String> {
        println!("{question}");
        loop {
            print!("> ");
            io::stdout().flush().map_err(|e| e.to_string())?;
            let mut line = String::new();
            if io::stdin()
                .read_line(&mut line)
                .map_err(|e| e.to_string())?
                == 0
            {
                return Err("input closed; resume the history file to continue".into());
            }
            let answer = line.trim_end_matches(['\r', '\n']);
            if answer == "/cancel" {
                return Err("input cancelled; resume the history file to continue".into());
            }
            if !answer.trim().is_empty() {
                return Ok(answer.to_owned());
            }
            println!("Please enter a response, or /cancel.");
        }
    }
}
