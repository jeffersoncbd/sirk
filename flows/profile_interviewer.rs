use sirk_sdk::Sirk;

pub fn run(sirk: &Sirk) -> Result<(), Box<dyn std::error::Error>> {
    let profile = sirk.agent("profile-interviewer", "Start the profile interview.")?;
    println!("{profile}");
    Ok(())
}
