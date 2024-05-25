use dbus::blocking::Connection;
use dbus::message::MatchRule;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::new_system()?;

    let rule = MatchRule::new_signal("org.freedesktop.DBus.Properties", "PropertiesChanged");

    conn.add_match(rule, move |_: (), _, msg| {
        println!("Received signal: {:?}", msg);
        let test = msg.get_items();
        println!("test: {:?}", test);
        true
    })?;

    loop {
        conn.process(Duration::from_millis(1000))?;
    }
}
