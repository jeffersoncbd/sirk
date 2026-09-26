use super::output_name::valid;
use std::collections::BTreeMap;

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
            .filter(|key| valid(key))
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
