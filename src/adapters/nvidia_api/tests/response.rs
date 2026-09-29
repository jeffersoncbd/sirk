use crate::{
    adapters::NvidiaApiAdapter,
    harness::{HarnessAdapter, HarnessResponse, TokenUsage},
};

#[test]
fn extracts_the_first_choice_content_and_rejects_missing_choices() {
    let adapter = NvidiaApiAdapter::new("curl", Some("key".into()));
    assert_eq!(
        adapter
            .response(r#"{"choices":[{"message":{"content":"Done."}}],"usage":{"prompt_tokens":12,"completion_tokens":4}}"#.to_owned())
            .unwrap(),
        HarnessResponse {
            text: "Done.".to_owned(),
            usage: Some(TokenUsage {
                input_tokens: 12,
                output_tokens: 4,
            }),
        }
    );
    assert!(adapter.response(r#"{"choices":[]}"#.to_owned()).is_err());
}
