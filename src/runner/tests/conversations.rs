use super::*;

#[test]
fn runs_an_agent_through_its_call_prefix() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/planner.md"),
        "---\nadapter: codex\ncall_prefix: [docker, exec, -i, harness]\n---\nMake a plan.",
    )
    .unwrap();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: planner\n  output: plan\n").unwrap();

    let output = run_with(&workflow, &project.0, |invocation| {
        assert_eq!(invocation.program, "docker");
        assert_eq!(
            invocation.arguments[..5],
            ["exec", "-i", "harness", "codex", "exec"]
        );
        assert!(
            invocation
                .arguments
                .last()
                .unwrap()
                .contains("Make a plan.")
        );
        Ok("Plan".into())
    })
    .unwrap();

    assert_eq!(output["plan"], "Plan");
}
#[test]
fn tree_is_not_advertised_or_executed_without_permission() {
    let project = Project::new();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();

    let error = run_with(&workflow, &project.0, |invocation| {
        assert!(
            !invocation
                .arguments
                .last()
                .unwrap()
                .contains("Available tool: TREE")
        );
        Ok("TREE".into())
    })
    .unwrap_err();

    assert_eq!(error, "agent requested TREE_TOOL without permission");
}
#[test]
fn resumes_failed_turn_with_context_and_replays_edited_output() {
    let project = Project::new();
    let mut calls = 0;
    let result = run_interactive_with(
        &workflow(),
        &project.0,
        |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            assert!(prompt.contains("Initial context"));
            if calls == 1 {
                Ok("ASK: Which business?".into())
            } else {
                Err("process failed".into())
            }
        },
        &mut answers(&["Scheduling", "Dentist"]),
    );
    assert!(result.is_err());
    let path = project.log();
    let mut history = History::open(&path).unwrap();
    assert_eq!(
        history.steps[0].last(),
        Some(&Block::Input("Dentist".into()))
    );
    // Resumption uses the saved configuration even if source files disappear.
    fs::remove_file(project.0.join(".agents/planner.md")).unwrap();
    let mut calls = 0;
    let outputs = continue_with(
        &mut history,
        |invocation| {
            calls += 1;
            let prompt = invocation.arguments.last().unwrap();
            if calls == 1 {
                for text in ["Make a plan", "Scheduling", "Which business?", "Dentist"] {
                    assert!(prompt.contains(text));
                }
                Ok("Plan v1".into())
            } else {
                assert!(prompt.contains("Plan v1"));
                Ok("Review v1".into())
            }
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(outputs["review"], "Review v1");
    drop(history);
    let source = fs::read_to_string(&path).unwrap();
    let cut = source.find("<== OUTPUT\nPlan v1").unwrap();
    fs::write(&path, &source[..cut]).unwrap();
    let mut history = History::open(&path).unwrap();
    let mut calls = 0;
    let outputs = continue_with(
        &mut history,
        |invocation| {
            calls += 1;
            if calls == 1 {
                assert!(!invocation.arguments.last().unwrap().contains("Plan v1"));
                Ok("Plan v2".into())
            } else {
                assert!(invocation.arguments.last().unwrap().contains("Plan v2"));
                Ok("Review v2".into())
            }
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(outputs["review"], "Review v2");
    drop(history);
    let mut history = History::open(&path).unwrap();
    let outputs = continue_with(
        &mut history,
        |_| panic!("completed execution must not run again"),
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(outputs["plan"], "Plan v2");
}
#[test]
fn resumes_questions_without_repeating_model_calls() {
    let project = Project::new();
    assert!(
        run_interactive_with(
            &workflow(),
            &project.0,
            |_| panic!("must await user"),
            &mut answers(&[])
        )
        .is_err()
    );
    let mut history = History::open(&project.log()).unwrap();
    assert!(
        continue_with(
            &mut history,
            |_| Ok("ASK: Details?".into()),
            &mut answers(&["Idea"])
        )
        .is_err()
    );
    drop(history);
    let mut history = History::open(&project.log()).unwrap();
    let mut calls = 0;
    continue_with(
        &mut history,
        |_| {
            calls += 1;
            Ok("Done".into())
        },
        &mut answers(&["Details"]),
    )
    .unwrap();
    assert_eq!(calls, 2);
}

#[test]
fn user_can_answer_a_question_with_a_read_request_and_resume_after_it() {
    let project = Project::new();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: planner\n  output: plan\n").unwrap();
    fs::write(
        project.0.join("Cargo.toml"),
        "[package]\nname = \"example\"\n",
    )
    .unwrap();
    let mut calls = 0;
    assert!(
        run_interactive_with(
            &workflow,
            &project.0,
            |invocation| {
                calls += 1;
                let prompt = invocation.arguments.last().unwrap();
                assert!(prompt.contains("User:\nREAD: Cargo.toml"));
                assert!(prompt.contains("Tool result (READ):\n[package]"));
                Err("interrupted after user tool result".into())
            },
            &mut answers(&["READ: Cargo.toml"]),
        )
        .is_err()
    );
    assert_eq!(calls, 1);

    let mut history = History::open(&project.log()).unwrap();
    assert!(
        matches!(history.steps[0].last(), Some(Block::Read(result)) if result.contains("name = \"example\""))
    );
    let outputs = continue_with(
        &mut history,
        |invocation| {
            let prompt = invocation.arguments.last().unwrap();
            assert!(prompt.contains("Tool result (READ):\n[package]"));
            Ok("Plan based on Cargo.toml.".into())
        },
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(outputs["plan"], "Plan based on Cargo.toml.");
}

#[test]
fn user_can_answer_a_question_with_tree_without_agent_permission() {
    let project = Project::new();
    project.init_git();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: planner\n  output: plan\n").unwrap();

    let outputs = run_interactive_with(
        &workflow,
        &project.0,
        |invocation| {
            let prompt = invocation.arguments.last().unwrap();
            assert!(prompt.contains("User:\nTREE"));
            assert!(prompt.contains("Tool result (TREE)"));
            Ok("Plan based on the tree.".into())
        },
        &mut answers(&["TREE"]),
    )
    .unwrap();

    assert_eq!(outputs["plan"], "Plan based on the tree.");
}

#[test]
fn rejects_downstream_steps_after_deleted_response() {
    let project = Project::new();
    run_interactive_with(
        &workflow(),
        &project.0,
        |_| Ok("Done".into()),
        &mut answers(&["Idea"]),
    )
    .unwrap();
    let mut history = History::open(&project.log()).unwrap();
    history.steps[0].pop();
    assert!(
        continue_with(
            &mut history,
            |_| panic!("invalid state must not execute"),
            &mut answers(&[])
        )
        .is_err()
    );
}
#[test]
fn runs_without_input_and_keeps_inserted_templates_literal() {
    let project = Project::new();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: first\n- agent: second\n  input: '{{ outputs.first }}'\n  output: final\n").unwrap();
    let mut calls = 0;
    let outputs = run_with(&workflow, &project.0, |invocation| {
        calls += 1;
        if calls == 1 {
            Ok("literal {{ outputs.missing }}\n".into())
        } else {
            assert!(
                invocation
                    .arguments
                    .last()
                    .unwrap()
                    .contains("literal {{ outputs.missing }}")
            );
            Ok("Done".into())
        }
    })
    .unwrap();
    assert_eq!(outputs["final"], "Done");
    let mut history = History::open(&project.log()).unwrap();
    let restored = continue_with(
        &mut history,
        |_| panic!("already complete"),
        &mut answers(&[]),
    )
    .unwrap();
    assert_eq!(outputs, restored);
}
#[test]
fn tool_step_passes_named_output_and_resumes_without_rerunning_tree() {
    let project = Project::new();
    project.init_git();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: TREE\n  output: tree\n- agent: second\n  input: '{{ outputs.tree }}'\n  output: final\n").unwrap();
    assert!(
        run_with(&workflow, &project.0, |invocation| {
            let prompt = invocation.arguments.last().unwrap();
            assert!(prompt.contains("\"visible.txt\""));
            assert!(!prompt.contains("\"ignored.txt\""));
            Err("interrupted agent".into())
        })
        .is_err()
    );
    fs::remove_dir_all(project.0.join(".git")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    let outputs = continue_with(&mut history, |_| Ok("Done".into()), &mut answers(&[])).unwrap();
    assert!(outputs["tree"].contains("visible.txt"));
    assert_eq!(outputs["final"], "Done");
}
#[test]
fn agent_tree_request_and_result_survive_interruptions() {
    let project = Project::new();
    fs::write(
        project.0.join(".agents/second.md"),
        "---\nadapter: codex\nTREE_TOOL: allow\n---\nReview the plan.",
    )
    .unwrap();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();
    // The request is saved even if TREE fails (no repository yet).
    assert!(run_with(&workflow, &project.0, |_| Ok("TREE\n".into())).is_err());
    project.init_git();
    let mut history = History::open(&project.log()).unwrap();
    assert!(
        continue_with(
            &mut history,
            |invocation| {
                let prompt = invocation.arguments.last().unwrap();
                assert!(prompt.contains("Tool result (TREE)"));
                assert!(prompt.contains("\"visible.txt\""));
                Err("interrupted after tool result".into())
            },
            &mut answers(&[])
        )
        .is_err()
    );
    assert!(matches!(history.steps[0].last(), Some(Block::Tree(_))));
    drop(history);
    fs::remove_dir_all(project.0.join(".git")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    let outputs = continue_with(
        &mut history,
        |_| Ok("ASK: Continue?".into()),
        &mut answers(&[]),
    );
    assert!(outputs.is_err());
    let outputs = continue_with(
        &mut history,
        |_| Ok("Final plan".into()),
        &mut answers(&["Yes"]),
    )
    .unwrap();
    assert_eq!(outputs["final"], "Final plan");
}
#[test]
fn tree_only_workflow_needs_no_agents_and_embedded_tree_is_plain_text() {
    let project = Project::new();
    project.init_git();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- tool: TREE\n  output: tree\n").unwrap();
    fs::remove_dir_all(project.0.join(".agents")).unwrap();
    let outputs = run_with(&workflow, &project.0, |_| {
        panic!("tools do not invoke harnesses")
    })
    .unwrap();
    assert!(outputs["tree"].contains("visible.txt"));
    let project = Project::new();
    let workflow: Workflow =
        serde_yaml::from_str("version: 1\nsteps:\n- agent: second\n  output: final\n").unwrap();
    for response in ["Use TREE please", "```\nTREE\n```", "TREE: details", "tree"] {
        let outputs = run_with(&workflow, &project.0, |_| Ok(response.into())).unwrap();
        assert_eq!(outputs["final"], response);
    }
}
#[test]
fn read_step_passes_verbatim_contents_and_restores_saved_result() {
    let project = Project::new();
    let content = "first\r\n==> READ\n<== OUTPUT\nlast\n\n";
    fs::write(project.0.join("a file.txt"), content).unwrap();
    let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: READ\n  input: a file.txt\n  output: file\n- agent: second\n  input: '{{ outputs.file }}'\n  output: final\n").unwrap();
    let outputs = run_with(&workflow, &project.0, |invocation| {
        assert!(invocation.arguments.last().unwrap().contains(content));
        Ok("Done".into())
    })
    .unwrap();
    assert_eq!(outputs["file"], content);
    fs::remove_file(project.0.join("a file.txt")).unwrap();
    let mut history = History::open(&project.log()).unwrap();
    assert_eq!(
        outputs,
        continue_with(&mut history, |_| panic!("completed"), &mut answers(&[])).unwrap()
    );
}
