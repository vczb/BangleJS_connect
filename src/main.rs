use ble_utils::find_bangle_characteristic;
use ble_utils::{connect_device, find_light, is_device_connected};
use btleplug::api::Manager as _;
use btleplug::api::Peripheral;
use btleplug::platform::Manager;
use notifications::subscribe_and_notify;
use std::error::Error;

mod actions;
mod ble_utils;
mod mouse;
mod notifications;
mod types;
mod utils;

const DEVICE_NAME: &str = "Bangle";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;

    let adapters = manager.adapters().await?;
    let central_option = adapters.into_iter().nth(0);
    let central = central_option.unwrap();

    let bangle_option = find_light(&central, &DEVICE_NAME).await;
    let bangle = bangle_option.unwrap();
    let is_connected = is_device_connected(&bangle).await;

    if !is_connected {
        connect_device(&bangle).await?;
    } else {
        bangle.discover_services().await?;
    }

    let bangle_characteristics = bangle.characteristics();

    let characteristic_option = find_bangle_characteristic(bangle_characteristics).await;

    if let Some(characteristic) = characteristic_option {
        let _ = subscribe_and_notify(&bangle, characteristic).await;
    } else {
        println!("Characteristic not found");
    }

    Ok(())
}
