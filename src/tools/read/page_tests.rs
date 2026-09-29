use crate::tools::read::page;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn reads_requested_line_pages_and_preserves_readignore() {
    let directory = std::env::temp_dir().join(format!(
        "read-page-test-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir(&directory).unwrap();
    fs::write(directory.join("visible.txt"), "one\ntwo\nthree\nfour\n").unwrap();
    fs::write(directory.join("hidden.txt"), "secret").unwrap();
    fs::write(directory.join(".readignore"), "hidden.txt\n").unwrap();
    let result = page(&directory, r#"{"path":"visible.txt","offset":2,"limit":2}"#).unwrap();
    assert_eq!(result.offset, 2);
    assert_eq!(result.content, "two\nthree\n");
    assert_eq!(
        page(&directory, "visible.txt").unwrap().content,
        "one\ntwo\nthree\nfour\n"
    );
    assert_eq!(
        page(&directory, r#"{"path":"hidden.txt","offset":1,"limit":1}"#).map(|_| ()),
        Err("AccessDenied".into())
    );
    fs::remove_dir_all(directory).unwrap();
}
