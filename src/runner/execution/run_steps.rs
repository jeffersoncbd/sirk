use super::Engine;
use crate::{
    history::Block,
    input::UserInput,
    services::Invocation,
    workflow::{Step, condition, loop_items, loop_target},
};
use std::collections::BTreeMap;

impl<E, I> Engine<'_, E, I>
where
    E: FnMut(&Invocation) -> Result<String, String>,
    I: UserInput,
{
    pub(super) fn run_steps(
        &mut self,
        steps: &[Step],
        prefix: &str,
        outputs: &mut BTreeMap<String, String>,
        locals: &mut Option<BTreeMap<String, String>>,
    ) -> Result<(), String> {
        for (position, step) in steps.iter().enumerate() {
            let id = format!("{prefix}{}", position + 1);
            let index = self.cursor;
            self.cursor += 1;
            if index == self.history.steps.len() {
                self.history.steps.push(Vec::new());
                self.history
                    .labels
                    .push(format!("Step {id} — {}", step.name()));
            }
            if step.tool.as_deref() == Some("LOOP") {
                if self.history.steps[index].is_empty() {
                    let argument = step.render_input(outputs, locals.as_ref())?;
                    loop_items(&argument)?;
                    self.history.steps[index].push(Block::Input(argument));
                    self.history.save()?;
                }
                let items = loop_items(self.history.steps[index][0].text())?;
                for (iteration, item) in items.into_iter().enumerate() {
                    let mut child_outputs = outputs.clone();
                    let mut child_locals = Some(BTreeMap::from([("item".into(), item)]));
                    self.run_steps(
                        &step.iter,
                        &format!("{id}.{}.", iteration + 1),
                        &mut child_outputs,
                        &mut child_locals,
                    )?;
                }
                continue;
            }
            if step.tool.as_deref() == Some("IF") {
                if self.history.steps[index].is_empty() {
                    let argument = step.render_input(outputs, locals.as_ref())?;
                    condition(&argument)?;
                    self.history.steps[index].push(Block::Input(argument));
                    self.history.save()?;
                }
                let selected = condition(self.history.steps[index][0].text())?;
                let (branch, body) = step.branch(selected);
                self.run_steps(body, &format!("{id}.{branch}."), outputs, locals)?;
                continue;
            }
            let result = self.run_step(step, index, outputs, locals.as_ref())?;
            let version_content = if step.tool.as_deref() == Some("READ") && step.enumerate() {
                crate::tools::read::enumerated_content(&result)?
            } else {
                result.clone()
            };
            let version = crate::tools::edit::version(&version_content);
            for (name, value) in step
                .output
                .iter()
                .map(|name| (name, &result))
                .chain(step.version_output.iter().map(|name| (name, &version)))
            {
                if let Some(key) = loop_target(name) {
                    locals
                        .as_mut()
                        .ok_or("missing loop scope")?
                        .insert(key.into(), value.clone());
                } else if let Some(values) = locals.as_mut() {
                    values.insert(name.clone(), value.clone());
                } else {
                    outputs.insert(name.clone(), value.clone());
                }
            }
        }
        Ok(())
    }
}
