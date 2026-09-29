use super::{flow::flow, request::request};
use crate::services::{BashService, Invocation};
use axum::body::to_bytes;
use std::{
    fs, io,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn handles_git_add_requests() {
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
    fs::write(directory.join(".git/info/exclude"), "history/\n").unwrap();
    fs::write(directory.join("generated.md"), "generated").unwrap();
    let flow_id = flow(&directory).await;
    let response = request(
        "POST",
        "/v1/git/add",
        serde_json::json!({ "directory": directory }).to_string(),
        Some(&flow_id),
    )
    .await;
    let status = response.status();
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(status, 200, "{}", String::from_utf8_lossy(&response));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&response).unwrap()["status"],
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
