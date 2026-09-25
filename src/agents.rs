use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub adapter: String,
    pub instructions: String,
    #[serde(default, deserialize_with = "deserialize_model")]
    pub model: Option<String>,
    pub json: bool,
    pub ask: Option<String>,
    #[serde(default, rename = "EDIT_TOOL")]
    pub edit_tool: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    adapter: String,
    #[serde(default, deserialize_with = "deserialize_model")]
    model: Option<String>,
    #[serde(default)]
    json: bool,
    ask: Option<String>,
    #[serde(
        default,
        rename = "EDIT_TOOL",
        deserialize_with = "deserialize_edit_tool"
    )]
    edit_tool: bool,
}

fn deserialize_edit_tool<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = String::deserialize(deserializer)?;
    if value == "allow" {
        Ok(true)
    } else {
        Err(serde::de::Error::custom("`EDIT_TOOL` must be `allow`"))
    }
}

fn deserialize_model<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    Ok(Option::<String>::deserialize(deserializer)?.map(|model| model.trim().to_lowercase()))
}

pub fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
}

impl Agent {
    pub fn load(directory: &Path, id: &str) -> Result<Self, String> {
        if !valid_id(id) {
            return Err(format!(
                "invalid agent name `{id}`; use letters, digits, `_` or `-`"
            ));
        }
        let path = directory.join(format!("{id}.md"));
        let source = fs::read_to_string(&path).map_err(|error| {
            format!(
                "could not load agent `{id}` from `{}`: {error}",
                path.display()
            )
        })?;
        Self::parse(id, &source)
            .map_err(|error| format!("invalid agent file `{}`: {error}", path.display()))
    }

    pub(crate) fn parse(id: &str, source: &str) -> Result<Self, String> {
        let mut lines = source.lines();
        if lines.next() != Some("---") {
            return Err("expected YAML front matter starting with `---`".to_owned());
        }
        let mut metadata = String::new();
        let mut closed = false;
        for line in lines.by_ref() {
            if line == "---" {
                closed = true;
                break;
            }
            metadata.push_str(line);
            metadata.push('\n');
        }
        if !closed {
            return Err("missing closing `---` for YAML front matter".to_owned());
        }
        let metadata: Metadata = serde_yaml::from_str(&metadata)
            .map_err(|error| format!("invalid metadata: {error}"))?;
        if metadata.adapter.trim().is_empty() {
            return Err("`adapter` cannot be empty".to_owned());
        }
        if metadata.json {
            return Err(
                "`json: true` is not supported until event normalization exists".to_owned(),
            );
        }
        let instructions = lines.collect::<Vec<_>>().join("\n").trim().to_owned();
        if instructions.is_empty() {
            return Err("Markdown instructions cannot be empty".to_owned());
        }
        Ok(Self {
            id: id.to_owned(),
            adapter: metadata.adapter,
            instructions,
            model: metadata.model,
            json: metadata.json,
            ask: metadata.ask,
            edit_tool: metadata.edit_tool,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_metadata_and_markdown_with_crlf() {
        let agent = Agent::parse(
            "reviewer",
            "---\r\nadapter: codex\r\nmodel: custom\r\n---\r\n# Review\r\n\r\nBe careful.\r\n",
        )
        .unwrap();
        assert_eq!(agent.id, "reviewer");
        assert_eq!(agent.instructions, "# Review\n\nBe careful.");
        assert_eq!(agent.model.as_deref(), Some("custom"));
        assert!(!agent.json);
        assert!(!agent.edit_tool);
    }

    #[test]
    fn normalizes_models_from_markdown_and_saved_snapshots() {
        let agent = Agent::parse(
            "planner",
            "---\nadapter: codex\nmodel: ' GPT-6-Astra '\n---\nPlan.",
        )
        .unwrap();
        assert_eq!(agent.model.as_deref(), Some("gpt-6-astra"));
        let mut snapshot = serde_yaml::to_string(&agent).unwrap();
        snapshot = snapshot.replace("gpt-6-astra", "GPT-6-Astra");
        let restored: Agent = serde_yaml::from_str(&snapshot).unwrap();
        assert_eq!(restored.model.as_deref(), Some("gpt-6-astra"));
    }
    #[test]
    fn rejects_malformed_definitions_and_paths() {
        for source in [
            "No header",
            "---\nadapter: codex",
            "---\nadapter: codex\n---",
            "---\nadapter: codex\nunknown: true\n---\nReview",
            "---\nadapter: codex\nwrite: true\n---\nReview",
            "---\nadapter: codex\njson: true\n---\nReview",
            "---\nadapter: codex\nEDIT_TOOL: deny\n---\nReview",
        ] {
            assert!(Agent::parse("reviewer", source).is_err());
        }
        for id in ["", "../outside", "/tmp/agent", "agent.md"] {
            assert!(Agent::load(Path::new(".agents"), id).is_err());
        }
    }

    #[test]
    fn enables_the_external_edit_tool_only_when_allowed() {
        let agent = Agent::parse(
            "editor",
            "---\nadapter: codex\nEDIT_TOOL: allow\n---\nUpdate documentation.",
        )
        .unwrap();
        assert!(agent.edit_tool);
        assert!(
            serde_yaml::to_string(&agent)
                .unwrap()
                .contains("EDIT_TOOL: true")
        );
    }
}
