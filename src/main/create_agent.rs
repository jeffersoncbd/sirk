use std::env;

pub(super) fn create() -> Result<(), String> {
    let path = sirk::tools::new_agent::create(
        &env::current_dir().map_err(|error| error.to_string())?,
        &mut sirk::input::TerminalInput,
    )?;
    println!("Agent created: {}", path.display());
    Ok(())
}
