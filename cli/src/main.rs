mod flags;
mod prompt;
mod utils;
use crate::prompt::run::run;
use crate::utils::{display_introduction, display_prompt, parse_stdin};

fn main() -> ! {
    let _given_flags = flags::parse();
    display_introduction();
    loop {
        display_prompt();
        run(parse_stdin());
    }
}
