mod plan;
mod executor;
mod device_discovery;

use device_discovery::discover_all_devices;
use plan::build_plan;
use executor::execute_wipe;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let report = discover_all_devices().await?;
    
    for device in &report.devices {
        println!("--- DEVICE DEBUG ---");
        println!("name        : {}", device.name);
        println!("label       : {:?}", device.label);
        println!("sysfs       : {}", device.sysfs_path);
        println!("bus         : {:?}", device.bus);
        println!("erase_cap   : {:?}", device.erase_capability);

        // Build wipe plan in simulation mode
        if let Some(plan) = build_plan(&device) {
            let _cert = execute_wipe(&device, &plan, true); // true to simulate
        } else {
            println!("No wipe plan available for device: {}", device.name);
        }
    }

    println!("{}", serde_json::to_string_pretty(&report)?);

    Ok(())
}