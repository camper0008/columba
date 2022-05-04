mod cmd;
mod prompt_parse;
mod utils;
use crate::utils::{display_prompt, parse_stdin};

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
