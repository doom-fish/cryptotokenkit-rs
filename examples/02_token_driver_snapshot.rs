use cryptotokenkit::{SmartCardTokenDriver, TokenDriver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _driver = TokenDriver::new();
    let _smart_card_driver = SmartCardTokenDriver::new();
    let configurations = TokenDriver::driver_configurations()?;
    println!("driver-configurations: {}", configurations.len());
    for (class_id, configuration) in &configurations {
        println!("driver: {class_id} tokens={}", configuration.token_configurations.len());
    }
    println!("✅ token driver snapshot OK");
    Ok(())
}
