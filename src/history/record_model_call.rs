use super::{History, ResumeCall};
use crate::harness::TokenUsage;

impl History {
    pub fn record_model_call(&mut self, adapter: &str, usage: Option<&TokenUsage>) {
        let call = self
            .resume_calls
            .iter_mut()
            .find(|call| call.adapter == adapter);
        let call = match call {
            Some(call) => call,
            None => {
                self.resume_calls.push(ResumeCall {
                    adapter: adapter.to_owned(),
                    calls: 0,
                    input_tokens: 0,
                    output_tokens: 0,
                    has_usage: false,
                });
                self.resume_calls.last_mut().expect("call was just added")
            }
        };
        call.calls += 1;
        if let Some(usage) = usage {
            call.input_tokens += usage.input_tokens;
            call.output_tokens += usage.output_tokens;
            call.has_usage = true;
        }
    }
}
