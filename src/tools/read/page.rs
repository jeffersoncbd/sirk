use super::request::request;
use std::path::Path;

pub struct Page {
    pub content: String,
    pub offset: usize,
}

pub fn page(directory: &Path, input: &str) -> Result<Page, String> {
    let request = request(input)?;
    let content = super::read(directory, &request.path)?;
    let lines: Vec<_> = content.split_inclusive('\n').collect();
    let start = request.offset - 1;
    if start > lines.len() || (start == lines.len() && !content.is_empty()) {
        return Err("READ offset is beyond the end of the file".into());
    }
    Ok(Page {
        content: lines.into_iter().skip(start).take(request.limit).collect(),
        offset: request.offset,
    })
}
