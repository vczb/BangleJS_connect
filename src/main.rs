use std::error::Error;

use dbus_event_listener::event_listener;

mod actions;
mod bluetooth_pairing;
mod dbus_event_listener;
mod mouse;
mod types;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    //let _ = pair_device();

    event_listener()
}
