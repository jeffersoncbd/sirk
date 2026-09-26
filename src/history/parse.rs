use super::{Block, History, ParsedHistory, SEPARATOR, Snapshot, TITLE, finish::finish};

impl History {
    pub(super) fn parse(source: &str) -> Result<ParsedHistory, String> {
        let source = source
            .strip_prefix(TITLE)
            .ok_or("unsupported history format; only v2 transcripts can be resumed")?;
        let (metadata, body) = source
            .split_once("\n---\n")
            .ok_or("missing history configuration boundary")?;
        let snapshot: Snapshot = serde_yaml::from_str(metadata)
            .map_err(|e| format!("invalid history configuration: {e}"))?;
        let mut steps: Vec<Vec<Block>> = Vec::new();
        let mut labels = Vec::new();
        let mut active: Option<(&str, Vec<String>)> = None;
        for line in body.split_terminator('\n') {
            if line == SEPARATOR
                || line.starts_with("Step ")
                || matches!(
                    line,
                    "==> ASK"
                        | "==> INPUT"
                        | "<== OUTPUT"
                        | "==> TREE"
                        | "==> READ"
                        | "==> EDIT"
                        | "==> DELETE"
                )
            {
                finish(&mut active, &mut steps)?;
                if line.starts_with("Step ") {
                    labels.push(line.to_owned());
                    steps.push(Vec::new());
                } else if line != SEPARATOR {
                    active = Some((line, Vec::new()));
                }
            } else if let Some((_, lines)) = &mut active {
                lines.push(line.strip_prefix('\\').unwrap_or(line).to_owned());
            } else if !line.is_empty() {
                return Err(format!("unexpected history line: {line}"));
            }
        }
        finish(&mut active, &mut steps)?;
        Ok((snapshot, steps, labels))
    }
}
