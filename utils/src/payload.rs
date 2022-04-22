pub fn string_payload_to_sized_raw(payload: String) -> Vec<u8> {
    // for simplicity, payloads should be 4096 bytes
    payload
        .chars()
        .chain((payload.len()..4096).map(|_| ' '))
        .enumerate()
        .filter_map(|(i, c)| if i < 4096 { Some(c) } else { None })
        .map(|c| c as u8)
        .collect()
}
