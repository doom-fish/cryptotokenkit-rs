use cryptotokenkit::{Token, TokenDriver};

#[test]
fn token_configuration_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.token-test")?;
    token.set_configuration_data(Some(b"token-config"))?;

    let snapshot = token.configuration()?;
    assert_eq!(snapshot.instance_id, "com.example.cryptotokenkit.token-test");
    assert_eq!(snapshot.configuration_data, Some(b"token-config".to_vec()));
    assert!(snapshot.keychain_items.is_empty());
    assert_eq!(token.instance_id()?, snapshot.instance_id);
    Ok(())
}
