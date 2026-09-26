use crate::input::UserInput;

pub(super) fn nonempty(input: &mut impl UserInput, question: &str) -> Result<String, String> {
    loop {
        let value = input.ask(question)?;
        if !value.trim().is_empty() {
            return Ok(value);
        }
    }
}
