use std::{fs, path::Path};

fn main() -> Result<(), String> {
    let document = sirk::http::openapi_document();
    let yaml = document.to_yaml().map_err(|error| error.to_string())?;
    fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("openapi.yaml"),
        yaml,
    )
    .map_err(|error| error.to_string())
}
