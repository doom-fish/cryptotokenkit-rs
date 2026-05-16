use cryptotokenkit::{
    SmartCardPinFormat, Token, TokenAuthOperation, TokenDriver, TokenPasswordAuthOperation,
    TokenSession, TokenSmartCardPinAuthOperation,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.session")?;
    let session = TokenSession::new(&token);

    match session.token_instance_id() {
        Ok(instance_id) => println!("session token: {instance_id}"),
        Err(error) => println!("session token unavailable: {error}"),
    }

    let auth = TokenAuthOperation::new();
    auth.finish()?;

    let password = TokenPasswordAuthOperation::new();
    password.set_password(Some("1234"))?;
    password.finish()?;

    let pin = TokenSmartCardPinAuthOperation::new();
    pin.set_pin(Some("1234"))?;
    pin.set_apdu_template(Some(vec![0x00, 0x20, 0x00, 0x00]))?;
    pin.set_pin_format(SmartCardPinFormat::default())?;
    pin.finish()?;

    println!("✅ token session auth ops OK");
    Ok(())
}
