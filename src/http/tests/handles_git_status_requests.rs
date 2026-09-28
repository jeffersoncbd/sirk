use super::super::handle::handle;
use crate::services::{BashService, Invocation};
use std::{
    fs, io,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn handles_git_status_requests() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-git-status-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let result = BashService::default()
        .execute_to(
            &Invocation {
                program: "git".to_owned(),
                arguments: vec!["init".to_owned(), "--quiet".to_owned()],
                working_directory: directory.clone(),
                environment: Default::default(),
            },
            &mut io::sink(),
        )
        .unwrap();
    assert!(result.status.success());
    fs::write(directory.join("visible.rs"), "visible").unwrap();
    fs::write(directory.join(".treeignore"), "hidden.rs\n").unwrap();
    fs::write(directory.join("hidden.rs"), "hidden").unwrap();
    let response = handle(
        "POST",
        "/v1/git/status",
        &serde_json::json!({ "directory": directory }).to_string(),
    );
    assert_eq!(response.status, 200, "{}", response.body);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&response.body).unwrap()["paths"],
        serde_json::json!([".treeignore", "visible.rs"])
    );
    fs::remove_dir_all(directory).unwrap();
}
