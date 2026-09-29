use super::{flow::flow, request::request};
use crate::services::{BashService, Invocation};
use axum::body::to_bytes;
use std::{
    fs, io,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn handles_tree_requests() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-http-tree-{}-{}",
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
    fs::write(directory.join("visible.rs"), "visible").unwrap();
    fs::write(directory.join(".treeignore"), "hidden.rs\n").unwrap();
    fs::write(directory.join("hidden.rs"), "hidden").unwrap();
    let flow_id = flow(&directory).await;
    let response = request(
        "POST",
        "/v1/tree",
        serde_json::json!({ "directory": directory }).to_string(),
        Some(&flow_id),
    )
    .await;
    let status = response.status();
    let response = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(status, 200, "{}", String::from_utf8_lossy(&response));
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&response).unwrap()["paths"],
        serde_json::json!([".treeignore", "visible.rs"])
    );
    fs::remove_dir_all(directory).unwrap();
}
