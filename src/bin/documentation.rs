use sirk_sdk::Sirk;

#[path = "../../flows/documentation.rs"]
mod documentation;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sirk = Sirk::connect()?;
    documentation::run(&sirk)
}
