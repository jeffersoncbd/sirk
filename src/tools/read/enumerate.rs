/// Presents exact file content with stable, one-based line coordinates.
///
/// The header makes the prefix self-describing while each source line, including
/// its original line ending, remains after its `N | ` prefix.
pub fn enumerate(content: &str) -> String {
    let mut numbered = String::from("Line | Content\n");
    for (index, line) in content.split_inclusive('\n').enumerate() {
        numbered.push_str(&(index + 1).to_string());
        numbered.push_str(" | ");
        numbered.push_str(line);
    }
    numbered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::read::enumerated_content;

    #[test]
    fn enumerates_lines_without_losing_exact_content() {
        for content in ["", "one", "one\n", "one\r\ntwo\n\n"] {
            let numbered = enumerate(content);
            assert!(numbered.starts_with("Line | Content\n"));
            assert_eq!(enumerated_content(&numbered).unwrap(), content);
        }
        assert_eq!(enumerate("one\ntwo"), "Line | Content\n1 | one\n2 | two");
        assert!(enumerated_content("1 | one\n").is_err());
        assert!(enumerated_content("Line | Content\n2 | one\n").is_err());
    }
}
