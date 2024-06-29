pub fn bytes_to_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(payload) => return payload,
        Err(e) => {
            eprintln!("Failed to deserialize binary data: {:?}", e);
            "".to_string()
        }
    }
}
