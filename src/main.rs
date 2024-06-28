use actions::{handle_btn1, handle_drag, handle_touch};
use bluetooth_pairing::pair_device;
use dbus::blocking::Connection;
use dbus::message::MatchRule;
use std::error::Error;
use std::time::Duration;
use utils::{bytes_to_string, dbus_to_bytes};

mod actions;
mod bluetooth_pairing;
mod mouse;
mod types;
mod utils;
use types::Events;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = pair_device();

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
