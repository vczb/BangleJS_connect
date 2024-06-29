use std::error::Error;

use bluetooth_pairing::pair_device;
//use dbus_event_listener::event_listener;

mod actions;
mod bluetooth_pairing;
mod dbus_event_listener;
mod mouse;
mod types;
mod utils;

use btleplug::api::Peripheral;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = pair_device().await {
        println!("Error pairing device: {}", e);
    }
    //event_listener()

    Ok(())
}
