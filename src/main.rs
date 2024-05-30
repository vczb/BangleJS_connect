use actions::{handle_btn1, handle_drag};
use dbus::blocking::Connection;
use dbus::message::MatchRule;
use std::error::Error;
use std::time::Duration;
use utils::{bytes_to_string, dbus_to_bytes};
mod types;

mod actions;
mod mouse;
mod utils;
use types::Events;

fn main() -> Result<(), Box<dyn Error>> {
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
                        println!("Drag: {:?}", drag);
                        handle_drag(drag);
                    }
                    Events::Btn1(btn1) => {
                        println!("Btn1: {:?}", btn1);
                        handle_btn1();
                    }
                },
                Err(e) => eprintln!("Failed to parse JSON: {:?}", e),
            }
        }

        true
    })?;

    loop {
        conn.process(Duration::from_millis(1000))?;
    }
}
