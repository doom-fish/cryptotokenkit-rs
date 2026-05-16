use std::sync::{Arc, Mutex};

use cryptotokenkit::{
    SmartCard, SmartCardToken, SmartCardTokenDriver, SmartCardTokenDriverDelegate, Token,
    TokenConfigurationSnapshot, TokenDelegate, TokenDriver, TokenDriverDelegate, TokenObjectId,
    TokenOperation, TokenPasswordAuthOperation, TokenSession, TokenSessionDelegate,
};
use serde_json::json;

struct SessionDelegate {
    log: Arc<Mutex<Vec<String>>>,
}

impl TokenSessionDelegate for SessionDelegate {
    fn begin_auth_for_operation(
        &mut self,
        _session: &TokenSession,
        operation: TokenOperation,
        constraint: &serde_json::Value,
    ) -> Result<Option<cryptotokenkit::TokenAuthOperationHandle>, cryptotokenkit::CryptoTokenKitError>
    {
        self.log
            .lock()
            .unwrap()
            .push(format!("begin-auth:{operation:?}:{constraint}"));
        Ok(Some(TokenPasswordAuthOperation::new().into()))
    }
}

struct TokenDelegateImpl;

impl TokenDelegate for TokenDelegateImpl {
    fn create_session(
        &mut self,
        token: &Token,
    ) -> Result<Option<TokenSession>, cryptotokenkit::CryptoTokenKitError> {
        Ok(Some(TokenSession::new(token)))
    }
}

struct DriverDelegateImpl;

impl TokenDriverDelegate for DriverDelegateImpl {
    fn token_for_configuration(
        &mut self,
        driver: &TokenDriver,
        configuration: &TokenConfigurationSnapshot,
    ) -> Result<Option<Token>, cryptotokenkit::CryptoTokenKitError> {
        Ok(Some(Token::new(driver, &configuration.instance_id)?))
    }
}

struct SmartCardDriverDelegateImpl;

impl SmartCardTokenDriverDelegate for SmartCardDriverDelegateImpl {
    fn create_token_for_smart_card(
        &mut self,
        driver: &SmartCardTokenDriver,
        smart_card: &SmartCard,
        aid: Option<&[u8]>,
    ) -> Result<Option<SmartCardToken>, cryptotokenkit::CryptoTokenKitError> {
        Ok(Some(SmartCardToken::new(
            smart_card,
            aid,
            "com.example.delegate-example.smartcard",
            driver,
        )?))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.delegate-example.token")?;
    let session = TokenSession::new(&token);
    let session_log = Arc::new(Mutex::new(Vec::new()));
    let _session_handle = session.set_delegate(SessionDelegate {
        log: Arc::clone(&session_log),
    })?;
    let _ = session.invoke_delegate_begin_auth(TokenOperation::SignData, &json!("pin"))?;

    let _token_handle = token.set_delegate(TokenDelegateImpl)?;
    let created_session = token
        .invoke_delegate_create_session()?
        .expect("token delegate should create a session");

    let configuration = TokenDriver::add_token_configuration(
        "com.example.delegate-example.driver",
        "com.example.delegate-example.config",
    )?;
    let _driver_handle = driver.set_delegate(DriverDelegateImpl)?;
    let configured_token = driver
        .invoke_delegate_token_for_configuration(&configuration)?
        .expect("driver delegate should create a token");
    TokenDriver::remove_token_configuration(
        "com.example.delegate-example.driver",
        "com.example.delegate-example.config",
    )?;

    let smart_card_driver = SmartCardTokenDriver::new();
    let mock_card = SmartCard::mock("Example Mock Reader")?;
    let _smart_card_handle = smart_card_driver.set_delegate(SmartCardDriverDelegateImpl)?;
    let smart_card_token = smart_card_driver
        .invoke_delegate_create_token(&mock_card, Some(&[0xA0, 0x00, 0x00, 0x01]))?
        .expect("smart-card delegate should create a token");

    println!("token-session: {}", created_session.token_instance_id()?);
    println!("configured-token: {}", configured_token.instance_id()?);
    println!("smart-card-token-aid: {:?}", smart_card_token.aid()?);
    println!("session-log: {:?}", session_log.lock().unwrap().clone());
    println!("key-id: {}", TokenObjectId::new("example-key").0);
    Ok(())
}
