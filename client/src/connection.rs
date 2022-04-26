use crate::api::handlers;
use crate::error::KeyRingError;
use crate::keyring::KeyRing;
use std::net::TcpStream;

pub enum ConnectionError {
    IoError,
    KeyRingError(KeyRingError),
}

pub struct Connection {
    pub stream: TcpStream,
    pub responses: Vec<u8>,
}

impl Connection {
    pub fn connect(ip: &str) -> std::io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(ip)?,
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
}
