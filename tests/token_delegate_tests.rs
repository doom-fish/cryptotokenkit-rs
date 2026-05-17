use std::sync::{Arc, Mutex};

use cryptotokenkit::{
    SmartCard, SmartCardToken, SmartCardTokenDriver, SmartCardTokenDriverDelegate, TKErrorCode,
    Token, TokenAuthOperationHandle, TokenConfigurationSnapshot, TokenDelegate, TokenDriver,
    TokenDriverDelegate, TokenKeyExchangeParameters, TokenObjectId, TokenOperation,
    TokenPasswordAuthOperation, TokenSession, TokenSessionDelegate, TK_ERROR_DOMAIN,
};
use serde_json::{json, Value};

#[derive(Default)]
struct SessionState {
    begin_auth_constraints: Vec<Value>,
    supports_checks: usize,
    sign_payloads: Vec<Vec<u8>>,
    decrypt_payloads: Vec<Vec<u8>>,
    key_exchange_requested_size: Option<isize>,
    key_exchange_shared_info: Option<Vec<u8>>,
}

struct RecordingSessionDelegate {
    state: Arc<Mutex<SessionState>>,
}

impl TokenSessionDelegate for RecordingSessionDelegate {
    fn begin_auth_for_operation(
        &mut self,
        _session: &TokenSession,
        operation: TokenOperation,
        constraint: &Value,
    ) -> Result<Option<TokenAuthOperationHandle>, cryptotokenkit::CryptoTokenKitError> {
        assert_eq!(operation, TokenOperation::SignData);
        self.state
            .lock()
            .unwrap()
            .begin_auth_constraints
            .push(constraint.clone());
        Ok(Some(TokenPasswordAuthOperation::new().into()))
    }

    fn supports_operation(
        &mut self,
        _session: &TokenSession,
        operation: TokenOperation,
        key_object_id: &TokenObjectId,
        algorithm: &cryptotokenkit::TokenKeyAlgorithm,
    ) -> bool {
        {
            let mut state = self.state.lock().unwrap();
            state.supports_checks += 1;
        }
        operation == TokenOperation::SignData
            && key_object_id.0 == "key-1"
            && algorithm.is_algorithm("com.example.base").unwrap()
            && algorithm.supports_algorithm("com.example.extra").unwrap()
    }

    fn sign_data(
        &mut self,
        _session: &TokenSession,
        data: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &cryptotokenkit::TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, cryptotokenkit::CryptoTokenKitError> {
        self.state.lock().unwrap().sign_payloads.push(data.to_vec());
        Ok(data.iter().rev().copied().collect())
    }

    fn decrypt_data(
        &mut self,
        _session: &TokenSession,
        ciphertext: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &cryptotokenkit::TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, cryptotokenkit::CryptoTokenKitError> {
        self.state
            .lock()
            .unwrap()
            .decrypt_payloads
            .push(ciphertext.to_vec());
        Ok(ciphertext.to_vec())
    }

    fn perform_key_exchange(
        &mut self,
        _session: &TokenSession,
        other_party_public_key_data: &[u8],
        _object_id: &TokenObjectId,
        _algorithm: &cryptotokenkit::TokenKeyAlgorithm,
        parameters: &TokenKeyExchangeParameters,
    ) -> Result<Vec<u8>, cryptotokenkit::CryptoTokenKitError> {
        let requested_size = parameters.requested_size();
        let shared_info = parameters.shared_info()?;
        {
            let mut state = self.state.lock().unwrap();
            state.key_exchange_requested_size = Some(requested_size);
            state.key_exchange_shared_info = shared_info;
        }
        let public_key_len = u8::try_from(other_party_public_key_data.len()).map_err(|_| {
            cryptotokenkit::CryptoTokenKitError::InvalidArgument(
                "public key too large for test fixture".into(),
            )
        })?;
        let requested_size = u8::try_from(requested_size).map_err(|_| {
            cryptotokenkit::CryptoTokenKitError::InvalidArgument(
                "requested size out of range for test fixture".into(),
            )
        })?;
        Ok(vec![public_key_len, requested_size])
    }
}

#[derive(Default)]
struct TokenState {
    created_sessions: usize,
    terminated_sessions: usize,
}

struct RecordingTokenDelegate {
    state: Arc<Mutex<TokenState>>,
}

impl TokenDelegate for RecordingTokenDelegate {
    fn create_session(
        &mut self,
        token: &Token,
    ) -> Result<Option<TokenSession>, cryptotokenkit::CryptoTokenKitError> {
        self.state.lock().unwrap().created_sessions += 1;
        Ok(Some(TokenSession::new(token)))
    }

    fn terminate_session(&mut self, _token: &Token, _session: &TokenSession) {
        self.state.lock().unwrap().terminated_sessions += 1;
    }
}

#[derive(Default)]
struct DriverState {
    created_for_instance_ids: Vec<String>,
    terminated_tokens: usize,
}

struct RecordingDriverDelegate {
    state: Arc<Mutex<DriverState>>,
}

impl TokenDriverDelegate for RecordingDriverDelegate {
    fn token_for_configuration(
        &mut self,
        driver: &TokenDriver,
        configuration: &TokenConfigurationSnapshot,
    ) -> Result<Option<Token>, cryptotokenkit::CryptoTokenKitError> {
        self.state
            .lock()
            .unwrap()
            .created_for_instance_ids
            .push(configuration.instance_id.clone());
        Ok(Some(Token::new(driver, &configuration.instance_id)?))
    }

    fn terminate_token(&mut self, _driver: &TokenDriver, _token: &Token) {
        self.state.lock().unwrap().terminated_tokens += 1;
    }
}

#[derive(Default)]
struct SmartCardDriverState {
    aids: Vec<Option<Vec<u8>>>,
    terminated_tokens: usize,
}

struct RecordingSmartCardDriverDelegate {
    state: Arc<Mutex<SmartCardDriverState>>,
}

impl SmartCardTokenDriverDelegate for RecordingSmartCardDriverDelegate {
    fn create_token_for_smart_card(
        &mut self,
        driver: &SmartCardTokenDriver,
        smart_card: &SmartCard,
        aid: Option<&[u8]>,
    ) -> Result<Option<SmartCardToken>, cryptotokenkit::CryptoTokenKitError> {
        self.state
            .lock()
            .unwrap()
            .aids
            .push(aid.map(ToOwned::to_owned));
        Ok(Some(SmartCardToken::new(
            smart_card,
            aid,
            "com.example.smartcard-token",
            driver,
        )?))
    }

    fn terminate_token(&mut self, _driver: &SmartCardTokenDriver, _token: &SmartCardToken) {
        self.state.lock().unwrap().terminated_tokens += 1;
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn token_delegates_and_gap_fill_helpers_work() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(TK_ERROR_DOMAIN, "CryptoTokenKit");
    assert_eq!(
        TKErrorCode::try_from(-5).unwrap(),
        TKErrorCode::AuthenticationFailed
    );

    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.delegate-test")?;
    let session = TokenSession::new(&token);
    let key_id = TokenObjectId::new("key-1");

    let session_state = Arc::new(Mutex::new(SessionState::default()));
    let session_handle = session.set_delegate(RecordingSessionDelegate {
        state: Arc::clone(&session_state),
    })?;
    assert!(session.has_delegate());
    assert_eq!(session.token()?.instance_id()?, token.instance_id()?);

    let auth = session
        .invoke_delegate_begin_auth(TokenOperation::SignData, &json!("pin"))?
        .expect("delegate should return auth context");
    match auth {
        TokenAuthOperationHandle::Password(password) => password.finish()?,
        _ => panic!("expected password auth operation"),
    }
    assert!(session.invoke_delegate_supports_operation(
        TokenOperation::SignData,
        &key_id,
        "com.example.base",
        &["com.example.extra"],
    )?);
    assert_eq!(
        session.invoke_delegate_sign_data(
            b"abc",
            &key_id,
            "com.example.base",
            &["com.example.extra"],
        )?,
        b"cba"
    );
    assert_eq!(
        session.invoke_delegate_decrypt_data(
            b"cipher",
            &key_id,
            "com.example.base",
            &["com.example.extra"],
        )?,
        b"cipher"
    );
    assert_eq!(
        session.invoke_delegate_perform_key_exchange(
            &[0x01, 0x02, 0x03],
            &key_id,
            "com.example.base",
            &["com.example.extra"],
            32,
            Some(&[0xAA, 0xBB]),
        )?,
        vec![3, 32]
    );
    drop(session_handle);
    assert!(!session.has_delegate());

    let token_state = Arc::new(Mutex::new(TokenState::default()));
    let token_handle = token.set_delegate(RecordingTokenDelegate {
        state: Arc::clone(&token_state),
    })?;
    assert!(token.has_delegate());
    let created_session = token
        .invoke_delegate_create_session()?
        .expect("delegate should create a token session");
    assert_eq!(
        created_session.token_instance_id()?,
        "com.example.cryptotokenkit.delegate-test"
    );
    token.invoke_delegate_terminate_session(&created_session);
    drop(token_handle);
    assert!(!token.has_delegate());

    let class_id = "com.example.cryptotokenkit.driver-config";
    let driver_snapshot = TokenDriver::add_token_configuration(class_id, "driver-instance")?;
    assert_eq!(driver_snapshot.instance_id, "driver-instance");
    let configurations = TokenDriver::driver_configurations()?;
    assert!(configurations
        .get(class_id)
        .and_then(|snapshot| snapshot.token_configurations.get("driver-instance"))
        .is_some());

    let driver_state = Arc::new(Mutex::new(DriverState::default()));
    let driver_handle = driver.set_delegate(RecordingDriverDelegate {
        state: Arc::clone(&driver_state),
    })?;
    assert!(driver.has_delegate());
    let delegate_token = driver
        .invoke_delegate_token_for_configuration(&driver_snapshot)?
        .expect("delegate should create token for configuration");
    assert_eq!(delegate_token.instance_id()?, "driver-instance");
    driver.invoke_delegate_terminate_token(&delegate_token);
    drop(driver_handle);
    assert!(!driver.has_delegate());
    TokenDriver::remove_token_configuration(class_id, "driver-instance")?;

    let smart_card_driver = SmartCardTokenDriver::new();
    let mock_smart_card = SmartCard::mock("Mock Token Reader")?;
    let smart_card_state = Arc::new(Mutex::new(SmartCardDriverState::default()));
    let smart_card_handle = smart_card_driver.set_delegate(RecordingSmartCardDriverDelegate {
        state: Arc::clone(&smart_card_state),
    })?;
    assert!(smart_card_driver.has_delegate());
    let smart_card_token = smart_card_driver
        .invoke_delegate_create_token(&mock_smart_card, Some(&[0xA0, 0x00, 0x00, 0x01]))?
        .expect("delegate should create smart-card token");
    assert_eq!(smart_card_token.aid()?, Some(vec![0xA0, 0x00, 0x00, 0x01]));
    smart_card_driver.invoke_delegate_terminate_token(&smart_card_token);
    drop(smart_card_handle);
    assert!(!smart_card_driver.has_delegate());

    {
        let session_state = session_state.lock().unwrap();
        assert_eq!(session_state.begin_auth_constraints, vec![json!("pin")]);
        assert_eq!(session_state.supports_checks, 1);
        assert_eq!(session_state.sign_payloads, vec![b"abc".to_vec()]);
        assert_eq!(session_state.decrypt_payloads, vec![b"cipher".to_vec()]);
        assert_eq!(session_state.key_exchange_requested_size, Some(32));
        assert_eq!(
            session_state.key_exchange_shared_info,
            Some(vec![0xAA, 0xBB])
        );
        drop(session_state);
    }

    {
        let token_state = token_state.lock().unwrap();
        assert_eq!(token_state.created_sessions, 1);
        assert_eq!(token_state.terminated_sessions, 1);
        drop(token_state);
    }

    {
        let driver_state = driver_state.lock().unwrap();
        assert_eq!(
            driver_state.created_for_instance_ids,
            vec!["driver-instance".to_string()]
        );
        assert_eq!(driver_state.terminated_tokens, 1);
        drop(driver_state);
    }

    {
        let smart_card_state = smart_card_state.lock().unwrap();
        assert_eq!(
            smart_card_state.aids,
            vec![Some(vec![0xA0, 0x00, 0x00, 0x01])]
        );
        assert_eq!(smart_card_state.terminated_tokens, 1);
        drop(smart_card_state);
    }

    Ok(())
}
