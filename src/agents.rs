mod ask_permission;
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
    #[serde(default, rename = "ASK_TOOL")]
    pub ask_tool: bool,
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
        rename = "ASK_TOOL",
        deserialize_with = "ask_permission::deserialize"
    )]
    ask_tool: bool,
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
            ask_tool: metadata.ask_tool,
            edit_tool: metadata.edit_tool,
            delete_tool: metadata.delete_tool,
            delete_without_confirm: metadata.delete_without_confirm,
        })
    }
}

#[cfg(test)]
#[path = "agents/tests.rs"]
mod tests;
