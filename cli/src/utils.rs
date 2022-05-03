use std::io::{stdout, Write};

fn flush_stdout() {
    stdout().flush().expect("unable to flush stdout");
}

pub fn display_prompt() {
    print!("-> ");
    flush_stdout();
}
