use crate::error::KeyRingError;
use crate::keyring::KeyRing;
use columba_utils::payload::string_payload_to_raw;
use std::io::{Read, Write};
use std::net::TcpStream;

pub enum ConnectionError {
    IoError,
    KeyRingError(KeyRingError),
}

pub struct Connection {
    stream: TcpStream,
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

        let string_payload = format!(
            "===BEGIN_CREATE_REQ===\nname\n{}\npublic\n{}\n===END_CREATE_REQ===\n",
            name, pub_key
        );

        let raw_payload = string_payload_to_raw(string_payload);

        self.stream
            .write(&raw_payload)
            .map_err(|_| ConnectionError::IoError)?;

        Ok(())
    }
    pub fn inbox(&mut self, name: String) -> Result<(), ConnectionError> {
        let string_payload = format!(
            "===BEGIN_INBOX_REQ===\ninbox\nname\n{}\n===END_INBOX_REQ===\n",
            name
        );

        let raw_payload = string_payload_to_raw(string_payload);

        self.stream
            .write(&raw_payload)
            .map_err(|_| ConnectionError::IoError)?;

        unimplemented!("buffer parsing not implemented");

        let mut buf = [0; 4096];
        self.stream
            .read(&mut buf)
            .map_err(|_| ConnectionError::IoError)?;

        Ok(())
    }

    pub fn read(&mut self, name: String) -> Result<(), ConnectionError> {
        let string_payload = format!(
            "===BEGIN_READ_REQ===\ninbox\nname\n{}\n===END_READ_REQ===\n",
            name
        );

        let raw_payload = string_payload_to_raw(string_payload);

        self.stream
            .write(&raw_payload)
            .map_err(|_| ConnectionError::IoError)?;

        Ok(())
    }
}
