mod prompt;
mod utils;
use crate::prompt::run::run;
use crate::utils::{display_prompt, parse_stdin};

fn main() -> ! {
    println!(
        "columba-cli {}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("")
    );
    println!("type 'help' for help");
    loop {
        display_prompt();
        run(parse_stdin());
    }
}
