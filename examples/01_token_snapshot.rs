use cryptotokenkit::{Token, TokenDriver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.token")?;
    token.set_configuration_data(Some(b"token-config"))?;

    match token.configuration() {
        Ok(snapshot) => {
            println!("instance-id: {}", snapshot.instance_id);
            println!("configuration-bytes: {}", snapshot.configuration_data.map_or(0, |bytes| bytes.len()));
            println!("keychain-items: {}", snapshot.keychain_items.len());
        }
        Err(error) => println!("token configuration unavailable: {error}"),
    }

    println!("✅ token snapshot OK");
    Ok(())
}
