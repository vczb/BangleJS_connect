use btleplug::api::Characteristic;
use btleplug::api::Peripheral;
use futures::stream::StreamExt;
use std::error::Error;

use crate::actions::handle_click;
use crate::actions::handle_keypress;
use crate::actions::handle_move;
use crate::types::Events;
use crate::utils::bytes_to_string;

pub async fn subscribe_and_notify(
    peripheral: &impl Peripheral,
    characteristic: Characteristic,
) -> Result<(), Box<dyn Error>> {
    peripheral.subscribe(&characteristic).await?;

    let mut notification_stream = peripheral.notifications().await?;

    while let Some(data) = notification_stream.next().await {
        let value = data.value;
        let string_data = bytes_to_string(value);

        if !string_data.is_empty() {
            match serde_json::from_str::<Events>(&string_data) {
                Ok(event) => match event {
                    Events::MouseMove(cursor) => {
                        handle_move(cursor);
                    }
                    Events::Click(_click) => {
                        handle_click();
                    }
                    Events::KeyPress(_keys) => {
                        handle_keypress();
                    }
                },
                Err(_e) => {
                    // eprintln!("Failed to parse JSON: {:?}", e)
                }
            }
        }
    }
    Ok(())
}
