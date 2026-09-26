use super::*;

#[test]
fn edit_rejects_stale_read_version_and_preserves_file() {
    let project = Project::new();
    fs::write(project.0.join("file"), "original\n").unwrap();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: READ\n  input: file\n  version-output: revision\n- agent: second\n- tool: EDIT\n  path: file\n  operation: delete\n  start: 1\n  end: 1\n  version: '{{ outputs.revision }}'\n").unwrap();
    let error = run_with(&workflow, &project.0, |_| {
        fs::write(project.0.join("file"), "external change\n").unwrap();
        Ok("Done".into())
    })
    .unwrap_err();
    assert!(error.contains("version conflict"));
    assert_eq!(
        fs::read_to_string(project.0.join("file")).unwrap(),
        "external change\n"
    );
}

#[test]
fn prepared_edits_recover_before_and_after_commit_without_duplication() {
    use crate::tools::edit::{Operation, Pending, Request};
    for operation in [Operation::Append, Operation::Prepend] {
        for already_written in [false, true] {
            let project = Project::new();
            fs::write(project.0.join("file"), "original\n").unwrap();
            let op = if operation == Operation::Append {
                "append"
            } else {
                "prepend"
            };
            let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: file\n  operation: {op}\n  input: \"entry\\n\"\n  output: diff\n")).unwrap();
            let mut pending = Pending {
                request: Request {
                    path: "file".into(),
                    operation,
                    line: None,
                    start: None,
                    end: None,
                    version: None,
                    input: "entry\n".into(),
                },
                before: None,
                was_missing: false,
            };
            pending.prepare(&project.0).unwrap();
            let expected = pending.request.apply_to("original\n").unwrap();
            let mut history = History::create(Snapshot {
                directory: project.0.clone(),
                workflow,
                agents: vec![],
            })
            .unwrap();
            history.labels.push("Step 1 — EDIT".into());
            history
                .steps
                .push(vec![Block::Input(serde_json::to_string(&pending).unwrap())]);
            history.save().unwrap();
            if already_written {
                pending.commit(&project.0).unwrap();
            }
            let path = history.path.clone();
            drop(history);
            let mut history = History::open(&path).unwrap();
            let outputs =
                continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap();
            assert_eq!(outputs["diff"], pending.diff().unwrap());
            assert_eq!(
                fs::read_to_string(project.0.join("file")).unwrap(),
                expected
            );
            assert_eq!(
                continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap(),
                outputs
            );
            assert_eq!(
                fs::read_to_string(project.0.join("file")).unwrap(),
                expected
            );
            // A pending operation followed by later history is rejected before applying.
            history.steps[0].pop();
            history.steps.push(vec![Block::Input("later".into())]);
            assert!(continue_with(&mut history, |_| panic!("invalid"), &mut answers(&[])).is_err());
            history.steps.pop();
            fs::write(project.0.join("file"), "unrelated").unwrap();
            assert!(
                continue_with(&mut history, |_| panic!("conflict"), &mut answers(&[]))
                    .unwrap_err()
                    .contains("conflict")
            );
            assert_eq!(
                fs::read_to_string(project.0.join("file")).unwrap(),
                "unrelated"
            );
            history.steps[0].push(Block::Output("forged diff".into()));
            assert!(
                continue_with(&mut history, |_| panic!("invalid"), &mut answers(&[]))
                    .unwrap_err()
                    .contains("invalid EDIT result")
            );
        }
    }
}

#[test]
fn edit_confines_paths_and_preserves_permissions() {
    use crate::tools::edit::{Operation, Pending, Request};
    let project = Project::new();
    let outside = Project::new();
    fs::write(project.0.join("file"), "before").unwrap();
    fs::write(project.0.join("binary"), [255]).unwrap();
    fs::write(outside.0.join("file"), "outside").unwrap();
    let mut pending = Pending {
        request: Request {
            path: "file".into(),
            operation: Operation::Append,
            line: None,
            start: None,
            end: None,
            version: None,
            input: "after".into(),
        },
        before: None,
        was_missing: false,
    };
    for path in [
        ".agents".into(),
        "binary".into(),
        outside.0.join("file").to_str().unwrap().into(),
    ] {
        pending.request.path = path;
        assert!(pending.prepare(&project.0).is_err());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{PermissionsExt, symlink};
        symlink(project.0.join("file"), project.0.join("link")).unwrap();
        symlink(&outside.0, project.0.join("external")).unwrap();
        for path in ["link", "external/file"] {
            pending.request.path = path.into();
            assert!(pending.prepare(&project.0).is_err());
        }
        fs::set_permissions(project.0.join("file"), fs::Permissions::from_mode(0o751)).unwrap();
        pending.request.path = "file".into();
        pending.prepare(&project.0).unwrap();
        pending.commit(&project.0).unwrap();
        assert_eq!(
            fs::metadata(project.0.join("file"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o751
        );
    }
    assert!(crate::tools::request("EDIT: file").is_none());
}
#[test]
fn edit_creates_missing_files_and_recovers_creation() {
    for operation in ["append", "prepend"] {
        for content in ["entry\n", ""] {
            let project = Project::new();
            let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: new.txt\n  operation: {operation}\n  input: {}\n  output: diff\n", serde_json::to_string(content).unwrap())).unwrap();
            let outputs = run_with(&workflow, &project.0, |_| panic!("no model")).unwrap();
            assert_eq!(
                fs::read_to_string(project.0.join("new.txt")).unwrap(),
                content
            );
            if !content.is_empty() {
                assert!(outputs["diff"].contains("--- /dev/null"));
            }
            let mut history = History::open(&project.log()).unwrap();
            // Simulate a crash after publication but before saving OUTPUT.
            history.steps[0].pop();
            history.save().unwrap();
            let path = history.path.clone();
            drop(history);
            let mut history = History::open(&path).unwrap();
            assert_eq!(
                continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap(),
                outputs
            );
            assert_eq!(
                fs::read_to_string(project.0.join("new.txt")).unwrap(),
                content
            );
            // Simulate a prepared creation which has not reached the filesystem yet.
            history.steps[0].pop();
            fs::remove_file(project.0.join("new.txt")).unwrap();
            continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap();
            assert_eq!(
                fs::read_to_string(project.0.join("new.txt")).unwrap(),
                content
            );
            // A different file created in between must not be replaced.
            history.steps[0].pop();
            fs::write(project.0.join("new.txt"), "other writer").unwrap();
            assert!(
                continue_with(&mut history, |_| panic!("conflict"), &mut answers(&[]))
                    .unwrap_err()
                    .contains("conflict")
            );
            assert_eq!(
                fs::read_to_string(project.0.join("new.txt")).unwrap(),
                "other writer"
            );
        }
    }
}

#[test]
fn missing_edit_targets_remain_confined_and_line_edits_require_files() {
    let project = Project::new();
    let outside = Project::new();
    for path in [
        "missing-parent/new.txt".to_owned(),
        outside.0.join("new.txt").to_str().unwrap().to_owned(),
    ] {
        let workflow: Workflow = serde_yaml::from_str(&format!(
            "version: 1\nsteps:\n- tool: EDIT\n  path: {}\n  operation: append\n  input: text\n",
            serde_json::to_string(&path).unwrap()
        ))
        .unwrap();
        assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
    }
    for operation in ["insert", "delete", "replace"] {
        let coordinates = if operation == "insert" {
            "line: 1"
        } else {
            "start: 1\n  end: 1"
        };
        let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: missing.txt\n  operation: {operation}\n  {coordinates}\n  version: '{}'\n", crate::tools::edit::version(""))).unwrap();
        assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
        assert!(!project.0.join("missing.txt").exists());
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(outside.0.join("absent"), project.0.join("dangling")).unwrap();
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: EDIT\n  path: dangling\n  operation: append\n  input: text\n").unwrap();
        assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
        assert!(!outside.0.join("absent").exists());
    }
}
#[test]
fn skipped_write_completes_and_resume_does_not_recreate_it() {
    let project = Project::new();
    fs::write(project.0.join("file"), "original").unwrap();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: WRITE\n  path: file\n  input: replacement\n  skip: true\n  output: skipped\n- tool: WRITE\n  path: next\n  input: continued\n").unwrap();
    let outputs = run_with(&workflow, &project.0, |_| panic!("no model")).unwrap();
    assert_eq!(outputs["skipped"], "");
    assert_eq!(
        fs::read_to_string(project.0.join("file")).unwrap(),
        "original"
    );
    assert_eq!(
        fs::read_to_string(project.0.join("next")).unwrap(),
        "continued"
    );
    fs::remove_file(project.0.join("file")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    assert_eq!(
        continue_with(&mut history, |_| panic!("no model"), &mut answers(&[])).unwrap(),
        outputs
    );
    assert!(!project.0.join("file").exists());
}

#[test]
fn delete_requires_confirmation_unless_forced() {
    let project = Project::new();
    fs::write(project.0.join("obsolete"), "content").unwrap();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- tool: DELETE\n  path: obsolete\n").unwrap();
    assert!(run_with(&workflow, &project.0, |_| panic!("no model")).is_err());
    assert!(project.0.join("obsolete").exists());
    run_interactive_with(
        &workflow,
        &project.0,
        |_| panic!("no model"),
        &mut answers(&[""]),
    )
    .unwrap();
    assert!(!project.0.join("obsolete").exists());

    fs::write(project.0.join("forced"), "content").unwrap();
    let forced: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- tool: DELETE\n  path: forced\n  force: true\n")
            .unwrap();
    run_with(&forced, &project.0, |_| panic!("no model")).unwrap();
    assert!(!project.0.join("forced").exists());
}

#[test]
fn agent_delete_requires_permission_and_reports_its_result() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/cleaner.md"),
        "---\nadapter: codex\nDELETE_TOOL: allow\n---\nClean generated files.",
    )
    .unwrap();
    fs::write(project.0.join("obsolete"), "content").unwrap();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: cleaner\n  output: result\n").unwrap();
    let mut calls = 0;
    let outputs = run_interactive_with(
        &workflow,
        &project.0,
        |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            match calls {
                1 => {
                    assert!(prompt.contains("External tool: DELETE"));
                    Ok("DELETE: {\"path\":\"obsolete\"}".into())
                }
                2 => {
                    assert!(prompt.contains("Tool result (DELETE)"));
                    Ok("Deleted the obsolete file.".into())
                }
                _ => panic!("unexpected model call"),
            }
        },
        &mut answers(&[""]),
    )
    .unwrap();
    assert_eq!(outputs["result"], "Deleted the obsolete file.");
    assert!(!project.0.join("obsolete").exists());
}

#[test]
fn agent_delete_force_requires_the_separate_permission() {
    let project = Project::new();
    fs::write(
            project.0.join(".agents/cleaner.md"),
            "---\nadapter: codex\nDELETE_TOOL: allow\nDELETE_WITHOUT_CONFIRM: allow\n---\nClean generated files.",
        )
        .unwrap();
    fs::write(project.0.join("obsolete"), "content").unwrap();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: cleaner\n  output: result\n").unwrap();
    let mut calls = 0;
    let outputs = run_with(&workflow, &project.0, |invocation| {
        calls += 1;
        match calls {
            1 => Ok("DELETE: {\"path\":\"obsolete\",\"force\":true}".into()),
            2 => {
                assert!(
                    invocation
                        .arguments
                        .last()
                        .unwrap()
                        .contains("Tool result (DELETE)")
                );
                Ok("Deleted without confirmation.".into())
            }
            _ => panic!("unexpected model call"),
        }
    })
    .unwrap();
    assert_eq!(outputs["result"], "Deleted without confirmation.");
    assert!(!project.0.join("obsolete").exists());
}
