use crate::keygen::{generate_key_pair, KeyGenError};
use openssl::error::ErrorStack;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn file_exists(file_location: &str) -> bool {
    Path::new(file_location).exists()
}

fn create_key_file(key_location: &str) -> Result<(), KeyRingError> {
    let pair = generate_key_pair().map_err(|err| KeyRingError::KeyGenError(err))?;

    let private = pair
        .private_key_to_pem()
        .map_err(|err| KeyRingError::OpenSSLError(err))?;
    File::create(key_location)
        .map_err(|_| KeyRingError::IoError)?
        .write_all(&private)
        .map_err(|_| KeyRingError::IoError)?;

    let pub_key_location = key_location.to_owned() + ".pub";
    let public = pair
        .public_key_to_pem()
        .map_err(|err| KeyRingError::OpenSSLError(err))?;
    File::create(pub_key_location)
        .map_err(|_| KeyRingError::IoError)?
        .write_all(&public)
        .map_err(|_| KeyRingError::IoError)?;

    Ok(())
}

fn create_key_file_if_doesnt_exist(key_location: &str) -> Result<(), KeyRingError> {
    let pub_key_location = key_location.to_owned() + ".pub";
    if file_exists(key_location) || file_exists(&pub_key_location) {
        return Err(KeyRingError::FileExists);
    }

    create_key_file(key_location)
}

fn read_key_file(key_location: &str) -> Result<String, KeyRingError> {
    let mut buf = String::new();
    File::open(key_location)
        .map_err(|_| KeyRingError::IoError)?
        .read_to_string(&mut buf)
        .map_err(|_| KeyRingError::IoError)?;

    Ok(buf)
}

pub enum KeyRingError {
    FileExists,
    FileNotFound,
    IoError,
    KeyGenError(KeyGenError),
    OpenSSLError(ErrorStack),
}

pub struct KeyRing {
    key_location: String,
    pub_key_location: String,
}

impl KeyRing {
    pub fn new(key_location: String) -> Result<Self, KeyRingError> {
        create_key_file_if_doesnt_exist(&key_location)?;

        let pub_key_location = key_location.clone() + ".pub";

        Ok(Self {
            key_location,
            pub_key_location,
        })
    }

    pub fn from(key_location: String) -> Result<Self, KeyRingError> {
        let pub_key_location = key_location.to_owned() + ".pub";

        if !file_exists(&key_location) || !file_exists(&pub_key_location) {
            Err(KeyRingError::FileNotFound)
        } else {
            Ok(Self {
                key_location,
                pub_key_location,
            })
        }
    }

    pub fn private_key(&self) -> Result<String, KeyRingError> {
        read_key_file(&self.key_location)
    }
    pub fn public_key(&self) -> Result<String, KeyRingError> {
        read_key_file(&self.pub_key_location)
    }
}
