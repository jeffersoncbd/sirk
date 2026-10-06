use sirk_sdk::Sirk;

pub fn run(sirk: &Sirk) -> Result<(), Box<dyn std::error::Error>> {
    let _ = sirk.agent("requirements-analyst", "")?;

    Ok(())
}
