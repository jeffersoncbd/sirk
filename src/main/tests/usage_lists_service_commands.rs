use super::*;

#[test]
fn usage_lists_service_commands() {
    let text = usage();
    assert!(text.starts_with("S.I.R.K."));
    assert!(text.contains("sirk --newAgent"));
    assert!(text.contains("sirk http [address]"));
    assert!(!text.contains("sirk rpc"));
    assert!(!text.contains("sirk run"));
    assert!(!text.contains("sirk resume"));
}
