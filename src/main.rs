use ble_utils::{connect_device, find_light, is_device_connected};
use btleplug::api::Peripheral;
use btleplug::api::{bleuuid::BleUuid, Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use dbus_event_listener::event_listener;
use futures::stream::StreamExt;
use std::error::Error;
use uuid::Uuid;
mod actions;
mod ble_utils;
mod dbus_event_listener;
mod mouse;
mod types;
mod utils;

const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);
// mod bluetooth_pairing;

const DEVICE_NAME: &str = "Bangle";
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize Bluetooth manager
    let manager = Manager::new().await?;

    // Retrieve Bluetooth adapters
    let adapters = manager.adapters().await?;
    let central_wrap = adapters.into_iter().nth(0);
    let central = central_wrap.unwrap();

    // Find and connect to the Bangle device
    let bangle_wrap = find_light(&central, &DEVICE_NAME).await;
    let bangle = bangle_wrap.unwrap();
    // let is_connected = is_device_connected(&bangle).await;
    // dbg!(&bangle.characteristics());

    // if !is_connected {
    connect_device(&bangle).await?;
    // }

    // BUG: We receive some data and the process stop, investigate
    // Iterate through characteristics of the Bangle device
    // for characteristic in bangle.characteristics() {
    //     println!("Checking characteristic {:?}", characteristic);

    //     // Subscribe to notifications from the characteristic with the selected UUID
    //     if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID {
    //         println!("Subscribing to characteristic {:?}", characteristic.uuid);
    //         bangle.subscribe(&characteristic).await?;

    //         // Print the first 4 notifications received
    //         let mut notification_stream = bangle.notifications().await?.take(4);
    //         while let Some(data) = notification_stream.next().await {
    //             println!(
    //                 "Received data from {:?} [{:?}]: {:?}",
    //                 DEVICE_NAME, data.uuid, data.value
    //             );
    //         }
    //     }
    // }

    // TODO: Revove dbus listener when btleplug implementation be done
    let _ = event_listener();

    Ok(())
}
