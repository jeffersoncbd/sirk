use super::*;

#[test]
fn rejects_an_invalid_command() {
    assert!(
        run(vec!["invalid".to_owned()])
            .unwrap_err()
            .contains("invalid command")
    );
}
