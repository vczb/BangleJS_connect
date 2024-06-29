use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;

pub async fn pair_device() -> Result<Peripheral, Box<dyn Error>> {
    // Create a new manager instance
    let manager = Manager::new().await.map_err(|e| {
        println!("Failed to create manager: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    // Get the first Bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).ok_or_else(|| {
        let err_msg = "No Bluetooth adapters found";
        println!("{}", err_msg);
        Box::<dyn Error>::from(err_msg)
    })?;

    // Start scanning for devices
    central
        .start_scan(ScanFilter::default())
        .await
        .map_err(|e| {
            println!("Failed to start scan: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;
    println!("Scanning for devices...");
    time::sleep(Duration::from_secs(2)).await;

    // Find the device we're interested in
    let light = find_light(&central).await.ok_or_else(|| {
        println!("No device found!!!");
        Box::<dyn Error>::from("Device not found")
    })?;

    // Check if already connected
    if !light.is_connected().await? {
        println!("Not connected, attempting to connect...");
        connect_device(&light).await?;
    } else {
        println!("Already connected");
    }

    Ok(light)
}

async fn connect_device(light: &Peripheral) -> Result<bool, Box<dyn Error>> {
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

async fn find_light(central: &Adapter) -> Option<Peripheral> {
    const DEVICE_NAME: &str = "Bangle";
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
                                .any(|name| name.contains(DEVICE_NAME))
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
