use std::path::PathBuf;

pub enum Command {
    Help,
    Exit,
    Clear,
    Keyring { location: PathBuf },
    Create { name: String },
    Inbox { name: String },
    Read { id: String },
    Send { name: String, msg: String },
    Error { msg: String },
}

fn parse_keyring(args: Vec<&str>) -> Command {
    let location_opt = args.get(1);
    if location_opt.is_none() {
        return Command::Error {
            msg: String::from("missing argument 1"),
        };
    }
    let location_str = location_opt.unwrap();
    let location = PathBuf::from(location_str);

    Command::Keyring { location }
}

fn parse_create(args: Vec<&str>) -> Command {
    let name_opt = args.get(1);
    if name_opt.is_none() {
        return Command::Error {
            msg: String::from("missing argument 1"),
        };
    }
    let name = name_opt.unwrap().to_string();

    Command::Create { name }
}

fn parse_inbox(args: Vec<&str>) -> Command {
    let name_opt = args.get(1);
    if name_opt.is_none() {
        return Command::Error {
            msg: String::from("missing argument 1"),
        };
    }
    let name = name_opt.unwrap().to_string();

    Command::Inbox { name }
}

fn parse_read(args: Vec<&str>) -> Command {
    let id_opt = args.get(1);
    if id_opt.is_none() {
        return Command::Error {
            msg: String::from("missing argument 1"),
        };
    }
    let id = id_opt.unwrap().to_string();

    Command::Read { id }
}

fn parse_send(args: Vec<&str>) -> Command {
    let name_opt = args.get(1);
    if name_opt.is_none() {
        return Command::Error {
            msg: String::from("missing argument 1"),
        };
    }
    let name = name_opt.unwrap().to_string();

    let msg_opt = args
        .into_iter()
        .skip(2)
        .map(|s| s.to_string())
        .reduce(|acc, x| acc + &x);

    if msg_opt.is_none() {
        return Command::Error {
            msg: String::from("missing argument 2"),
        };
    }

    let msg = msg_opt.unwrap();

    Command::Send { name, msg }
}

pub fn parse_input(cmd: String) -> Command {
    let split: Vec<&str> = cmd.split(" ").map(|s| s.trim()).collect();

    match split.get(0).unwrap().to_lowercase().as_str() {
        "help" => Command::Help,
        "exit" => Command::Exit,
        "clear" | "cls" => Command::Clear,
        "keyring" => parse_keyring(split),
        "create" => parse_create(split),
        "inbox" => parse_inbox(split),
        "read" => parse_read(split),
        "send" => parse_send(split),
        other_cmd => Command::Error {
            msg: format!("unrecognized command '{other_cmd}', try 'help'"),
        },
    }
}
