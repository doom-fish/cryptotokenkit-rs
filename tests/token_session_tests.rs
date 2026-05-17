use cryptotokenkit::{
    SmartCardPinFormat, Token, TokenAuthOperation, TokenDriver, TokenPasswordAuthOperation,
    TokenSession, TokenSmartCardPinAuthOperation,
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
    assert_eq!(password.password()?.as_deref(), Some("1234"));
    password.finish()?;

    let pin = TokenSmartCardPinAuthOperation::new();
    pin.set_pin_format(SmartCardPinFormat::default())?;
    pin.set_apdu_template(Some(vec![0x00, 0x20, 0x00, 0x00]))?;
    pin.set_pin_byte_offset(0)?;
    pin.set_pin(Some("1234"))?;
    assert_eq!(pin.pin()?.as_deref(), Some("1234"));
    pin.finish()?;
    Ok(())
}
