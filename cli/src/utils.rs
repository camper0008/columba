use crate::prompt::parse::{parse, Command};
use std::io::{stdin, stdout, Write};

pub fn parse_stdin() -> Command {
    let mut prompt = String::new();
    stdin().read_line(&mut prompt).unwrap();
    parse(prompt)
}

fn flush_stdout() {
    stdout().flush().expect("unable to flush stdout");
}

pub fn display_prompt() {
    print!("-> ");
    flush_stdout();
}

pub fn display_introduction() {
    println!(
        "columba-cli {}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("")
    );
    println!("type 'help' for help");
}
