mod call_prefix;
mod delete_permission;
mod edit_permission;
mod id;
mod load;
mod model;
mod tree_default;
mod tree_permission;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub adapter: String,
    pub instructions: String,
    #[serde(default, deserialize_with = "model::deserialize")]
    pub model: Option<String>,
    #[serde(default, deserialize_with = "call_prefix::deserialize")]
    pub call_prefix: Vec<String>,
    #[serde(default = "tree_default::allow", rename = "TREE_TOOL")]
    pub tree_tool: bool,
    pub json: bool,
    pub ask: Option<String>,
    #[serde(default, rename = "EDIT_TOOL")]
    pub edit_tool: bool,
    #[serde(default, rename = "DELETE_TOOL")]
    pub delete_tool: bool,
    #[serde(default, rename = "DELETE_WITHOUT_CONFIRM")]
    pub delete_without_confirm: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Metadata {
    adapter: String,
    #[serde(default, deserialize_with = "model::deserialize")]
    model: Option<String>,
    #[serde(default, deserialize_with = "call_prefix::deserialize")]
    call_prefix: Vec<String>,
    #[serde(
        default,
        rename = "TREE_TOOL",
        deserialize_with = "tree_permission::deserialize"
    )]
    tree_tool: bool,
    #[serde(default)]
    json: bool,
    ask: Option<String>,
    #[serde(
        default,
        rename = "EDIT_TOOL",
        deserialize_with = "edit_permission::deserialize"
    )]
    edit_tool: bool,
    #[serde(
        default,
        rename = "DELETE_TOOL",
        deserialize_with = "delete_permission::deserialize"
    )]
    delete_tool: bool,
    #[serde(
        default,
        rename = "DELETE_WITHOUT_CONFIRM",
        deserialize_with = "delete_permission::deserialize"
    )]
    delete_without_confirm: bool,
}

pub use id::valid_id;

impl Agent {
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
        if metadata.delete_without_confirm && !metadata.delete_tool {
            return Err("`DELETE_WITHOUT_CONFIRM` requires `DELETE_TOOL: allow`".to_owned());
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
            call_prefix: metadata.call_prefix,
            tree_tool: metadata.tree_tool,
            json: metadata.json,
            ask: metadata.ask,
            edit_tool: metadata.edit_tool,
            delete_tool: metadata.delete_tool,
            delete_without_confirm: metadata.delete_without_confirm,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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
        assert!(agent.call_prefix.is_empty());
        assert!(!agent.tree_tool);
        assert!(!agent.json);
        assert!(!agent.edit_tool);
        assert!(!agent.delete_tool);
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
    fn reads_call_prefix_as_one_token_or_an_argument_list() {
        let single = Agent::parse(
            "containerized",
            "---\nadapter: codex\ncall_prefix: wrapper\n---\nReview.",
        )
        .unwrap();
        assert_eq!(single.call_prefix, ["wrapper"]);

        let docker = Agent::parse(
            "containerized",
            "---\nadapter: codex\ncall_prefix: [docker, exec, -i, harness]\n---\nReview.",
        )
        .unwrap();
        assert_eq!(docker.call_prefix, ["docker", "exec", "-i", "harness"]);
        assert!(
            serde_yaml::to_string(&docker)
                .unwrap()
                .contains("call_prefix:\n- docker\n- exec")
        );
    }

    #[test]
    fn enables_tree_only_when_explicitly_allowed() {
        let agent = Agent::parse(
            "explorer",
            "---\nadapter: codex\nTREE_TOOL: allow\n---\nInspect the project.",
        )
        .unwrap();
        assert!(agent.tree_tool);
        assert!(
            serde_yaml::to_string(&agent)
                .unwrap()
                .contains("TREE_TOOL: true")
        );
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
            "---\nadapter: codex\nDELETE_TOOL: deny\n---\nReview",
            "---\nadapter: codex\nDELETE_WITHOUT_CONFIRM: allow\n---\nReview",
            "---\nadapter: codex\nTREE_TOOL: deny\n---\nReview",
            "---\nadapter: codex\ncall_prefix: [docker, '']\n---\nReview",
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

    #[test]
    fn enables_delete_permissions_only_when_explicitly_allowed() {
        let agent = Agent::parse(
            "cleaner",
            "---\nadapter: codex\nDELETE_TOOL: allow\nDELETE_WITHOUT_CONFIRM: allow\n---\nClean generated files.",
        )
        .unwrap();
        assert!(agent.delete_tool);
        assert!(agent.delete_without_confirm);
    }
}
