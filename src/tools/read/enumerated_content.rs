/// Recovers exact source content from `enumerate` output saved in a transcript.
pub fn enumerated_content(numbered: &str) -> Result<String, String> {
    let body = numbered
        .strip_prefix("Line | Content\n")
        .ok_or("invalid enumerated READ result")?;
    let mut content = String::new();
    let mut expected = 1usize;
    let mut remaining = body;
    while !remaining.is_empty() {
        let line_end = remaining
            .find('\n')
            .map_or(remaining.len(), |index| index + 1);
        let line = &remaining[..line_end];
        let (number, source) = line
            .split_once(" | ")
            .ok_or("invalid enumerated READ result")?;
        if number.parse::<usize>().ok() != Some(expected) {
            return Err("invalid enumerated READ result".into());
        }
        content.push_str(source);
        remaining = &remaining[line_end..];
        expected += 1;
    }
    Ok(content)
}
