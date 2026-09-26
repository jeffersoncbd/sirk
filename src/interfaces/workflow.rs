use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Workflow {
    pub version: u8,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(
        default,
        rename = "custom-tool",
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_tool: Option<String>,
    #[serde(default)]
    pub input: StepInput,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<crate::tools::edit::Operation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<EditCoordinate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<EditCoordinate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<EditCoordinate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        default,
        rename = "version-output",
        skip_serializing_if = "Option::is_none"
    )]
    pub version_output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enumerate: Option<bool>,
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub iter: Vec<Step>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub is_true: Vec<Step>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub is_false: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum EditCoordinate {
    Number(usize),
    Template(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StepInput {
    Text(String),
    Array(Vec<String>),
    Bool(bool),
}

mod default;
