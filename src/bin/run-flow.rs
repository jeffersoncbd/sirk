use sirk_sdk::Sirk;

#[path = "../../flows/documentation.rs"]
mod documentation;
#[path = "../../flows/profile_interviewer.rs"]
mod profile_interviewer;

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
        "profile_interviewer" => profile_interviewer::run(&Sirk::connect()?),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("flow `{flow}` was not found in flows/"),
        )
        .into()),
    }
}
