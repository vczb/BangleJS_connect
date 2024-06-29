use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time; // Adding log crate for logging

#[tokio::main]
pub async fn pair_device() -> Result<(), Box<dyn Error>> {
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
    let find_light_result = find_light(&central).await;

    let light = match find_light_result {
        Some(peripheral) => peripheral,
        None => {
            println!("No device found!!!");
            return Ok(());
        }
    };

    // Connect to the device
    light.connect().await.map_err(|e| {
        println!("Failed to connect to device: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;
    println!("Connected to device");

    // Discover services
    light.discover_services().await.map_err(|e| {
        println!("Failed to discover services: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;
    println!("Services discovered");

    Ok(())
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
                            //dbg!(&properties);

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
