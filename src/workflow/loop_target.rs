use super::output_name::valid;

pub fn loop_target(output: &str) -> Option<&str> {
    output
        .trim()
        .strip_prefix("{{")?
        .strip_suffix("}}")?
        .trim()
        .strip_prefix("loop.")
        .filter(|name| valid(name))
}
