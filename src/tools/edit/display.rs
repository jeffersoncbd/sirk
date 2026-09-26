use std::io::IsTerminal;

pub fn display(diff: &str) {
    let color = std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    print!("{}", super::render::render(diff, color));
}
