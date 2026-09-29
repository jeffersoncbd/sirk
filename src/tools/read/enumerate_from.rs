pub fn enumerate_from(content: &str, offset: usize) -> String {
    let mut numbered = String::from("Line | Content\n");
    for (index, line) in content.split_inclusive('\n').enumerate() {
        numbered.push_str(&(offset + index).to_string());
        numbered.push_str(" | ");
        numbered.push_str(line);
    }
    numbered
}
