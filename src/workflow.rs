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
    pub line: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(
        default,
        rename = "version-output",
        skip_serializing_if = "Option::is_none"
    )]
    pub version_output: Option<String>,
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub iter: Vec<Step>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StepInput {
    Text(String),
    Array(Vec<String>),
}

impl Default for StepInput {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl StepInput {
    fn render(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        match self {
            Self::Text(text) => render_scoped(text, outputs, locals),
            Self::Array(items) => items
                .iter()
                .map(|item| render_scoped(item, outputs, locals))
                .collect::<Result<Vec<_>, _>>()
                .and_then(|items| serde_json::to_string(&items).map_err(|error| error.to_string())),
        }
    }

    fn text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            Self::Array(_) => None,
        }
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
            .or(self.custom_tool.as_deref())
            .unwrap_or("invalid")
    }

    pub fn is_tool_step(&self) -> bool {
        self.tool.is_some() || self.custom_tool.is_some()
    }

    pub fn render_input(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        self.input.render(outputs, locals)
    }

    pub fn render_path(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<String, String> {
        let path = self.path.as_deref().ok_or("tool requires a path")?;
        render_scoped(path, outputs, locals)
    }

    pub fn force(&self) -> bool {
        self.force.unwrap_or(false)
    }

    pub fn edit_request(
        &self,
        outputs: &BTreeMap<String, String>,
        locals: Option<&BTreeMap<String, String>>,
    ) -> Result<crate::tools::edit::Request, String> {
        Ok(crate::tools::edit::Request {
            path: self.render_path(outputs, locals)?,
            operation: self.operation.ok_or("EDIT requires operation")?,
            line: self.line,
            start: self.start,
            end: self.end,
            version: self
                .version
                .as_deref()
                .map(|v| render_scoped(v, outputs, locals))
                .transpose()?,
            input: self.render_input(outputs, locals)?,
        })
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
        let is_write = step.tool.as_deref() == Some("WRITE");
        let is_edit = step.tool.as_deref() == Some("EDIT");
        if !is_edit
            && (step.operation.is_some()
                || step.line.is_some()
                || step.start.is_some()
                || step.end.is_some()
                || step.version.is_some())
        {
            return Err("operation, line, start, end and version are only accepted on EDIT".into());
        }
        if step.version_output.is_some() && step.tool.as_deref() != Some("READ") {
            return Err("version-output is only accepted on READ".into());
        }
        if step.version_output.is_some() && step.version_output == step.output {
            return Err("READ output and version-output must have different names".into());
        }
        if !is_loop && !step.iter.is_empty() {
            return Err("iter is only accepted on LOOP steps".into());
        }
        if !is_write && !is_edit && step.path.is_some() {
            return Err(format!(
                "step {}: path is only accepted on WRITE or EDIT",
                index + 1
            ));
        }
        if !is_write && step.force.is_some() {
            return Err(format!(
                "step {}: force is only accepted on WRITE",
                index + 1
            ));
        }
        if !is_write && step.skip.is_some() {
            return Err("skip is only accepted on WRITE".into());
        }
        if step.force() && step.skip.unwrap_or(false) {
            return Err("WRITE cannot combine force: true with skip: true".into());
        }
        match (&step.agent, &step.tool, &step.custom_tool) {
            (Some(agent), None, None) if crate::agents::valid_id(agent) => (),
            (None, Some(tool), None) if tool == "LOOP" => {
                if step.iter.is_empty() {
                    return Err("LOOP requires a nonempty iter list".into());
                }
                if step.output.is_some() {
                    return Err("LOOP has no aggregate output; store results inside iter".into());
                }
                if let Some(input) = step.input.text()
                    && !input.contains("{{")
                {
                    loop_items(input)?;
                }
            }
            (None, Some(tool), None) if crate::tools::supports(tool) => {
                if tool == "TREE" && step.input.text() != Some("") {
                    return Err(format!("step {}: TREE does not accept input", index + 1));
                }
                if tool == "READ"
                    && step
                        .input
                        .text()
                        .is_none_or(|input| input.trim().is_empty())
                {
                    return Err(format!(
                        "step {}: READ requires a file path in input",
                        index + 1
                    ));
                }
                if tool == "WRITE" || tool == "EDIT" {
                    if !matches!(step.input, StepInput::Text(_)) {
                        return Err(format!(
                            "step {}: {tool} input must be text content",
                            index + 1
                        ));
                    }
                    if step.path.as_ref().is_none_or(|path| path.trim().is_empty()) {
                        return Err(format!("step {}: {tool} requires a path", index + 1));
                    }
                }
                if is_edit {
                    let mut request = step.edit_request(outputs, locals.as_deref())?;
                    // References resolve to placeholder values during static validation.
                    request.path = step.path.clone().unwrap();
                    if step.version.as_ref().is_some_and(|v| v.contains("{{")) {
                        request.version = Some("0".repeat(64));
                    }
                    request.validate()?;
                    if request.operation == crate::tools::edit::Operation::Delete
                        && step.input.text() != Some("")
                    {
                        return Err("EDIT delete does not accept input".into());
                    }
                }
            }
            (None, None, Some(tool)) if crate::tools::custom::valid_name(tool) => {
                if !matches!(step.input, StepInput::Array(_)) {
                    return Err(format!(
                        "step {}: CUSTOM-TOOL input must be a list of strings",
                        index + 1
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "step {} must specify exactly one valid agent, tool (TREE, READ, WRITE, EDIT, LOOP), or custom-tool",
                    index + 1
                ));
            }
        }
        step.input
            .render(outputs, locals.as_deref())
            .map_err(|error| format!("step {}: {error}", index + 1))?;
        if let Some(path) = &step.path {
            render_scoped(path, outputs, locals.as_deref())
                .map_err(|error| format!("step {}: {error}", index + 1))?;
        }
        if is_loop {
            let mut child_outputs = outputs.clone();
            let mut child_locals = BTreeMap::from([("item".into(), String::new())]);
            validate_steps(&step.iter, &mut child_outputs, Some(&mut child_locals))?;
        }
        for name in step.output.iter().chain(step.version_output.iter()) {
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
    fn validates_edit_operations_and_version_outputs() {
        let valid = "version: 1\nsteps:\n- tool: READ\n  input: file\n  output: source\n  version-output: revision\n- tool: EDIT\n  path: file\n  operation: replace\n  start: 1\n  end: 2\n  version: '{{ outputs.revision }}'\n  input: new\n";
        serde_yaml::from_str::<Workflow>(valid)
            .unwrap()
            .validate()
            .unwrap();
        for invalid in [
            valid.replace("start: 1", "start: 0"),
            valid.replace("end: 2", "end: 0"),
            valid.replace("  version: '{{ outputs.revision }}'\n", ""),
            valid.replace("operation: replace", "operation: append"),
            valid.replace("operation: replace", "operation: insert"),
            valid.replace("operation: replace", "operation: delete"),
            valid.replace("version-output: revision", "version-output: source"),
            valid.replace("outputs.revision", "outputs.missing"),
            valid.replace("tool: EDIT", "tool: WRITE"),
            valid.replace("input: new", "input: [new]"),
        ] {
            assert!(
                serde_yaml::from_str::<Workflow>(&invalid)
                    .unwrap()
                    .validate()
                    .is_err(),
                "{invalid}"
            );
        }
        for operation in ["append", "prepend"] {
            let workflow: Workflow = serde_yaml::from_str(&format!("version: 1\nsteps:\n- tool: EDIT\n  path: log\n  operation: {operation}\n  input: entry\n")).unwrap();
            workflow.validate().unwrap();
        }
    }

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
            "tool: WRITE\n  input: content",
            "tool: WRITE\n  path: output.txt\n  input: [content]",
            "tool: READ\n  path: output.txt\n  input: source.txt",
            "agent: planner\n  force: false",
            "agent: planner\n  skip: false",
            "tool: WRITE\n  path: file\n  force: true\n  skip: true",
        ] {
            let workflow: Workflow =
                serde_yaml::from_str(&format!("version: 1\nsteps:\n- {step}\n")).unwrap();
            assert!(workflow.validate().is_err(), "{step}");
        }
    }
    #[test]
    fn validates_write_steps_and_renders_their_paths() {
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- agent: planner\n  output: filename\n- tool: WRITE\n  path: 'docs/{{ outputs.filename }}.md'\n  input: content\n  force: true\n  output: written\n",
        )
        .unwrap();
        workflow.validate().unwrap();
        assert_eq!(
            workflow.steps[1]
                .render_path(&BTreeMap::from([("filename".into(), "guide".into())]), None)
                .unwrap(),
            "docs/guide.md"
        );
        assert!(workflow.steps[1].force());
    }
    #[test]
    fn validates_custom_tools_and_renders_each_argument_separately() {
        let workflow: Workflow = serde_yaml::from_str(
            "version: 1\nsteps:\n- agent: planner\n  output: value\n- custom-tool: format-name\n  input:\n  - '{{ outputs.value }}'\n  - literal argument\n  output: formatted\n",
        )
        .unwrap();
        workflow.validate().unwrap();
        let rendered = workflow.steps[1]
            .render_input(
                &BTreeMap::from([("value".into(), "quotes \" and {{ untouched }}".into())]),
                None,
            )
            .unwrap();
        assert_eq!(
            crate::tools::custom::arguments(&rendered).unwrap(),
            ["quotes \" and {{ untouched }}", "literal argument"]
        );
        for step in [
            "custom-tool: invalid/name\n  input: []",
            "custom-tool: valid\n  input: text",
            "agent: planner\n  custom-tool: valid\n  input: []",
            "tool: READ\n  custom-tool: valid\n  input: []",
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
