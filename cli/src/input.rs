use std::io::stdin;

pub enum Command {
    Help,
    Exit,
    Clear,
    Keyring { location: String },
    Create { name: String },
    Inbox { name: String },
    ReadMessage { id: String },
    SendMessage { reciever: String, msg: String },
    Error { msg: String },
}

pub fn parse_stdin() -> Command {
    let mut cmd = String::new();
    stdin().read_line(&mut cmd).unwrap();
    parse_input(cmd)
}

pub fn parse_input(cmd: String) -> Command {
    let split: Vec<&str> = cmd.split(" ").map(|s| s.trim()).collect();

    match split.get(0).unwrap().to_lowercase().as_str() {
        "help" => Command::Help,
        "exit" => Command::Exit,
        "clear" | "cls" => Command::Clear,
        other_cmd => Command::Error {
            msg: format!("unrecognized command '{other_cmd}', try 'help'"),
        },
    }
}
