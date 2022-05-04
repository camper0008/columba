use crate::prompt_parse::{parse_input, Command};
use std::io::{stdin, stdout, Write};

pub fn parse_stdin() -> Command {
    let mut cmd = String::new();
    stdin().read_line(&mut cmd).unwrap();
    parse_input(cmd)
}

fn flush_stdout() {
    stdout().flush().expect("unable to flush stdout");
}

pub fn display_prompt() {
    print!("-> ");
    flush_stdout();
}
