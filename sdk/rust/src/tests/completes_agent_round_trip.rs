use crate::Sirk;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn completes_agent_round_trip() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-sdk-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&directory).unwrap();
    let server = directory.join("mock-sirk");
    fs::write(
        &server,
        "#!/bin/sh\nIFS= read -r request\nprintf '%s\\n' '{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":\"documented\"}'\n",
    )
    .unwrap();
    let mut permissions = fs::metadata(&server).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&server, permissions).unwrap();
    let mut sirk = Sirk::start(&server, &directory).unwrap();
    assert_eq!(
        sirk.agent("code-explainer", "Explain this file").unwrap(),
        "documented"
    );
    drop(sirk);
    fs::remove_dir_all(directory).unwrap();
}
