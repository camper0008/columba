use crate::error::KeyRingError;
use crate::keyring::KeyRing;
use std::io::Write;
use std::net::TcpStream;

pub enum ConnectionError {
    IoError,
    KeyRingError(KeyRingError),
}

pub struct Connection {
    stream: TcpStream,
}

fn string_payload_to_sized_raw(payload: String) -> Vec<u8> {
    // for simplicity, payloads should be 4096 bytes
    payload
        .chars()
        .chain((payload.len()..4096).map(|_| ' '))
        .enumerate()
        .filter_map(|(i, c)| if i < 4096 { Some(c) } else { None })
        .map(|c| c as u8)
        .collect()
}

impl Connection {
    pub fn connect(ip: &str) -> std::io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(ip)?,
        })
    }

    pub fn create(&mut self, mut name: String, keyring: KeyRing) -> Result<(), ConnectionError> {
        name.shrink_to(128);

        let pub_key = keyring
            .public_key()
            .map_err(|err| ConnectionError::KeyRingError(err))?;

        let string_payload = format!("name\n{}\npublic\n{}\n", name, pub_key);

        let raw_payload = string_payload_to_sized_raw(string_payload);

        self.stream
            .write(&raw_payload)
            .map_err(|_| ConnectionError::IoError)?;

        Ok(())
    }
    pub fn inbox(&mut self, name: String) -> Result<(), ConnectionError> {
        let string_payload = format!("inbox\nname\n{}\n", name);

        let raw_payload = string_payload_to_sized_raw(string_payload);

        self.stream
            .write(&raw_payload)
            .map_err(|_| ConnectionError::IoError)?;

        Ok(())
    }
}
