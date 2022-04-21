use std::io::{Read, Write};

extern crate client;

use client::connection::connect;

fn main() -> std::io::Result<()> {
    let mut stream = connect("127.0.0.1:8080");
    stream.write(b"create\n")?;

    let mut buf = [0; 4096];
    stream.read(&mut buf).expect("error reading stream");
    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}
