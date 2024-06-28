use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use log::{error, info, warn};
use std::error::Error;
use std::time::Duration;
use tokio::time; // Adding log crate for logging

#[tokio::main]
pub async fn pair_device() -> Result<(), Box<dyn Error>> {
    env_logger::init(); // Initialize the logger

    // Create a new manager instance
    let manager = Manager::new().await.map_err(|e| {
        error!("Failed to create manager: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;

    // Get the first Bluetooth adapter
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().nth(0).ok_or_else(|| {
        let err_msg = "No Bluetooth adapters found";
        error!("{}", err_msg);
        Box::<dyn Error>::from(err_msg)
    })?;

    // Start scanning for devices
    central
        .start_scan(ScanFilter::default())
        .await
        .map_err(|e| {
            error!("Failed to start scan: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;
    info!("Scanning for devices...");
    time::sleep(Duration::from_secs(2)).await;

    // Find the device we're interested in
    let find_light_result = find_light(&central).await;

    let light = match find_light_result {
        Some(peripheral) => peripheral,
        None => {
            warn!("No device found!!!");
            return Ok(());
        }
    };

    // Connect to the device
    light.connect().await.map_err(|e| {
        error!("Failed to connect to device: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;
    info!("Connected to device");

    // Discover services
    light.discover_services().await.map_err(|e| {
        error!("Failed to discover services: {}", e);
        Box::new(e) as Box<dyn Error>
    })?;
    info!("Services discovered");

    Ok(())
}

async fn find_light(central: &Adapter) -> Option<Peripheral> {
    match central.peripherals().await {
        Ok(peripherals) => {
            for p in peripherals {
                match p.properties().await {
                    Ok(Some(properties)) => {
                        if properties
                            .local_name
                            .iter()
                            .any(|name| name.contains("Bangle"))
                        {
                            return Some(p);
                        }
                    }
                    Ok(None) => continue,
                    Err(e) => warn!("Failed to get properties for a peripheral: {}", e),
                }
            }
        }
        Err(e) => error!("Failed to get peripherals: {}", e),
    }
    None
}
