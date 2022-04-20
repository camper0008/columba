mod handlers;
mod utils;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::handlers::create;
use crate::utils::logger::possible_write_error;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    stream.read(&mut buf).expect("error reading input buffer");
    let mut iter = buf.split(|x| *x == '\n' as u8);
    match iter.nth(0) {
        Some(b"send") => {}
        Some(b"read") => {}
        Some(b"inbox") => {}
        Some(c) => possible_write_error(
            stream.write(
                &format!("unrecognized input '{}'", String::from_utf8_lossy(c))
                    .bytes()
                    .collect::<Vec<u8>>(),
            ),
        ),
        _ => possible_write_error(stream.write(b"no input given")),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
