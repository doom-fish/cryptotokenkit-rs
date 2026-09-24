use core::ffi::{c_char, c_void};
use std::ptr;

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, Zeroizing};

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::{checked, decode_json, encode_json_cstring, status_result};
use crate::smart_card::{SmartCard, SmartCardPinFormat};
use crate::token::{SmartCardToken, Token};

/// Wraps `TKTokenSession`.
pub struct TokenSession {
    raw: *mut c_void,
}

/// Wraps `TKSmartCardTokenSession`.
pub struct SmartCardTokenSession {
    raw: *mut c_void,
}

/// Wraps `TKTokenAuthOperation`.
pub struct TokenAuthOperation {
    raw: *mut c_void,
}

/// Wraps `TKTokenPasswordAuthOperation`.
pub struct TokenPasswordAuthOperation {
    raw: *mut c_void,
}

/// Wraps `TKTokenSmartCardPINAuthOperation`.
pub struct TokenSmartCardPinAuthOperation {
    raw: *mut c_void,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TokenSmartCardPinAuthOperationSnapshot {
    pub pin_format: SmartCardPinFormat,
    pub apdu_template: Option<Vec<u8>>,
    pub pin_byte_offset: i64,
    pub has_smart_card: bool,
}

type SecretGetter =
    unsafe extern "C" fn(*mut c_void, *mut *mut u8, *mut usize, *mut *mut c_char) -> i32;
type SecretSetter =
    unsafe extern "C" fn(*mut c_void, *const u8, usize, bool, *mut *mut c_char) -> i32;

fn read_secret(
    operation: *mut c_void,
    getter: SecretGetter,
) -> Result<Option<Zeroizing<String>>, CryptoTokenKitError> {
    let mut bytes = ptr::null_mut();
    let mut len = 0usize;
    let mut error_ptr = ptr::null_mut();
    let status = unsafe { getter(operation, &raw mut bytes, &raw mut len, &raw mut error_ptr) };
    status_result(status, error_ptr)?;
    if bytes.is_null() {
        return Ok(None);
    }
    let owned = unsafe { std::slice::from_raw_parts(bytes, len) }.to_vec();
    unsafe { std::slice::from_raw_parts_mut(bytes, len) }.zeroize();
    unsafe { libc::free(bytes.cast()) };
    match String::from_utf8(owned) {
        Ok(secret) => Ok(Some(Zeroizing::new(secret))),
        Err(error) => {
            error.into_bytes().zeroize();
            Err(CryptoTokenKitError::FrameworkError(
                "the framework returned a secret that is not valid UTF-8".into(),
            ))
        }
    }
}

fn write_secret(
    operation: *mut c_void,
    setter: SecretSetter,
    secret: Option<&str>,
    kind: &str,
) -> Result<(), CryptoTokenKitError> {
    if secret.is_some_and(|secret| secret.as_bytes().contains(&0)) {
        return Err(CryptoTokenKitError::InvalidArgument(format!(
            "{kind} must not contain NUL bytes"
        )));
    }
    let (secret_ptr, secret_len) =
        secret.map_or((ptr::null(), 0), |secret| (secret.as_ptr(), secret.len()));
    let mut error_ptr = ptr::null_mut();
    let status = unsafe {
        setter(
            operation,
            secret_ptr,
            secret_len,
            secret.is_some(),
            &raw mut error_ptr,
        )
    };
    status_result(status, error_ptr)
}

impl TokenSession {
    /// Creates a new wrapper around `TKTokenSession`.
    pub fn new(token: &Token) -> Result<Self, CryptoTokenKitError> {
        let raw = checked(|error| unsafe {
            ffi::token_session::ctk_token_session_new(token.raw(), error)
        })?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token session".into(),
            ));
        }
        Ok(Self { raw })
    }

    #[must_use]
    pub(crate) const fn raw(&self) -> *mut c_void {
        self.raw
    }

    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn into_raw(mut self) -> *mut c_void {
        let raw = self.raw;
        self.raw = ptr::null_mut();
        raw
    }

    /// Returns the corresponding `TKTokenSession` value.
    pub fn token_instance_id(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = checked(|error| unsafe {
            ffi::token_session::ctk_token_session_token_instance_id(self.raw, error)
        })?;
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null session token identifier".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }
}

impl SmartCardTokenSession {
    /// Creates a new wrapper around `TKSmartCardTokenSession`.
    pub fn new(token: &SmartCardToken) -> Result<Self, CryptoTokenKitError> {
        let raw = checked(|error| unsafe {
            ffi::token_session::ctk_smart_card_token_session_new(token.raw(), error)
        })?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card token session".into(),
            ));
        }
        Ok(Self { raw })
    }

    /// Returns the corresponding `TKSmartCardTokenSession` value.
    pub fn token_instance_id(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = checked(|error| unsafe {
            ffi::token_session::ctk_token_session_token_instance_id(self.raw, error)
        })?;
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null session token identifier".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    /// Returns the corresponding `TKSmartCardTokenSession` value.
    pub fn smart_card(&self) -> Result<Option<SmartCard>, CryptoTokenKitError> {
        let raw = checked(|error| unsafe {
            ffi::token_session::ctk_smart_card_token_session_smart_card(self.raw, error)
        })?;
        Ok((!raw.is_null()).then_some(SmartCard::from_raw(raw)))
    }

    /// Returns the corresponding `TKSmartCardTokenSession` value via the reply-based framework entry point.
    pub fn get_smart_card(&self) -> Result<Option<SmartCard>, CryptoTokenKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_smart_card_token_session_get_smart_card(
                self.raw,
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then_some(SmartCard::from_raw(raw)))
    }
}

impl TokenAuthOperation {
    #[must_use]
    /// Creates a new wrapper around `TKTokenAuthOperation`.
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_session::ctk_token_auth_operation_new() };
        assert!(
            !raw.is_null(),
            "Swift bridge returned a null token auth operation"
        );
        Self { raw }
    }

    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn into_raw(mut self) -> *mut c_void {
        let raw = self.raw;
        self.raw = ptr::null_mut();
        raw
    }

    /// Invokes the corresponding `TKTokenAuthOperation` operation.
    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_auth_operation_finish(self.raw, &raw mut error_ptr)
        };
        status_result(status, error_ptr)
    }
}

impl Default for TokenAuthOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenPasswordAuthOperation {
    #[must_use]
    /// Creates a new wrapper around `TKTokenPasswordAuthOperation`.
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_session::ctk_token_password_auth_operation_new() };
        assert!(
            !raw.is_null(),
            "Swift bridge returned a null token password auth operation"
        );
        Self { raw }
    }

    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn into_raw(mut self) -> *mut c_void {
        let raw = self.raw;
        self.raw = ptr::null_mut();
        raw
    }

    /// Wraps the corresponding `TKTokenPasswordAuthOperation` operation.
    pub fn password(&self) -> Result<Option<Zeroizing<String>>, CryptoTokenKitError> {
        read_secret(
            self.raw,
            ffi::token_session::ctk_token_password_auth_operation_password,
        )
    }

    /// Sets the corresponding `TKTokenPasswordAuthOperation` value.
    pub fn set_password(&self, password: Option<&str>) -> Result<(), CryptoTokenKitError> {
        write_secret(
            self.raw,
            ffi::token_session::ctk_token_password_auth_operation_set_password,
            password,
            "passwords",
        )
    }

    /// Invokes the corresponding `TKTokenPasswordAuthOperation` operation.
    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_auth_operation_finish(self.raw, &raw mut error_ptr)
        };
        status_result(status, error_ptr)
    }
}

impl Default for TokenPasswordAuthOperation {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenSmartCardPinAuthOperation {
    #[must_use]
    /// Creates a new wrapper around `TKTokenSmartCardPINAuthOperation`.
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_session::ctk_token_smart_card_pin_auth_operation_new() };
        assert!(
            !raw.is_null(),
            "Swift bridge returned a null smart-card PIN auth operation"
        );
        Self { raw }
    }

    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub(crate) fn into_raw(mut self) -> *mut c_void {
        let raw = self.raw;
        self.raw = ptr::null_mut();
        raw
    }

    fn snapshot(&self) -> Result<TokenSmartCardPinAuthOperationSnapshot, CryptoTokenKitError> {
        let ptr = checked(|error| unsafe {
            ffi::token_session::ctk_token_smart_card_pin_auth_operation_json(self.raw, error)
        })?;
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card PIN auth operation snapshot".into(),
            ));
        }
        decode_json(ptr)
    }

    fn update(
        &self,
        snapshot: &TokenSmartCardPinAuthOperationSnapshot,
    ) -> Result<(), CryptoTokenKitError> {
        let payload = encode_json_cstring(snapshot)?;
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_smart_card_pin_auth_operation_update_json(
                self.raw,
                payload.as_ptr(),
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Wraps the corresponding `TKTokenSmartCardPINAuthOperation` operation.
    pub fn pin_format(&self) -> Result<SmartCardPinFormat, CryptoTokenKitError> {
        Ok(self.snapshot()?.pin_format)
    }

    /// Sets the corresponding `TKTokenSmartCardPINAuthOperation` value.
    pub fn set_pin_format(
        &self,
        pin_format: SmartCardPinFormat,
    ) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.pin_format = pin_format;
        self.update(&snapshot)
    }

    /// Wraps the corresponding `TKTokenSmartCardPINAuthOperation` operation.
    pub fn apdu_template(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        Ok(self.snapshot()?.apdu_template)
    }

    /// Sets the corresponding `TKTokenSmartCardPINAuthOperation` value.
    pub fn set_apdu_template(
        &self,
        apdu_template: Option<Vec<u8>>,
    ) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.apdu_template = apdu_template;
        self.update(&snapshot)
    }

    /// Wraps the corresponding `TKTokenSmartCardPINAuthOperation` operation.
    pub fn pin_byte_offset(&self) -> Result<i64, CryptoTokenKitError> {
        Ok(self.snapshot()?.pin_byte_offset)
    }

    /// Sets the corresponding `TKTokenSmartCardPINAuthOperation` value.
    pub fn set_pin_byte_offset(&self, pin_byte_offset: i64) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.pin_byte_offset = pin_byte_offset;
        self.update(&snapshot)
    }

    /// Returns whether `TKTokenSmartCardPINAuthOperation` currently has the associated bridge state.
    pub fn has_smart_card(&self) -> Result<bool, CryptoTokenKitError> {
        Ok(self.snapshot()?.has_smart_card)
    }

    /// Sets the corresponding `TKTokenSmartCardPINAuthOperation` value.
    pub fn set_smart_card(
        &self,
        smart_card: Option<&SmartCard>,
    ) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let raw = smart_card.map_or(ptr::null_mut(), SmartCard::raw);
        let status = unsafe {
            ffi::token_session::ctk_token_smart_card_pin_auth_operation_set_smart_card(
                self.raw,
                raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Wraps the corresponding `TKTokenSmartCardPINAuthOperation` operation.
    pub fn pin(&self) -> Result<Option<Zeroizing<String>>, CryptoTokenKitError> {
        read_secret(
            self.raw,
            ffi::token_session::ctk_token_smart_card_pin_auth_operation_pin,
        )
    }

    /// Sets the corresponding `TKTokenSmartCardPINAuthOperation` value.
    pub fn set_pin(&self, pin: Option<&str>) -> Result<(), CryptoTokenKitError> {
        write_secret(
            self.raw,
            ffi::token_session::ctk_token_smart_card_pin_auth_operation_set_pin,
            pin,
            "PINs",
        )
    }

    /// Invokes the corresponding `TKTokenSmartCardPINAuthOperation` operation.
    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_auth_operation_finish(self.raw, &raw mut error_ptr)
        };
        status_result(status, error_ptr)
    }
}

impl Default for TokenSmartCardPinAuthOperation {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_drop_release {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                if !self.raw.is_null() {
                    unsafe { ffi::ctk_object_release(self.raw) };
                    self.raw = ptr::null_mut();
                }
            }
        }
    };
}

impl_drop_release!(TokenSession);
impl_drop_release!(SmartCardTokenSession);
impl_drop_release!(TokenAuthOperation);
impl_drop_release!(TokenPasswordAuthOperation);
impl_drop_release!(TokenSmartCardPinAuthOperation);

#[cfg(test)]
mod tests {
    use super::{
        SmartCardTokenSession, TokenAuthOperation, TokenPasswordAuthOperation, TokenSession,
        TokenSmartCardPinAuthOperation,
    };
    use crate::ffi;
    use crate::private::checked;
    use crate::private::test_support::{assert_wrong_handle, retained};
    use crate::token::Token;
    use crate::token_driver::TokenDriver;
    use crate::token_keychain_contents::TokenObjectId;

    #[test]
    fn the_pin_never_enters_the_json_snapshot() {
        let operation = TokenSmartCardPinAuthOperation::new();
        operation.set_pin(Some("97531")).expect("set pin");
        operation
            .set_pin_byte_offset(1)
            .expect("snapshot update keeps working");
        let json = crate::error::take_owned_c_string(
            checked(|error| unsafe {
                ffi::token_session::ctk_token_smart_card_pin_auth_operation_json(
                    operation.raw,
                    error,
                )
            })
            .expect("snapshot"),
        );
        assert!(json.contains("pinByteOffset"), "{json}");
        assert!(!json.contains("97531"), "{json}");
        assert_eq!(
            operation
                .pin()
                .expect("pin")
                .as_ref()
                .map(|pin| pin.as_str()),
            Some("97531")
        );
    }

    #[test]
    fn auth_operation_handles_of_another_class_are_rejected() {
        let pin = TokenSmartCardPinAuthOperation::from_raw(TokenAuthOperation::new().into_raw());
        assert_wrong_handle(pin.pin());
        assert_wrong_handle(pin.set_pin(Some("1234")));
        assert_wrong_handle(pin.pin_format());
        assert_wrong_handle(pin.set_smart_card(None));

        let password =
            TokenPasswordAuthOperation::from_raw(TokenSmartCardPinAuthOperation::new().into_raw());
        assert_wrong_handle(password.password());
        assert_wrong_handle(password.set_password(Some("1234")));
    }

    #[test]
    fn session_handles_of_another_class_are_rejected() {
        let driver = TokenDriver::new();
        let token =
            Token::new(&driver, "com.example.cryptotokenkit.session-handles").expect("token");
        let plain = SmartCardTokenSession {
            raw: TokenSession::new(&token).expect("session").into_raw(),
        };
        assert_wrong_handle(plain.smart_card());
        assert_wrong_handle(plain.get_smart_card());
        assert_eq!(
            plain.token_instance_id().expect("still a token session"),
            "com.example.cryptotokenkit.session-handles"
        );

        let not_a_session = TokenSession::from_raw(retained(token.raw()));
        assert_wrong_handle(not_a_session.token_instance_id());
        assert_wrong_handle(not_a_session.token());
        assert_wrong_handle(not_a_session.has_delegate());
        assert_wrong_handle(not_a_session.clear_delegate());
        assert_wrong_handle(not_a_session.invoke_delegate_sign_data(
            b"payload",
            &TokenObjectId::new("key"),
            "com.example.base",
            &[],
        ));
    }
}
