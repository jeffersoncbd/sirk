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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[serde(default, deserialize_with = "deserialize_input")]
    pub input: String,
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub iter: Vec<Step>,
}

fn deserialize_input<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Input {
        Text(String),
        Array(Vec<String>),
    }
    match Input::deserialize(deserializer)? {
        Input::Text(text) => Ok(text),
        Input::Array(items) => serde_json::to_string(&items).map_err(serde::de::Error::custom),
    }
}

pub fn loop_items(input: &str) -> Result<Vec<String>, String> {
    serde_json::from_str(input)
        .map_err(|error| format!("LOOP requires a JSON array of strings: {error}"))
}

pub fn loop_target(output: &str) -> Option<&str> {
    output
        .trim()
        .strip_prefix("{{")?
        .strip_suffix("}}")?
        .trim()
        .strip_prefix("loop.")
        .filter(|name| valid_output_name(name))
}

impl Step {
    pub fn name(&self) -> &str {
        self.agent
            .as_deref()
            .or(self.tool.as_deref())
            .unwrap_or("invalid")
    }
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
        validate_steps(&self.steps, &mut BTreeMap::new(), None)
    }
}

fn validate_steps(
    steps: &[Step],
    outputs: &mut BTreeMap<String, String>,
    mut locals: Option<&mut BTreeMap<String, String>>,
) -> Result<(), String> {
    for (index, step) in steps.iter().enumerate() {
        let is_loop = step.tool.as_deref() == Some("LOOP");
        if !is_loop && !step.iter.is_empty() {
            return Err("iter is only accepted on LOOP steps".into());
        }
        match (&step.agent, &step.tool) {
            (Some(agent), None) if crate::agents::valid_id(agent) => (),
            (None, Some(tool)) if tool == "LOOP" => {
                if step.iter.is_empty() {
                    return Err("LOOP requires a nonempty iter list".into());
                }
                if step.output.is_some() {
                    return Err("LOOP has no aggregate output; store results inside iter".into());
                }
                if !step.input.contains("{{") {
                    loop_items(&step.input)?;
                }
            }
            (None, Some(tool)) if crate::tools::supports(tool) => {
                if tool == "TREE" && !step.input.is_empty() {
                    return Err(format!("step {}: TREE does not accept input", index + 1));
                }
                if tool == "READ" && step.input.trim().is_empty() {
                    return Err(format!(
                        "step {}: READ requires a file path in input",
                        index + 1
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "step {} must specify either a valid agent or a supported tool (TREE, READ, LOOP)",
                    index + 1
                ));
            }
        }
        render_scoped(&step.input, outputs, locals.as_deref())
            .map_err(|error| format!("step {}: {error}", index + 1))?;
        if is_loop {
            let mut child_outputs = outputs.clone();
            let mut child_locals = BTreeMap::from([("item".into(), String::new())]);
            validate_steps(&step.iter, &mut child_outputs, Some(&mut child_locals))?;
        }
        if let Some(name) = &step.output {
            if let Some(key) = loop_target(name) {
                if key == "item" {
                    return Err("loop.item is read-only".into());
                }
                let values = locals
                    .as_deref_mut()
                    .ok_or("loop outputs are only available inside iter")?;
                values.insert(key.into(), String::new());
                continue;
            }
            if !valid_output_name(name) {
                return Err(format!(
                    "invalid output name `{name}`; use letters, digits, `_` or `-`"
                ));
            }
            if let Some(values) = locals.as_deref_mut() {
                if name == "item" {
                    return Err("loop.item is read-only".into());
                }
                values.insert(name.clone(), String::new());
                continue;
            }
            if outputs.insert(name.clone(), String::new()).is_some() {
                return Err(format!("output `{name}` is declared more than once"));
            }
        }
    }
    Ok(())
}

fn valid_output_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
}

pub fn render_input(template: &str, outputs: &BTreeMap<String, String>) -> Result<String, String> {
    render_scoped(template, outputs, None)
}

pub fn render_scoped(
    template: &str,
    outputs: &BTreeMap<String, String>,
    locals: Option<&BTreeMap<String, String>>,
) -> Result<String, String> {
    let mut rendered = String::new();
    let mut remaining = template;
    while let Some(start) = remaining.find("{{") {
        rendered.push_str(&remaining[..start]);
        let placeholder = &remaining[start + 2..];
        let end = placeholder
            .find("}}")
            .ok_or_else(|| "unclosed output reference; use `{{ outputs.name }}`".to_owned())?;
        let expression = placeholder[..end].trim();
        let (prefix, values) = if expression.starts_with("loop.") {
            (
                "loop.",
                locals.ok_or("loop variables are only available inside iter")?,
            )
        } else {
            ("outputs.", outputs)
        };
        let key = expression
            .strip_prefix(prefix)
            .filter(|key| valid_output_name(key))
            .ok_or_else(|| {
                format!("invalid reference `{{{{ {expression} }}}}`; use `{{{{ outputs.name }}}}`")
            })?;
        let output = values
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
    fn loop_scopes_validate_and_reject_invalid_arrays() {
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: LOOP\n  input: [a, b]\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: '{{ loop.content }}'\n  - agent: planner\n    input: '{{ loop.content }}'\n").unwrap();
        workflow.validate().unwrap();
        for value in [
            "{}",
            "null",
            "1",
            "\"hello\"",
            "[1]",
            "[\"a\", null]",
            "[[\"a\"]]",
        ] {
            assert!(loop_items(value).is_err(), "{value}");
        }
        assert!(loop_items("[]").unwrap().is_empty());
        for step in [
            "tool: READ\n  input: '{{ loop.item }}'",
            "agent: planner\n  output: '{{ loop.content }}'",
            "tool: LOOP\n  input: '[]'",
            "tool: LOOP\n  input: '[1]'\n  iter:\n  - agent: planner",
            "tool: LOOP\n  input: '[]'\n  iter:\n  - agent: planner\n    output: '{{ loop.item }}'",
            "tool: LOOP\n  input: '[]'\n  iter:\n  - agent: planner\n    input: '{{ loop.missing }}'",
            "tool: READ\n  input: file\n  iter:\n  - agent: planner",
        ] {
            let parsed: Workflow =
                serde_yaml::from_str(&format!("version: 1\nsteps:\n- {step}\n")).unwrap();
            assert!(parsed.validate().is_err(), "{step}");
        }
        let mut outside = workflow;
        outside
            .steps
            .push(serde_yaml::from_str("agent: planner\ninput: '{{ loop.content }}'").unwrap());
        assert!(outside.validate().is_err());
        assert!(
            serde_yaml::from_str::<Workflow>(
                "version: 1\nsteps:\n- tool: LOOP\n  input: [a, 1]\n  iter:\n  - agent: planner\n"
            )
            .is_err()
        );
    }
    #[test]
    fn plain_loop_outputs_do_not_escape_and_item_cannot_be_overwritten() {
        let base = "version: 1\nsteps:\n- tool: LOOP\n  input: [a]\n  iter:\n  - tool: READ\n    input: '{{ loop.item }}'\n    output: content\n  - agent: planner\n    input: '{{ loop.content }}'\n";
        serde_yaml::from_str::<Workflow>(base)
            .unwrap()
            .validate()
            .unwrap();
        for suffix in [
            "- agent: planner\n  input: '{{ loop.content }}'\n",
            "- agent: planner\n  input: '{{ outputs.content }}'\n",
        ] {
            assert!(
                serde_yaml::from_str::<Workflow>(&format!("{base}{suffix}"))
                    .unwrap()
                    .validate()
                    .is_err()
            );
        }
        assert!(
            serde_yaml::from_str::<Workflow>(&base.replace("output: content", "output: item"))
                .unwrap()
                .validate()
                .is_err()
        );
    }
    #[test]
    fn validates_tool_steps_and_output_references() {
        let workflow: Workflow = serde_yaml::from_str("version: 1\nsteps:\n- tool: TREE\n  output: tree\n- agent: planner\n  input: '{{ outputs.tree }}'\n").unwrap();
        workflow.validate().unwrap();
        for step in [
            "tool: UNKNOWN",
            "tool: tree",
            "agent: planner\n  tool: TREE",
            "input: hello",
            "tool: TREE\n  input: unsupported",
            "tool: READ",
            "tool: READ\n  input: ' '",
        ] {
            let workflow: Workflow =
                serde_yaml::from_str(&format!("version: 1\nsteps:\n- {step}\n")).unwrap();
            assert!(workflow.validate().is_err(), "{step}");
        }
    }
    #[test]
    fn injects_a_previous_step_output() {
        let outputs = BTreeMap::from([("analysis".to_owned(), "found one bug".to_owned())]);
        assert_eq!(
            render_input("Fix this: {{ outputs.analysis }}", &outputs).unwrap(),
            "Fix this: found one bug"
        );
    }
}
