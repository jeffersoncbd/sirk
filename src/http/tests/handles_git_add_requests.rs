use super::super::handle::handle;
use crate::services::{BashService, Invocation};
use std::{
    fs, io,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn handles_git_add_requests() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-git-add-{}-{}",
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
    fs::write(directory.join("generated.md"), "generated").unwrap();
    let response = handle(
        "POST",
        "/v1/git/add",
        &serde_json::json!({ "directory": directory }).to_string(),
    );
    assert_eq!(response.status, 200, "{}", response.body);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&response.body).unwrap()["status"],
        "ok"
    );
    let staged = BashService::default()
        .execute_to(
            &Invocation {
                program: "git".to_owned(),
                arguments: vec![
                    "diff".to_owned(),
                    "--cached".to_owned(),
                    "--name-only".to_owned(),
                ],
                working_directory: directory.clone(),
                environment: Default::default(),
            },
            &mut io::sink(),
        )
        .unwrap();
    assert_eq!(staged.stdout, "generated.md\n");
    fs::remove_dir_all(directory).unwrap();
}
