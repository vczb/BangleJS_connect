use std::{error::Error, time::Duration};

use dbus::{blocking::Connection, message::MatchRule};

use crate::{
    actions::{handle_btn1, handle_drag, handle_touch},
    types::Events,
    utils::{bytes_to_string, dbus_to_bytes},
};

pub fn event_listener() -> Result<(), Box<dyn Error>> {
    let conn = Connection::new_system()?;

    let rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged");

    conn.add_match(rule, move |_: (), _, msg| {
        let items = msg.get_items();

        let bytes = dbus_to_bytes(items);
        let string_data = bytes_to_string(bytes);

        if !string_data.is_empty() {
            match serde_json::from_str::<Events>(&string_data) {
                Ok(event) => match event {
                    Events::Drag(drag) => {
                        handle_drag(drag);
                    }
                    Events::Btn1(_btn1) => {
                        handle_btn1();
                    }
                    Events::Touch(_touch) => {
                        handle_touch();
                    }
                },
                Err(_e) => {
                    // eprintln!("Failed to parse JSON: {:?}", e)
                }
            }
        }

        true
    })?;

    loop {
        conn.process(Duration::from_millis(1000))?;
    }
}
