use crate::connection::Connection;
use crate::error::ConnectionError;
use crate::keyring::KeyRing;
use columba_utils::payload::string_payload_to_raw;
use std::io::Write;

pub fn send(
    con: &mut Connection,
    name: String,
    msg: String,
    reciever_keyring: KeyRing,
) -> Result<(), ConnectionError> {
    let encrypted = reciever_keyring
        .public_encrypt(msg)
        .map_err(|err| ConnectionError::KeyRingError(err))?;

    let string_payload = format!(
        "===BEGIN_SEND_REQ===\nname\n{}\nmessage\n{}\n===END_SEND_REQ===\n",
        name, encrypted
    );

    let raw_payload = string_payload_to_raw(string_payload);

    con.stream
        .write(&raw_payload)
        .map_err(|_| ConnectionError::IoError)?;

    Ok(())
}
