use crate::{Error, Sirk};

impl Sirk {
    pub fn agent(&self, agent: &str, input: &str) -> Result<String, Error> {
        self.transport.run_agent(&self.directory, agent, input)
    }
}
