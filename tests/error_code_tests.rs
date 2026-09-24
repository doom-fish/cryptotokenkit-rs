use cryptotokenkit::{
    CryptoTokenKitError, TKErrorCode, Token, TokenDriver, TokenKeyAlgorithm, TokenObjectId,
    TokenSession, TokenSessionDelegate,
};

struct FailingDelegate {
    sign_error: fn() -> CryptoTokenKitError,
}

impl TokenSessionDelegate for FailingDelegate {
    fn sign_data(
        &mut self,
        _session: &TokenSession,
        _data: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        Err((self.sign_error)())
    }
}

fn sign_error_through_delegate(sign_error: fn() -> CryptoTokenKitError) -> CryptoTokenKitError {
    let driver = TokenDriver::new();
    let token = Token::new(&driver, "com.example.cryptotokenkit.error-codes").expect("token");
    let session = TokenSession::new(&token).expect("session");
    let _handle = session
        .set_delegate(FailingDelegate { sign_error })
        .expect("delegate");
    session
        .invoke_delegate_sign_data(
            b"payload",
            &TokenObjectId::new("key"),
            "com.example.base",
            &[],
        )
        .expect_err("delegate must fail")
}

#[test]
fn every_sdk_error_code_round_trips() {
    for (raw, code) in [
        (-1, TKErrorCode::NotImplemented),
        (-2, TKErrorCode::CommunicationError),
        (-3, TKErrorCode::CorruptedData),
        (-4, TKErrorCode::CanceledByUser),
        (-5, TKErrorCode::AuthenticationFailed),
        (-6, TKErrorCode::ObjectNotFound),
        (-7, TKErrorCode::TokenNotFound),
        (-8, TKErrorCode::BadParameter),
        (-9, TKErrorCode::AuthenticationNeeded),
        (-10, TKErrorCode::InvalidatedDeviceKey),
    ] {
        assert_eq!(TKErrorCode::from_raw(raw), Some(code));
        assert_eq!(code as i32, raw);
    }
    assert_eq!(TKErrorCode::from_raw(-11), None);
    assert_eq!(TKErrorCode::try_from(0), Err(0));
}

#[test]
fn bridge_errors_do_not_alias_sdk_error_codes() {
    for error in [
        CryptoTokenKitError::InvalidArgument("bad".into()),
        CryptoTokenKitError::FrameworkError("failed".into()),
        CryptoTokenKitError::TimedOut("slow".into()),
        CryptoTokenKitError::Unsupported("missing".into()),
    ] {
        assert_eq!(error.framework_code(), None, "{error:?}");
    }
}

#[test]
fn sdk_error_codes_from_delegates_keep_their_meaning() {
    let corrupted = sign_error_through_delegate(|| CryptoTokenKitError::Unknown {
        code: TKErrorCode::CorruptedData as i32,
        message: "corrupted".into(),
    });
    assert!(
        !matches!(corrupted, CryptoTokenKitError::TimedOut(_)),
        "{corrupted:?}"
    );
    assert_eq!(corrupted.framework_code(), Some(TKErrorCode::CorruptedData));
    assert_eq!(corrupted.message(), "corrupted");

    let not_implemented = sign_error_through_delegate(|| CryptoTokenKitError::Unknown {
        code: TKErrorCode::NotImplemented as i32,
        message: "no".into(),
    });
    assert!(
        !matches!(not_implemented, CryptoTokenKitError::InvalidArgument(_)),
        "{not_implemented:?}"
    );
    assert_eq!(
        not_implemented.framework_code(),
        Some(TKErrorCode::NotImplemented)
    );
}

#[test]
fn bridge_errors_from_delegates_map_to_sdk_error_codes() {
    let invalid = sign_error_through_delegate(|| CryptoTokenKitError::InvalidArgument("x".into()));
    assert_eq!(invalid.framework_code(), Some(TKErrorCode::BadParameter));

    let unsupported = sign_error_through_delegate(|| CryptoTokenKitError::Unsupported("x".into()));
    assert_eq!(
        unsupported.framework_code(),
        Some(TKErrorCode::NotImplemented)
    );

    let success_code = sign_error_through_delegate(|| CryptoTokenKitError::Unknown {
        code: 0,
        message: "zero".into(),
    });
    assert_eq!(
        success_code.framework_code(),
        Some(TKErrorCode::CommunicationError)
    );
}
