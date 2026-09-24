use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

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
    pub agent: String,
    #[serde(default)]
    pub input: String,
    pub output: Option<String>,
}

impl Workflow {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let source = fs::read_to_string(path)
            .map_err(|error| format!("could not read workflow `{}`: {error}", path.display()))?;
        let workflow: Self = serde_yaml::from_str(&source)
            .map_err(|error| format!("invalid workflow `{}`: {error}", path.display()))?;
        workflow.validate()?;
        Ok(workflow)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err(format!(
                "unsupported workflow version `{}` (expected 1)",
                self.version
            ));
        }
        if self.steps.is_empty() {
            return Err("a workflow requires at least one step".to_owned());
        }
        let mut outputs = BTreeMap::new();
        for (index, step) in self.steps.iter().enumerate() {
            if !crate::agents::valid_id(&step.agent) {
                return Err(format!(
                    "step references invalid agent name `{}`",
                    step.agent
                ));
            }
            render_input(&step.input, &outputs)
                .map_err(|error| format!("step {}: {error}", index + 1))?;
            if let Some(name) = &step.output {
                if !valid_output_name(name) {
                    return Err(format!(
                        "invalid output name `{name}`; use letters, digits, `_` or `-`"
                    ));
                }
                if outputs.insert(name.clone(), String::new()).is_some() {
                    return Err(format!("output `{name}` is declared more than once"));
                }
            }
        }
        Ok(())
    }
}

fn valid_output_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

pub fn render_input(template: &str, outputs: &BTreeMap<String, String>) -> Result<String, String> {
    let mut rendered = String::new();
    let mut remaining = template;
    while let Some(start) = remaining.find("{{") {
        rendered.push_str(&remaining[..start]);
        let placeholder = &remaining[start + 2..];
        let end = placeholder
            .find("}}")
            .ok_or_else(|| "unclosed output reference; use `{{ outputs.name }}`".to_owned())?;
        let expression = placeholder[..end].trim();
        let key = expression
            .strip_prefix("outputs.")
            .filter(|key| valid_output_name(key))
            .ok_or_else(|| {
                format!("invalid reference `{{{{ {expression} }}}}`; use `{{{{ outputs.name }}}}`")
            })?;
        let output = outputs
            .get(key)
            .ok_or_else(|| format!("output `{key}` is not available yet"))?;
        rendered.push_str(output);
        remaining = &placeholder[end + 2..];
    }
    rendered.push_str(remaining);
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injects_a_previous_step_output() {
        let outputs = BTreeMap::from([("analysis".to_owned(), "found one bug".to_owned())]);
        assert_eq!(
            render_input("Fix this: {{ outputs.analysis }}", &outputs).unwrap(),
            "Fix this: found one bug"
        );
    }
}
