use cryptotokenkit::{CryptoTokenKitError, SmartCardTokenDriver, TokenDriver};

#[test]
fn token_driver_configurations_come_from_the_framework() -> Result<(), Box<dyn std::error::Error>> {
    let _driver = TokenDriver::new();
    let _smart_card_driver = SmartCardTokenDriver::new();
    let class_id = "com.example.cryptotokenkit.not-hosted";

    let configurations = TokenDriver::driver_configurations()?;
    assert!(!configurations.contains_key(class_id));
    for (key, configuration) in &configurations {
        assert_eq!(key, &configuration.class_id);
    }

    let error = TokenDriver::add_token_configuration(class_id, "instance")
        .expect_err("the class is not hosted by this process");
    assert!(
        matches!(error, CryptoTokenKitError::Unsupported(_)),
        "{error:?}"
    );
    let error = TokenDriver::remove_token_configuration(class_id, "instance")
        .expect_err("the class is not hosted by this process");
    assert!(
        matches!(error, CryptoTokenKitError::Unsupported(_)),
        "{error:?}"
    );
    assert!(!TokenDriver::driver_configurations()?.contains_key(class_id));
    Ok(())
}
