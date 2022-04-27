use crate::api::handlers;
use crate::error::KeyRingError;
use crate::keyring::KeyRing;
use crate::parse::{parse_response, Response};
use std::io::Read;
use std::net::TcpStream;

pub enum ConnectionError {
    IoError,
    KeyRingError(KeyRingError),
}

pub struct Connection {
    pub stream: TcpStream,
    pub responses: Vec<Response>,
}

impl Connection {
    pub fn connect(ip: &str) -> std::io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(ip)?,
            responses: Vec::new(),
        })
    }

    pub fn create(&mut self, name: String, keyring: KeyRing) -> Result<(), ConnectionError> {
        handlers::create(self, name, keyring)
    }
    pub fn inbox(&mut self, name: String) -> Result<(), ConnectionError> {
        handlers::inbox(self, name)
    }

    pub fn read(&mut self, name: String) -> Result<(), ConnectionError> {
        handlers::read(self, name)
    }
    pub fn parse(&mut self) -> Result<(), ConnectionError> {
        let mut buf = Vec::new();
        self.stream
            .read_to_end(&mut buf)
            .map_err(|_| ConnectionError::IoError)?;

        let split = (&buf)
            .split(|b| *b == '\n' as u8)
            .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
            .collect();
        self.responses.extend(parse_response(split));
        Ok(())
    }
}
