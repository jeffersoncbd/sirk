use super::super::handle::handle;
use std::{
    fs,
    os::unix::fs::PermissionsExt,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn preserves_literal_template_input() {
    let directory = std::env::temp_dir().join(format!(
        "sirk-literal-input-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(directory.join(".agents")).unwrap();
    let adapter = directory.join("adapter");
    fs::write(&adapter, "#!/bin/sh\ncase \"$*\" in\n  *'{{ outputs.plan }}'*) printf '%s' 'literal preserved' ;;\n  *) exit 4 ;;\nesac\n").unwrap();
    let mut permissions = fs::metadata(&adapter).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&adapter, permissions).unwrap();
    fs::write(
        directory.join(".agents/reviewer.md"),
        format!(
            "---\nadapter: codex\ncall_prefix: '{}'\n---\nReview the input.",
            adapter.display()
        ),
    )
    .unwrap();
    let body = serde_json::json!({
        "directory": directory,
        "agent": "reviewer",
        "input": "File content: {{ outputs.plan }}"
    })
    .to_string();
    let response = handle("POST", "/v1/agent/run", &body);
    assert_eq!(response.status, 200, "{}", response.body);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&response.body).unwrap()["result"],
        "literal preserved"
    );
    fs::remove_dir_all(directory).unwrap();
}
