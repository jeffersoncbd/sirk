use super::*;

#[test]
fn if_reuses_docs_or_generates_them_and_restores_loop_outputs() {
    let project = Project::new();
    fs::create_dir(project.0.join("tools")).unwrap();
    fs::write(
        project.0.join("tools/file-exists.sh"),
        include_str!("../../../tools/file-exists.sh"),
    )
    .unwrap();
    fs::write(project.0.join("cached"), "existing documentation").unwrap();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [cached, missing]\n  iter:\n  - custom-tool: file-exists\n    input: ['{{ loop.item }}']\n    output: exists\n  - tool: IF\n    input: '{{ loop.exists }}'\n    is_true:\n    - tool: READ\n      input: '{{ loop.item }}'\n      output: explain\n    is_false:\n    - agent: second\n      input: generate\n      output: explain\n    - tool: WRITE\n      path: '{{ loop.item }}'\n      input: '{{ loop.explain }}'\n  - tool: EDIT\n    path: index\n    operation: append\n    input: '{{ loop.explain }}'\n").unwrap();
    let mut calls = 0;
    let outputs = run_with(&workflow, &project.0, |_| {
        calls += 1;
        Ok("generated documentation".into())
    })
    .unwrap();
    assert_eq!(calls, 1);
    assert!(outputs.is_empty());
    assert_eq!(
        fs::read_to_string(project.0.join("index")).unwrap(),
        "existing documentationgenerated documentation"
    );
    let mut history = History::open(&project.log()).unwrap();
    assert!(
        history
            .labels
            .iter()
            .any(|s| s == "Step 1.1.2.true.1 — READ")
    );
    assert!(
        history
            .labels
            .iter()
            .any(|s| s == "Step 1.2.2.false.1 — second")
    );
    fs::remove_file(project.0.join("cached")).unwrap();
    continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap();
    assert_eq!(
        fs::read_to_string(project.0.join("index")).unwrap(),
        "existing documentationgenerated documentation"
    );
}

#[test]
fn if_recovers_pending_nested_branch_and_rejects_edited_conditions() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: selected\n- tool: IF\n  input: '{{ outputs.selected }}'\n  is_true:\n  - tool: LOOP\n    input: [a, b]\n    iter:\n    - agent: second\n      input: '{{ loop.item }}'\n  is_false:\n  - tool: WRITE\n    path: wrong-branch\n    input: wrong\n- tool: WRITE\n  path: final\n  input: done\n").unwrap();
    let mut calls = 0;
    assert!(
        run_with(&workflow, &project.0, |_| {
            calls += 1;
            match calls {
                1 => Ok("true\n".into()),
                2 => Ok("first".into()),
                _ => Err("interrupted".into()),
            }
        })
        .is_err()
    );
    let mut history = History::open(&project.log()).unwrap();
    assert!(!project.0.join("wrong-branch").exists());
    let original = history.steps[1][0].clone();
    history.steps[1][0] = Block::Input("false".into());
    assert!(
        continue_with(
            &mut history,
            |_| panic!("must validate first"),
            &mut answers(&[])
        )
        .is_err()
    );
    assert!(!project.0.join("wrong-branch").exists());
    history.steps[1][0] = original;
    let mut resumed = 0;
    continue_with(
        &mut history,
        |invocation| {
            resumed += 1;
            assert!(invocation.arguments.last().unwrap().contains("b"));
            Ok("second".into())
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(resumed, 1);
    assert_eq!(fs::read_to_string(project.0.join("final")).unwrap(), "done");
    // Truncating a branch result while retaining later records must fail preflight.
    history.steps[3].pop();
    assert!(continue_with(&mut history, |_| panic!("invalid"), &mut answers(&[])).is_err());
}

#[test]
fn if_validates_unselected_agents_and_handles_empty_selection() {
    let project = Project::new();
    let invalid: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: IF\n  input: true\n  is_true:\n  - tool: TREE\n  is_false:\n  - agent: missing\n").unwrap();
    assert!(run_with(&invalid, &project.0, |_| panic!("validate first")).is_err());
    assert!(!project.0.join("history").exists());
    let empty: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: IF\n  input: false\n  is_true:\n  - tool: READ\n    input: missing\n- tool: WRITE\n  path: done\n  input: ok\n").unwrap();
    run_with(&empty, &project.0, |_| panic!("no model")).unwrap();
    assert_eq!(fs::read_to_string(project.0.join("done")).unwrap(), "ok");
    let mut history = History::open(&project.log()).unwrap();
    continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap();
    assert!(crate::tools::request("IF: true").is_none());
    assert!(!crate::tools::supports("IF"));
}

#[test]
fn write_step_creates_a_file_and_is_not_repeated_after_resume() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- tool: WRITE\n  path: generated.txt\n  input: generated content\n  output: written\n",
        )
        .unwrap();
    let outputs = run_with(&workflow, &project.0, |_| {
        panic!("WRITE does not invoke harnesses")
    })
    .unwrap();
    assert_eq!(outputs["written"], "");
    let target = project.0.join("generated.txt");
    assert_eq!(fs::read_to_string(&target).unwrap(), "generated content");
    fs::write(&target, "changed after completion").unwrap();
    let mut history = History::open(&project.log()).unwrap();
    assert_eq!(
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap(),
        outputs
    );
    assert_eq!(
        fs::read_to_string(target).unwrap(),
        "changed after completion"
    );
}
#[test]
fn loop_reads_items_with_local_outputs_and_resumes_at_pending_iteration() {
    let project = Project::new();
    fs::write(project.0.join("a.txt"), "A").unwrap();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: '[\"a.txt\", \"b.txt\"]'\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: '{{ loop.content }}'\n  - agent: second\n    input: '{{ loop.item }}: {{ loop.content }}'\n    output: iteration_result\n- agent: second\n  input: finished\n  output: final\n").unwrap();
    let mut calls = 0;
    assert!(
        run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            assert!(invocation.arguments.last().unwrap().contains("a.txt: A"));
            Ok("first done".into())
        })
        .is_err()
    );
    assert_eq!(calls, 1);
    let path = project.log();
    let log = fs::read_to_string(&path).unwrap();
    assert!(log.contains("Step 1.2.1 — READ"));
    fs::remove_file(project.0.join("a.txt")).unwrap();
    fs::write(project.0.join("b.txt"), "B").unwrap();
    let mut history = History::open(&path).unwrap();
    let mut calls = 0;
    let outputs = continue_with(
        &mut history,
        |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            if calls == 1 {
                assert!(prompt.contains("b.txt: B"));
                assert!(!prompt.contains("a.txt: A"));
            } else {
                assert!(prompt.contains("finished"));
            }
            Ok("done".into())
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(calls, 2);
    assert_eq!(outputs, BTreeMap::from([("final".into(), "done".into())]));
    drop(history);
    // Removing a result and subsequent records regenerates only that suffix.
    let source = fs::read_to_string(&path).unwrap();
    let start = source.find("Step 1.2.2 — second").unwrap();
    let cut = start + source[start..].find("<== OUTPUT").unwrap();
    fs::write(&path, &source[..cut]).unwrap();
    fs::remove_file(project.0.join("b.txt")).unwrap();
    let mut history = History::open(&path).unwrap();
    let mut calls = 0;
    continue_with(
        &mut history,
        |_| {
            calls += 1;
            Ok("replacement".into())
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(calls, 2);
}
#[test]
fn nested_and_empty_loops_restore_outer_scope() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: []\n  iter:\n  - agent: second\n- tool: LOOP\n  input: [outer, next]\n  iter:\n  - agent: second\n    input: '{{ loop.item }}'\n    output: '{{ loop.saved }}'\n  - tool: LOOP\n    input: [inner]\n    iter:\n    - agent: second\n      input: '{{ loop.item }}'\n  - agent: second\n    input: '{{ loop.item }} / {{ loop.saved }}'\n").unwrap();
    let mut prompts = Vec::new();
    let outputs = run_with(&workflow, &project.0, |invocation| {
        prompts.push(invocation.arguments.last().unwrap().clone());
        Ok("saved".into())
    })
    .unwrap();
    assert!(outputs.is_empty());
    assert_eq!(prompts.len(), 6);
    for (prompt, value) in prompts.iter().zip([
        "outer",
        "inner",
        "outer / saved",
        "next",
        "inner",
        "next / saved",
    ]) {
        assert!(prompt.contains(&format!("User:\n{value}\n")));
    }
    let mut history = History::open(&project.log()).unwrap();
    continue_with(
        &mut history,
        |_| panic!("completed loops must not rerun"),
        &mut answers(&[]),
    )
    .unwrap();
}
#[test]
fn loop_is_not_an_agent_tool_and_checks_nested_agents_early() {
    assert!(!crate::tools::supports("LOOP"));
    assert!(crate::tools::request("LOOP").is_none());
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str(
        "version: 1\nsteps:\n- tool: LOOP\n  input: []\n  iter:\n  - agent: missing\n",
    )
    .unwrap();
    assert!(run_with(&workflow, &project.0, |_| panic!("must validate first")).is_err());
    assert!(!project.0.join("history").exists());
}
#[test]
fn loop_rejects_non_array_output_before_running_body() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: items\n- tool: LOOP\n  input: '{{ outputs.items }}'\n  iter:\n  - agent: second\n    input: '{{ loop.item }}'\n").unwrap();
    let mut calls = 0;
    let error = run_with(&workflow, &project.0, |_| {
        calls += 1;
        Ok("[\"a\", 2]".into())
    })
    .unwrap_err();
    assert!(error.contains("array of strings"));
    assert_eq!(calls, 1);
}
#[test]
fn loop_refuses_later_records_after_an_edited_pending_child() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [a, b]\n  iter:\n  - agent: second\n    input: '{{ loop.item }}'\n").unwrap();
    run_with(&workflow, &project.0, |_| Ok("Done".into())).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    history.steps[1].pop();
    assert!(
        continue_with(
            &mut history,
            |_| panic!("inconsistent history must not run"),
            &mut answers(&[])
        )
        .is_err()
    );
}
#[test]
fn plain_loop_outputs_are_local_and_restored_on_resume() {
    let project = Project::new();
    fs::write(project.0.join("a.txt"), "A").unwrap();
    fs::write(project.0.join("b.txt"), "B").unwrap();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: content\n- tool: LOOP\n  input: [a.txt, b.txt]\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: content\n  - agent: second\n    input: '{{ outputs.content }} / {{ loop.item }} / {{ loop.content }}'\n    output: explain\n").unwrap();
    let mut calls = 0;
    assert!(
        run_with(&workflow, &project.0, |invocation| {
            calls += 1;
            if calls == 1 {
                return Ok("outer".into());
            }
            assert!(
                invocation
                    .arguments
                    .last()
                    .unwrap()
                    .contains("outer / a.txt / A")
            );
            Err("interrupted".into())
        })
        .is_err()
    );
    fs::remove_file(project.0.join("a.txt")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    let mut prompts = Vec::new();
    let outputs = continue_with(
        &mut history,
        |invocation| {
            prompts.push(invocation.arguments.last().unwrap().clone());
            Ok("explanation".into())
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(prompts.len(), 2);
    assert!(prompts[0].contains("outer / a.txt / A"));
    assert!(prompts[1].contains("outer / b.txt / B"));
    assert_eq!(
        outputs,
        BTreeMap::from([("content".into(), "outer".into())])
    );
}
#[test]
fn validates_all_agents_before_input_or_execution() {
    let project = Project::new();
    fs::write(project.0.join(".agents/second.md"), "invalid").unwrap();
    assert!(
        run_interactive_with(
            &workflow(),
            &project.0,
            |_| panic!("invalid agent must not execute"),
            &mut answers(&[])
        )
        .is_err()
    );
    assert!(!project.0.join("history").exists());
}
