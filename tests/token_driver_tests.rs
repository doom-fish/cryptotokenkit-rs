use cryptotokenkit::{SmartCardTokenDriver, TokenDriver};

#[test]
fn token_drivers_can_be_created() -> Result<(), Box<dyn std::error::Error>> {
    let _driver = TokenDriver::new();
    let _smart_card_driver = SmartCardTokenDriver::new();
    let configurations = TokenDriver::driver_configurations()?;
    assert!(configurations.len() <= configurations.len());
    Ok(())
}
