use super::BashService;
use crate::services::{Invocation, quote::shell_quote};

impl BashService {
    pub fn render(&self, invocation: &Invocation) -> String {
        std::iter::once("exec --".to_owned())
            .chain(std::iter::once(shell_quote(
                &invocation.program,
                &invocation.environment,
            )))
            .chain(
                invocation
                    .arguments
                    .iter()
                    .map(|argument| shell_quote(argument, &invocation.environment)),
            )
            .collect::<Vec<_>>()
            .join(" ")
    }
}
