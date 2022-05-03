use crate::connection::Connection;
use crate::error::ConnectionError;
use columba_utils::payload::string_payload_to_raw;
use std::io::Write;

pub fn send(con: &mut Connection, name: String, msg: String) -> Result<(), ConnectionError> {
    let string_payload = format!(
        "===BEGIN_SEND_REQ===\nname\n{}\nmessage\n{}\n===END_SEND_REQ===\n",
        name, msg
    );

    let raw_payload = string_payload_to_raw(string_payload);

    con.stream
        .write(&raw_payload)
        .map_err(|_| ConnectionError::IoError)?;

    Ok(())
}
