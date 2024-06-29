use actions::{handle_btn1, handle_drag, handle_touch};
use ble_utils::{connect_device, find_light, is_device_connected};
use btleplug::api::Manager as _;
use btleplug::api::Peripheral;
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use types::Events;
use utils::bytes_to_string;
use uuid::Uuid;
mod actions;
mod ble_utils;
mod mouse;
mod types;
mod utils;

const NOTIFY_CHARACTERISTIC_UUID: Uuid = Uuid::from_u128(0x6e400003_b5a3_f393_e0a9_e50e24dcca9e);

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
    let is_connected = is_device_connected(&bangle).await;
    // dbg!(&bangle.characteristics());

    if !is_connected {
        connect_device(&bangle).await?;
    } else {
        // refresh the connection
        bangle.discover_services().await?;
    }

    // Iterate through characteristics of the Bangle device
    for characteristic in bangle.characteristics() {
        println!("Checking characteristic {:?}", characteristic);

        // Subscribe to notifications from the characteristic with the selected UUID

        if characteristic.uuid == NOTIFY_CHARACTERISTIC_UUID {
            println!("Subscribing to characteristic {:?}", characteristic.uuid);
            bangle.subscribe(&characteristic).await?;

            let mut notification_stream = bangle.notifications().await?;

            // listen by notifications
            while let Some(data) = notification_stream.next().await {
                let value = data.value;
                let string_data = bytes_to_string(value);

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
            }
        }
    }

    Ok(())
}
