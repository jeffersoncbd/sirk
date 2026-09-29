use serde::Deserialize;

pub struct Request {
    pub path: String,
    pub offset: usize,
    pub limit: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Arguments {
    #[serde(alias = "filePath")]
    path: String,
    offset: Option<usize>,
    limit: Option<usize>,
}

pub fn request(input: &str) -> Result<Request, String> {
    if !input.trim_start().starts_with('{') {
        return Ok(Request {
            path: input.to_owned(),
            offset: 1,
            limit: usize::MAX,
        });
    }
    let arguments: Arguments =
        serde_json::from_str(input).map_err(|error| format!("invalid READ request: {error}"))?;
    let offset = arguments.offset.unwrap_or(1);
    let limit = arguments.limit.unwrap_or(2_000);
    if offset == 0 || limit == 0 {
        return Err("READ offset and limit must be positive integers".into());
    }
    Ok(Request {
        path: arguments.path,
        offset,
        limit,
    })
}
