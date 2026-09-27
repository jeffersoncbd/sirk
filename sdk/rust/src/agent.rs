use crate::{
    Error, Sirk,
    protocol::{AgentParams, Request, Response},
};
use std::io::{BufRead, Write};

impl Sirk {
    pub fn agent(&mut self, agent: &str, input: &str) -> Result<String, Error> {
        let id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| Error::Protocol("request id overflow".to_owned()))?;
        let request = Request {
            jsonrpc: "2.0",
            id,
            method: "agent.run",
            params: AgentParams { agent, input },
        };
        let writer = self
            .input
            .as_mut()
            .ok_or_else(|| Error::Protocol("CLI stdin is closed".to_owned()))?;
        serde_json::to_writer(&mut *writer, &request)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        let mut line = String::new();
        if self.output.read_line(&mut line)? == 0 {
            return Err(Error::Protocol(
                "CLI closed stdout before responding".to_owned(),
            ));
        }
        let response: Response = serde_json::from_str(&line)?;
        if response.jsonrpc != "2.0" || response.id != id {
            return Err(Error::Protocol(
                "response metadata did not match".to_owned(),
            ));
        }
        if let Some(error) = response.error {
            return Err(Error::Remote {
                code: error.code,
                message: error.message,
            });
        }
        response
            .result
            .ok_or_else(|| Error::Protocol("response has neither result nor error".to_owned()))
    }
}
