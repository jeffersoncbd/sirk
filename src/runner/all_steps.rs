use crate::workflow::Step;

pub(super) fn all_steps(steps: &[Step]) -> Vec<&Step> {
    let mut result = Vec::new();
    for step in steps {
        result.push(step);
        result.extend(all_steps(&step.iter));
        result.extend(all_steps(&step.is_true));
        result.extend(all_steps(&step.is_false));
    }
    result
}
