use crate::connection::Connection;
use crate::error::ConnectionError;
use columba_utils::payload::string_payload_to_raw;
use std::io::{Read, Write};

pub fn inbox(con: &mut Connection, name: String) -> Result<(), ConnectionError> {
    let string_payload = format!(
        "===BEGIN_INBOX_REQ===\ninbox\nname\n{}\n===END_INBOX_REQ===\n",
        name
    );

    let raw_payload = string_payload_to_raw(string_payload);

    con.stream
        .write(&raw_payload)
        .map_err(|_| ConnectionError::IoError)?;

    unimplemented!("buffer parsing not implemented");

    let mut buf = [0; 4096];
    con.stream
        .read(&mut buf)
        .map_err(|_| ConnectionError::IoError)?;

    Ok(())
}
