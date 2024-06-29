use std::error::Error;
mod bluetooth_pairing;
//use bluetooth_pairing::pair_device;
//use subscribe_and_notify::event_listener;
use dbus_event_listener::event_listener;
mod dbus_event_listener;
mod subscribe_and_notify;

mod actions;
mod mouse;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Pair with the device
    //let bangle = pair_device().await?;

    // BUG: characteristics returns empty

    // Subscribe to notifications for all characteristics
    //for characteristic in bangle.characteristics().iter() {
    //    println!("OK");
    //    if let Err(e) = bangle.subscribe(characteristic).await {
    //        println!("Failed to subscribe to characteristic: {}", e);
    //    } else {
    //        println!("Subscribed to characteristic {:?}", characteristic.uuid);
    //    }
    //}
    //

    println!("Listening for events...");
    let _ = event_listener();

    // Event listener loop
    //let _ = event_listener(bangle).await?;

    Ok(())
}
