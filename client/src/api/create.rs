use crate::connection::Connection;
use crate::error::ConnectionError;
use columba_crypto::keyring::KeyRing;
use columba_utils::payload::string_payload_to_raw;
use std::io::Write;

pub fn create(con: &mut Connection, name: String, keyring: KeyRing) -> Result<(), ConnectionError> {
    let pub_key = keyring
        .public_key()
        .map_err(|err| ConnectionError::KeyRingError(err))?;

    let string_payload = format!(
        "===BEGIN_CREATE_REQ===\nname\n{}\npublic\n{}\n===END_CREATE_REQ===\n",
        name, pub_key
    );

    let raw_payload = string_payload_to_raw(string_payload);

    con.stream
        .write(&raw_payload)
        .map_err(|_| ConnectionError::IoError)?;

    Ok(())
}
