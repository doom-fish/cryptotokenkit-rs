use cryptotokenkit::{
    CryptoTokenKitError, SmartCardPinFormat, Token, TokenAuthOperation, TokenDriver,
    TokenPasswordAuthOperation, TokenSession, TokenSmartCardPinAuthOperation,
};

#[test]
fn token_session_and_auth_operations_work() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.session-test")?;
    let session = TokenSession::new(&token);

    assert_eq!(
        session.token_instance_id()?,
        "com.example.cryptotokenkit.session-test"
    );

    let auth = TokenAuthOperation::new();
    auth.finish()?;

    let password = TokenPasswordAuthOperation::new();
    password.set_password(Some("1234"))?;
    assert_eq!(
        password
            .password()?
            .as_ref()
            .map(|password| password.as_str()),
        Some("1234")
    );
    password.finish()?;

    let pin = TokenSmartCardPinAuthOperation::new();
    pin.set_pin_format(SmartCardPinFormat::default())?;
    pin.set_apdu_template(Some(vec![0x00, 0x20, 0x00, 0x00]))?;
    pin.set_pin_byte_offset(0)?;
    pin.set_pin(Some("1234"))?;
    assert_eq!(pin.pin()?.as_ref().map(|pin| pin.as_str()), Some("1234"));
    assert!(!pin.has_smart_card()?);
    let error = pin
        .finish()
        .expect_err("a PIN operation without a smart card cannot finish");
    assert!(
        matches!(error, CryptoTokenKitError::FrameworkError(_)),
        "{error:?}"
    );
    Ok(())
}

#[test]
fn passwords_and_pins_round_trip_through_dedicated_buffers(
) -> Result<(), Box<dyn std::error::Error>> {
    let password = TokenPasswordAuthOperation::new();
    assert!(password.password()?.is_none());
    password.set_password(Some(""))?;
    assert_eq!(
        password
            .password()?
            .as_ref()
            .map(|password| password.as_str()),
        Some("")
    );
    password.set_password(Some("pässwörd-🔑"))?;
    assert_eq!(
        password
            .password()?
            .as_ref()
            .map(|password| password.as_str()),
        Some("pässwörd-🔑")
    );
    let error = password
        .set_password(Some("12\u{0}34"))
        .expect_err("NUL bytes are rejected");
    assert!(matches!(error, CryptoTokenKitError::InvalidArgument(_)));
    assert!(!error.message().contains("12"));
    password.set_password(None)?;
    assert!(password.password()?.is_none());

    let pin = TokenSmartCardPinAuthOperation::new();
    assert!(pin.pin()?.is_none());
    pin.set_pin(Some("246810"))?;
    pin.set_pin_format(SmartCardPinFormat::default())?;
    pin.set_apdu_template(Some(vec![0x00, 0x20, 0x00, 0x80]))?;
    pin.set_pin_byte_offset(5)?;
    assert_eq!(pin.pin()?.as_ref().map(|pin| pin.as_str()), Some("246810"));
    assert_eq!(pin.pin_byte_offset()?, 5);
    pin.set_pin(None)?;
    assert!(pin.pin()?.is_none());
    assert_eq!(pin.apdu_template()?, Some(vec![0x00, 0x20, 0x00, 0x80]));
    Ok(())
}
