use super::flow::flow;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[tokio::test]
async fn creates_the_flow_directory_layout() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-flow-layout-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&directory).unwrap();
    let flow_id = flow(&directory).await;
    let flow_directory = directory.join("history").join(&flow_id);
    assert!(flow_directory.join("flow.log").is_file());
    assert!(flow_directory.join("usage.log").is_file());
    assert!(flow_directory.join("conversations").is_dir());
    fs::remove_dir_all(directory).unwrap();
}
