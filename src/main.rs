use dbus::arg::messageitem::MessageItem;
use dbus::blocking::Connection;
use dbus::message::MatchRule;
use std::error::Error;
use std::time::Duration;

fn unbox<T>(value: Box<T>) -> T {
    *value
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::new_system()?;

    let rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged");

    conn.add_match(rule, move |_: (), _, msg| {
        let items = msg.get_items();

        if let Some(MessageItem::Str(source)) = items.first() {
            if source.contains("GattCharacteristic") {
                if let Some(MessageItem::Dict(dict_items)) = items.get(1) {
                    let mut bytes = Vec::new();
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
                    match String::from_utf8(bytes) {
                        Ok(payload) => {
                            println!("Deserialized payload: {:?}", payload);
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize binary data: {:?}", e);
                        }
                    }
                }
            }
        }

        true
    })?;

    loop {
        conn.process(Duration::from_millis(1000))?;
    }
}
