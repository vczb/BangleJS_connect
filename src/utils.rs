use dbus::arg::messageitem::MessageItem;

pub fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn dbus_to_bytes(items: Vec<MessageItem>) -> Vec<u8> {
    let mut bytes = Vec::new();
    if let Some(MessageItem::Str(source)) = items.first() {
        if source.contains("GattCharacteristic") {
            if let Some(MessageItem::Dict(dict_items)) = items.get(1) {
                for (_key, value) in dict_items.iter() {
                    if let MessageItem::Variant(variant) = value {
                        if let MessageItem::Array(array) = unbox(variant.clone()) {
                            for byte in array.iter() {
                                if let MessageItem::Byte(b) = byte {
                                    bytes.push(*b);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return bytes;
}

pub fn bytes_to_string(bytes: Vec<u8>) -> String {
    match String::from_utf8(bytes) {
        Ok(payload) => return payload,
        Err(e) => {
            eprintln!("Failed to deserialize binary data: {:?}", e);
            "".to_string()
        }
    }
}
