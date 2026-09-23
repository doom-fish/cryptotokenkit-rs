use cryptotokenkit::{CryptoTokenKitError, Token, TokenDriver};

#[test]
fn token_configuration_reflects_the_framework() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.token-test")?;
    match token.set_configuration_data(Some(b"token-config")) {
        Ok(()) => assert_eq!(
            token.configuration()?.configuration_data,
            Some(b"token-config".to_vec())
        ),
        Err(error) => {
            assert!(
                matches!(error, CryptoTokenKitError::Unsupported(_)),
                "{error:?}"
            );
            assert_eq!(token.configuration()?.configuration_data, None);
            token.set_configuration_data(None)?;
        }
    }

    let snapshot = token.configuration()?;
    assert_eq!(
        snapshot.instance_id,
        "com.example.cryptotokenkit.token-test"
    );
    assert!(snapshot.keychain_items.is_empty());
    assert_eq!(token.instance_id()?, snapshot.instance_id);
    Ok(())
}
