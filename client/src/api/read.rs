use crate::connection::Connection;
use crate::error::ConnectionError;
use columba_utils::payload::string_payload_to_raw;
use std::io::Write;

pub fn read(con: &mut Connection, name: String) -> Result<(), ConnectionError> {
    let string_payload = format!(
        "===BEGIN_READ_REQ===\ninbox\nname\n{}\n===END_READ_REQ===\n",
        name
    );

    let raw_payload = string_payload_to_raw(string_payload);

    con.stream
        .write(&raw_payload)
        .map_err(|_| ConnectionError::IoError)?;

    Ok(())
}
