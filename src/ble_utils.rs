use btleplug::api::{Central, Peripheral as _};
use btleplug::platform::{Adapter, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

pub async fn connect_device(light: &Peripheral) -> Result<bool, Box<dyn Error>> {
    const MAX_RETRIES: usize = 3;
    let mut retry_count = 0;

    loop {
        // Attempt to connect to the device
        match light.connect().await {
            Ok(_) => {
                println!("Connected to device");
                // Attempt to discover services
                match light.discover_services().await {
                    Ok(_) => {
                        println!("Services discovered");
                        return Ok(true); // Successfully connected and discovered services
                    }
                    Err(e) => {
                        println!("Failed to discover services: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect to device: {}", e);
            }
        }

        // Retry logic
        retry_count += 1;
        if retry_count >= MAX_RETRIES {
            break; // Exit loop if maximum retries reached
        } else {
            println!("Retrying in 2 seconds...");
            time::sleep(Duration::from_secs(2)).await;
        }
    }

    // If maximum retries reached without success, return false
    println!("Maximum retries reached, failed to connect or discover services");
    Ok(false)
}

/* Get device by name */
pub async fn find_light(central: &Adapter, device_name: &str) -> Option<Peripheral> {
    const MAX_RETRIES: usize = 3;
    let mut retry_count = 0;

    loop {
        match central.peripherals().await {
            Ok(peripherals) => {
                for p in peripherals {
                    match p.properties().await {
                        Ok(Some(properties)) => {
                            if properties
                                .local_name
                                .iter()
                                .any(|name| name.contains(&device_name))
                            {
                                return Some(p);
                            }
                        }
                        Ok(None) => {
                            println!("No device found!");
                            continue;
                        }
                        Err(e) => println!("Failed to get properties for a peripheral: {}", e),
                    }
                }
            }
            Err(e) => println!("Failed to get peripherals: {}", e),
        }

        // Retry logic
        retry_count += 1;
        if retry_count >= MAX_RETRIES {
            break; // Exit loop if maximum retries reached
        } else {
            println!("Retrying in 2 seconds...");
            time::sleep(Duration::from_secs(2)).await;
        }
    }

    None
}

pub async fn is_device_connected(peripheral: &Peripheral) -> bool {
    let is_connected_wrap = peripheral.is_connected().await;

    let is_connected = is_connected_wrap.unwrap();

    return is_connected;
}
