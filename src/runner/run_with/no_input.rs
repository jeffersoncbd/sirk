use crate::input::UserInput;

pub(super) struct NoInput;

impl UserInput for NoInput {
    fn ask(&mut self, _: &str) -> Result<String, String> {
        Err("this execution requires user input".into())
    }
}
