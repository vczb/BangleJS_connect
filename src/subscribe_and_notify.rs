use btleplug::api::{Peripheral, ValueNotification};
use futures::stream::StreamExt;
use std::error::Error;
//use subscribe_and_notify::event_listener;

pub async fn event_listener(
    peripheral: btleplug::platform::Peripheral,
) -> Result<(), Box<dyn Error>> {
    // Create a stream of notifications/events
    let mut notifications = peripheral.notifications().await?;

    // Listen for notifications/events in an infinite loop
    loop {
        // Await the next notification/event
        match notifications.next().await {
            Some(notification) => {
                // Handle the received notification
                handle_notification(&notification);
            }
            //Some(e) => {
            //    println!("Error receiving notification: {}", e);
            //    // Handle the error as needed
            //}
            None => {
                println!("Notification stream ended unexpectedly.");
                break; // Exit the loop if the stream ends unexpectedly
            }
        }
    }

    Ok(())
}

fn handle_notification(notification: &ValueNotification) {
    println!(
        "Received notification from characteristic {:?}: {:?}",
        notification.uuid, notification.value
    );
}
