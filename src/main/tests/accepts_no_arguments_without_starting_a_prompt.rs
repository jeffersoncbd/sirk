use super::*;

#[test]
fn accepts_no_arguments_without_starting_a_prompt() {
    run(vec![]).unwrap();
}
