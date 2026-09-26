use super::*;

#[test]
fn enumerated_read_preserves_raw_version_for_edit_and_resume() {
    let project = Project::new();
    fs::write(project.0.join("document.md"), "before\nobsolete\n").unwrap();
    let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- tool: READ\n  input: document.md\n  enumerate: true\n  output: document\n  version-output: revision\n- tool: EDIT\n  path: document.md\n  operation: replace\n  start: 2\n  end: 2\n  version: '{{ outputs.revision }}'\n  input: |\n    current\n",
        )
        .unwrap();
    let outputs = run_with(&workflow, &project.0, |_| {
        panic!("tools do not invoke harnesses")
    })
    .unwrap();
    assert_eq!(
        outputs["document"],
        "Line | Content\n1 | before\n2 | obsolete\n"
    );
    assert_eq!(
        outputs["revision"],
        crate::tools::edit::version("before\nobsolete\n")
    );
    assert_eq!(
        fs::read_to_string(project.0.join("document.md")).unwrap(),
        "before\ncurrent\n"
    );
    let mut history = History::open(&project.log()).unwrap();
    assert_eq!(
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap(),
        outputs
    );
}

#[test]
fn agent_edit_tool_gets_numbered_read_results() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/second.md"),
        "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
    )
    .unwrap();
    fs::write(project.0.join("document.md"), "before\nobsolete\n").unwrap();
    let workflow: Workflow = serde_yaml::from_str(
        "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
    )
    .unwrap();
    let mut calls = 0;
    let outputs = run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            match calls {
                1 => {
                    assert!(prompt.contains("Before a coordinate-based edit, request READ"));
                    Ok("READ: document.md".into())
                }
                2 => {
                    assert!(prompt.contains("Line | Content\n1 | before\n2 | obsolete\n"));
                    Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":2,\"end\":2,\"input\":\"current\\n\"}".into())
                }
                3 => Ok("Updated the document.".into()),
                _ => panic!("unexpected agent invocation"),
            }
        })
        .unwrap();
    assert_eq!(outputs["result"], "Updated the document.");
    assert_eq!(
        fs::read_to_string(project.0.join("document.md")).unwrap(),
        "before\ncurrent\n"
    );
}

#[test]
fn agent_edit_tool_applies_one_external_edit_and_resumes_without_repeating_it() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/second.md"),
        "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
    )
    .unwrap();
    fs::write(project.0.join("document.md"), "before\nobsolete\nafter\n").unwrap();
    let workflow: Workflow = serde_yaml::from_str(
        "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
    )
    .unwrap();
    let mut calls = 0;
    assert!(run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            if calls == 1 {
                assert!(prompt.contains("External tool: EDIT"));
                Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":2,\"end\":2,\"input\":\"current\\n\"}".into())
            } else {
                assert!(prompt.contains("Tool result (EDIT)"));
                assert!(prompt.contains("+current"));
                Err("interrupted after edit".into())
            }
        })
        .is_err());
    assert_eq!(
        fs::read_to_string(project.0.join("document.md")).unwrap(),
        "before\ncurrent\nafter\n"
    );
    let mut history = History::open(&project.log()).unwrap();
    let output = continue_with(
        &mut history,
        |invocation| {
            assert!(invocation.arguments.last().unwrap().contains("+current"));
            Ok("Updated only the obsolete line.".into())
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(output["result"], "Updated only the obsolete line.");
    assert_eq!(
        fs::read_to_string(project.0.join("document.md")).unwrap(),
        "before\ncurrent\nafter\n"
    );
}

#[test]
fn agent_edit_tool_ignores_a_repeated_completed_request() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/second.md"),
        "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
    )
    .unwrap();
    fs::write(project.0.join("document.md"), "before\nobsolete\nafter\n").unwrap();
    let workflow: Workflow = serde_yaml::from_str(
        "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
    )
    .unwrap();
    let request = "EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":2,\"end\":2,\"input\":\"current\\n\"}";
    let mut calls = 0;
    let output = run_with(&workflow, &project.0, |invocation| {
        calls += 1;
        match calls {
            1 | 2 => Ok(request.into()),
            3 => {
                assert!(
                    invocation
                        .arguments
                        .last()
                        .unwrap()
                        .contains(DUPLICATE_EDIT_RESULT)
                );
                Ok("Updated only the obsolete line.".into())
            }
            _ => panic!("unexpected agent invocation"),
        }
    })
    .unwrap();
    assert_eq!(output["result"], "Updated only the obsolete line.");
    assert_eq!(
        fs::read_to_string(project.0.join("document.md")).unwrap(),
        "before\ncurrent\nafter\n"
    );
    let mut history = History::open(&project.log()).unwrap();
    continue_with(
        &mut history,
        |_| panic!("completed agent turn must not run again"),
        &mut answers(&[]),
    )
    .unwrap();
}

#[test]
fn agent_edit_tool_returns_invalid_coordinates_for_correction() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/second.md"),
        "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate the supplied document.",
    )
    .unwrap();
    fs::write(project.0.join("document.md"), "before\nobsolete\n").unwrap();
    let workflow: Workflow = serde_yaml::from_str(
        "version: 1\nsteps:\n- agent: second\n  input: Update document.md\n  output: result\n",
    )
    .unwrap();
    let mut calls = 0;
    let output = run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            match calls {
                1 => Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":1,\"end\":3,\"input\":\"current\\n\"}".into()),
                2 => {
                    let prompt = invocation.arguments.last().unwrap();
                    assert!(prompt.contains(EDIT_FAILURE_PREFIX));
                    assert!(prompt.contains("range is beyond EOF"));
                    assert!(prompt.contains("`document.md` has 2 lines"));
                    Ok("EDIT:\n{\"path\":\"document.md\",\"operation\":\"replace\",\"start\":1,\"end\":2,\"input\":\"current\\n\"}".into())
                }
                3 => Ok("Updated the document.".into()),
                _ => panic!("unexpected agent invocation"),
            }
        })
        .unwrap();
    assert_eq!(output["result"], "Updated the document.");
    assert_eq!(
        fs::read_to_string(project.0.join("document.md")).unwrap(),
        "current\n"
    );
    let mut history = History::open(&project.log()).unwrap();
    continue_with(
        &mut history,
        |_| panic!("completed agent turn must not run again"),
        &mut answers(&[]),
    )
    .unwrap();
}

#[test]
fn await_pauses_once_and_is_complete_after_resume() {
    let project = Project::new();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- tool: AWAIT\n  output: confirmed\n").unwrap();
    let output = run_interactive_with(
        &workflow,
        &project.0,
        |_| panic!("AWAIT does not invoke an agent"),
        &mut answers(&[""]),
    )
    .unwrap();
    assert_eq!(output["confirmed"], "");
    let mut history = History::open(&project.log()).unwrap();
    let output = continue_with(
        &mut history,
        |_| panic!("completed AWAIT does not invoke an agent"),
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(output["confirmed"], "");
}

#[test]
fn ask_step_saves_its_answer_and_reuses_it_after_resume() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- tool: ASK\n  input: Which database?\n  output: database\n- tool: WRITE\n  path: selection.txt\n  input: '{{ outputs.database }}'\n",
        )
        .unwrap();

    let output = run_interactive_with(
        &workflow,
        &project.0,
        |_| panic!("ASK does not invoke an agent"),
        &mut answers(&["PostgreSQL"]),
    )
    .unwrap();
    assert_eq!(output["database"], "PostgreSQL");
    assert_eq!(
        fs::read_to_string(project.0.join("selection.txt")).unwrap(),
        "PostgreSQL"
    );

    let mut history = History::open(&project.log()).unwrap();
    assert_eq!(
        history.steps[0],
        vec![
            Block::Ask("Which database?".into()),
            Block::Input("PostgreSQL".into())
        ]
    );
    let output = continue_with(
        &mut history,
        |_| panic!("completed ASK does not invoke an agent"),
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(output["database"], "PostgreSQL");
}

#[test]
fn noninteractive_run_rejects_an_ask_step() {
    let project = Project::new();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- tool: ASK\n  input: Which database?\n")
            .unwrap();
    assert_eq!(
        run_with(&workflow, &project.0, |_| panic!(
            "ASK does not invoke an agent"
        ))
        .unwrap_err(),
        "this execution requires user input"
    );
}

#[test]
fn read_request_recovers_failure_and_reuses_empty_file_result() {
    let project = Project::new();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();
    assert!(run_with(&workflow, &project.0, |_| Ok("READ: missing.txt".into())).is_err());
    fs::write(project.0.join("missing.txt"), "").unwrap();
    let mut history = History::open(&project.log()).unwrap();
    assert!(
        continue_with(
            &mut history,
            |invocation| {
                assert!(
                    invocation
                        .arguments
                        .last()
                        .unwrap()
                        .contains("Tool result (READ)")
                );
                Err("interrupted".into())
            },
            &mut answers(&[])
        )
        .is_err()
    );
    drop(history);
    fs::remove_file(project.0.join("missing.txt")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    assert!(matches!(history.steps[0].last(), Some(Block::Read(text)) if text.is_empty()));
    assert_eq!(
        continue_with(&mut history, |_| Ok("Done".into()), &mut answers(&[])).unwrap()["final"],
        "Done"
    );
}
#[test]
fn read_rejects_directories_binary_files_and_outside_symlinks() {
    let project = Project::new();
    assert!(crate::tools::read::read(&project.0, "").is_err());
    assert!(crate::tools::read::read(&project.0, ".agents").is_err());
    fs::write(project.0.join("binary"), [0xff]).unwrap();
    assert!(crate::tools::read::read(&project.0, "binary").is_err());
    let outside = Project::new();
    let path = outside.0.join(".agents/second.md");
    assert!(crate::tools::read::read(&project.0, path.to_str().unwrap()).is_err());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(path, project.0.join("external")).unwrap();
        assert!(crate::tools::read::read(&project.0, "external").is_err());
    }
}
#[test]
fn custom_tool_passes_list_items_as_arguments_and_restores_its_output() {
    let project = Project::new();
    fs::create_dir(project.0.join("tools")).unwrap();
    fs::write(
        project.0.join("tools/combine.sh"),
        "printf '%s|%s|%s' \"$1\" \"$2\" \"$PWD\"\n",
    )
    .unwrap();
    let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- custom-tool: combine\n  input:\n  - spaces and 'quotes'\n  - '$(exit 19); `exit 20`'\n  output: combined\n- agent: second\n  input: '{{ outputs.combined }}'\n  output: final\n",
        )
        .unwrap();
    let outputs = run_with(&workflow, &project.0, |invocation| {
        let prompt = invocation.arguments.last().unwrap();
        assert!(prompt.contains("spaces and 'quotes'|$(exit 19); `exit 20`|"));
        assert!(prompt.contains(project.0.to_str().unwrap()));
        Ok("Done".into())
    })
    .unwrap();
    assert_eq!(
        outputs["combined"],
        format!(
            "spaces and 'quotes'|$(exit 19); `exit 20`|{}",
            project.0.display()
        )
    );
    fs::remove_file(project.0.join("tools/combine.sh")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    assert_eq!(
        outputs,
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap()
    );
}
#[test]
fn read_version_and_edit_work_in_loop_scopes_and_resume() {
    let project = Project::new();
    fs::create_dir(project.0.join("tools")).unwrap();
    fs::write(project.0.join("tools/line.sh"), "printf ' 4\\n'\n").unwrap();
    for name in ["a", "b"] {
        fs::write(project.0.join(name), "fn a() {\n}\nfn b() {\n}\n").unwrap();
    }
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [a, b]\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: source\n    version-output: revision\n  - custom-tool: line\n    input: []\n    output: target\n  - tool: EDIT\n    path: '{{ loop.item }}'\n    operation: replace\n    start: '{{ loop.target }}'\n    end: '{{ loop.target }}'\n    version: '{{ loop.revision }}'\n    input: \"  // literal {{ loop.source }}\\n}\\n\"\n    output: diff\n").unwrap();
    // Inserted source contains no templates; repeated braces select only line 4.
    let result = run_with(&workflow, &project.0, |_| panic!("no model")).unwrap();
    assert!(result.is_empty());
    for name in ["a", "b"] {
        let content = fs::read_to_string(project.0.join(name)).unwrap();
        assert!(content.starts_with("fn a() {\n}\nfn b() {\n  // literal"));
    }
    fs::remove_file(project.0.join("tools/line.sh")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    fs::remove_file(project.0.join("a")).unwrap();
    assert!(
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[]))
            .unwrap()
            .is_empty()
    );
}
