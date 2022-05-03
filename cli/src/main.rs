mod cmd;
mod input;
mod utils;
use crate::input::parse_stdin;
use crate::utils::display_prompt;

fn main() -> ! {
    println!(
        "columba-cli {}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("")
    );
    println!("type 'help' for help");
    loop {
        display_prompt();
        cmd::run(parse_stdin());
    }
}
