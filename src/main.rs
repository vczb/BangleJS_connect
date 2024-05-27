use dbus::blocking::Connection;
use dbus::message::MatchRule;
use std::error::Error;
use std::time::Duration;
use utils::{bytes_to_string, dbus_to_bytes};

mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::new_system()?;

    let rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged");

    conn.add_match(rule, move |_: (), _, msg| {
        let items = msg.get_items();

        let bytes = dbus_to_bytes(items);
        let string_data = bytes_to_string(bytes);
        println!("{:?}", string_data);

        true
    })?;

    loop {
        conn.process(Duration::from_millis(1000))?;
    }
}
