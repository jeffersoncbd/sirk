//! Standalone agent-file creation, not available to workflows or model requests.
use crate::{
    adapters,
    agents::{Agent, valid_id},
    harness::RunRequest,
    input::UserInput,
    services::{BashService, Invocation},
};
use serde::Serialize;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

pub fn create(directory: &Path, input: &mut impl UserInput) -> Result<PathBuf, String> {
    create_with(directory, input, |invocation| {
        let result = BashService::default()
            .execute_streaming(invocation)
            .map_err(|e| format!("agent generation failed: {e}"))?;
        if !result.status.success() {
            return Err(format!(
                "agent generation exited with {}; no agent was saved",
                result.status
            ));
        }
        Ok(result.stdout)
    })
}

pub fn create_with(
    directory: &Path,
    input: &mut impl UserInput,
    mut generate: impl FnMut(&Invocation) -> Result<String, String>,
) -> Result<PathBuf, String> {
    let directory = directory.canonicalize().map_err(|e| e.to_string())?;
    let description = nonempty(input, "Describe the agent you want to create:")?;
    let adapter = choose_adapter(input, "Which adapter will the CREATED agent use?")?;
    let model = nonempty(input, "Which model will the CREATED agent use?")?
        .trim()
        .to_lowercase();
    let generator_adapter =
        choose_adapter(input, "Which adapter should GENERATE the agent definition?")?;
    let generator_model = nonempty(input, "Which model should GENERATE the agent definition?")?
        .trim()
        .to_lowercase();
    let agents_directory = directory.join(".agents");
    let mut name_question =
        "Name the agent (letters, digits, underscores, and hyphens; no extension):".to_owned();
    let name = loop {
        let name = input.ask(&name_question)?.trim().to_owned();
        if !valid_id(&name) {
            name_question =
                "Invalid name. Use only letters, digits, underscores, and hyphens:".into();
        } else if agents_directory
            .join(format!("{name}.md"))
            .symlink_metadata()
            .is_ok()
        {
            name_question = format!("Agent `{name}` already exists. Enter a different name:");
        } else {
            break name;
        }
    };
    #[derive(Serialize)]
    struct Header<'a> {
        adapter: &'a str,
        model: &'a str,
    }
    let metadata = serde_yaml::to_string(&Header {
        adapter: &adapter,
        model: &model,
    })
    .map_err(|e| e.to_string())?;
    let prompt = format!(
        "Create a complete Markdown agent definition from the user's description below.\n\
         Return ONLY the file content, without surrounding code fences or commentary.\n\
         Start with this exact YAML front matter:\n---\n{metadata}---\n\
         Then write detailed, actionable agent instructions: role, objective, workflow, constraints, and expected response format as appropriate to the request.\n\
         Write the instructions in the language used by the user. Do not simply copy their description.\n\
         Do not change the requested adapter or model, add unsupported metadata, or write any files yourself.\n\
         The Markdown body must be nonempty. The agent name is {name}.\n\nUser description:\n{description}"
    );
    let invocation = adapters::resolve(&generator_adapter)
        .ok_or("unsupported generator adapter")?
        .invocation(&RunRequest {
            prompt,
            working_directory: directory.clone(),
            model: Some(generator_model),
            event_stream: false,
        })
        .map_err(|e| e.to_string())?;
    let generated = generate(&invocation)?;
    let agent = Agent::parse(&name, generated.trim()).map_err(|e| {
        format!("generator returned an invalid agent definition; no file was saved: {e}")
    })?;
    if agent.adapter != adapter
        || agent.model.as_deref() != Some(model.as_str())
        || agent.json
        || agent.ask.is_some()
    {
        return Err("generator changed the requested agent metadata; no file was saved".into());
    }
    // Keep the user's selected runtime metadata canonical; preserve generated instructions.
    let content = format!("---\n{metadata}---\n\n{}\n", agent.instructions);
    fs::create_dir_all(&agents_directory)
        .map_err(|e| format!("cannot create agent directory: {e}"))?;
    let path = agents_directory.join(format!("{name}.md"));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|e| format!("cannot create agent `{}`: {e}", path.display()))?;
    file.write_all(content.as_bytes())
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("cannot write agent `{}`: {e}", path.display()))?;
    Ok(path)
}

fn choose_adapter(input: &mut impl UserInput, question: &str) -> Result<String, String> {
    let mut question = format!("{question} Available: {}", adapters::AVAILABLE.join(", "));
    loop {
        let value = input.ask(&question)?.trim().to_lowercase();
        if adapters::AVAILABLE.contains(&value.as_str()) {
            return Ok(value);
        }
        question = format!(
            "Unsupported adapter. Choose one of: {}",
            adapters::AVAILABLE.join(", ")
        );
    }
}

fn nonempty(input: &mut impl UserInput, question: &str) -> Result<String, String> {
    loop {
        let value = input.ask(question)?;
        if !value.trim().is_empty() {
            return Ok(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::VecDeque,
        time::{SystemTime, UNIX_EPOCH},
    };
    struct Answers(VecDeque<String>);
    impl UserInput for Answers {
        fn ask(&mut self, _: &str) -> Result<String, String> {
            self.0.pop_front().ok_or("cancelled".into())
        }
    }
    fn answers(values: &[&str]) -> Answers {
        Answers(values.iter().map(|s| s.to_string()).collect())
    }
    struct Project(PathBuf);
    impl Project {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "new-agent-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }
    impl Drop for Project {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn creates_loadable_agent_and_preserves_prompt_language() {
        let project = Project::new();
        let prompt = "Você deve planejar.\n\nNão escreva código.";
        let path = create_with(
            &project.0,
            &mut answers(&[
                prompt,
                "unsupported",
                "Codex",
                "GPT-6-Sol",
                "codex",
                "GPT-6-Astra",
                "../bad",
                "planner",
            ]),
            |invocation| {
                assert_eq!(invocation.program, "codex");
                assert!(invocation.arguments.windows(2).any(|pair| pair == ["--model", "gpt-6-astra"]));
                let sent = invocation.arguments.last().unwrap();
                assert!(sent.contains(prompt));
                assert!(sent.contains("model: gpt-6-sol"));
                Ok("---\nadapter: codex\nmodel: GPT-6-Sol\n---\nVocê é um planejador.\n1. Analise os requisitos.\n2. Produza um plano detalhado.".into())
            },
        )
        .unwrap();
        assert_eq!(path, project.0.join(".agents/planner.md"));
        let agent = crate::agents::Agent::load(&project.0.join(".agents"), "planner").unwrap();
        assert!(agent.instructions.contains("Produza um plano detalhado"));
        assert_ne!(agent.instructions, prompt);
        assert_eq!(agent.adapter, "codex");
        assert_eq!(agent.model.as_deref(), Some("gpt-6-sol"));
        assert!(!project.0.join("history").exists());
    }
    #[test]
    fn cancellation_and_name_collisions_preserve_existing_agents() {
        let project = Project::new();
        assert!(
            create_with(
                &project.0,
                &mut answers(&["Instructions", "codex"]),
                |_| panic!("cancelled")
            )
            .is_err()
        );
        assert!(!project.0.join(".agents").exists());
        fs::create_dir(project.0.join(".agents")).unwrap();
        fs::write(project.0.join(".agents/existing.md"), "User content").unwrap();
        create_with(
            &project.0,
            &mut answers(&[
                "Instructions",
                "codex",
                "GPT-6-Sol",
                "codex",
                "GPT-6-Astra",
                "existing",
                "new",
            ]),
            |_| Ok("---\nadapter: codex\nmodel: gpt-6-sol\n---\nGenerated instructions.".into()),
        )
        .unwrap();
        assert_eq!(
            fs::read_to_string(project.0.join(".agents/existing.md")).unwrap(),
            "User content"
        );
    }

    #[test]
    fn generation_errors_and_invalid_definitions_do_not_create_agents() {
        for generated in [
            Err("process failure".to_owned()),
            Ok(String::new()),
            Ok("Here is your agent".into()),
            Ok("---\nadapter: codex\nmodel: gpt-6-astra\n---\nWrong runtime model.".into()),
            Ok("---\nadapter: codex\nmodel: gpt-6-sol\nwrite: true\n---\nInvalid metadata.".into()),
        ] {
            let project = Project::new();
            let result = create_with(
                &project.0,
                &mut answers(&[
                    "Write tests",
                    "codex",
                    "GPT-6-Sol",
                    "codex",
                    "GPT-6-Astra",
                    "tester",
                ]),
                |_| generated.clone(),
            );
            assert!(result.is_err());
            assert!(!project.0.join(".agents").exists());
        }
    }
}
