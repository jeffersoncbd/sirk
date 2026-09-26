/// Shared interaction boundary for initial questions and agent clarification.
pub trait UserInput {
    fn ask(&mut self, question: &str) -> Result<String, String>;

    fn await_confirmation(&mut self, prompt: &str) -> Result<(), String> {
        self.ask(prompt).map(|_| ())
    }
}
