use super::Pending;

impl Pending {
    pub fn diff(&self) -> Result<String, String> {
        let before = self.before.as_deref().ok_or("EDIT is not prepared")?;
        let after = self.request.apply_to(before)?;
        // Escape control characters in paths; file contents remain exact in the diff.
        let label = format!("{:?}", self.request.path);
        Ok(similar::TextDiff::from_lines(before, &after)
            .unified_diff()
            .context_radius(3)
            .header(
                if self.was_missing {
                    "/dev/null"
                } else {
                    &label
                },
                &label,
            )
            .to_string())
    }
}
