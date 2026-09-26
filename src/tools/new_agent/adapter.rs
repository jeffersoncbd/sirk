use crate::{adapters, input::UserInput};

pub(super) fn choose(input: &mut impl UserInput, question: &str) -> Result<String, String> {
    let mut question = format!("{question} Available: {}", adapters::AVAILABLE.join(", "));
    loop {
        let value = input.ask(&question)?.trim().to_lowercase();
        if adapters::AVAILABLE.contains(&value.as_str()) {
            return Ok(value);
        }
        question = format!(
            "Unsupported adapter. Choose one of: {}",
            adapters::AVAILABLE.join(", ")
        );
    }
}
