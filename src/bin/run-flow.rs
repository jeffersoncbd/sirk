use sirk_sdk::Sirk;

#[path = "../../flows/documentation.rs"]
mod documentation;
#[path = "../../flows/development.rs"]
mod development;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let flow = arguments.next().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "usage: cargo run --bin run-flow -- <flow>",
        )
    })?;
    if arguments.next().is_some() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "run-flow accepts exactly one flow name",
        )
        .into());
    }
    match flow.as_str() {
        "documentation" => documentation::run(&Sirk::connect()?),
        "development"   => development::run(&Sirk::connect()?),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("flow `{flow}` was not found in flows/"),
        )
        .into()),
    }
}
