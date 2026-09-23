use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::ptr;
use std::sync::{Mutex, MutexGuard, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;
use serde_json::Value;

use crate::error::{failure_status, CryptoTokenKitError, TKErrorCode};
use crate::ffi;
use crate::private::{
    decode_json, decode_optional_json, encode_json_cstring, json_to_ptr, status_result, to_cstring,
    write_error_ptr,
};
use crate::smart_card::SmartCard;
use crate::token::{SmartCardToken, Token, TokenConfigurationSnapshot};
use crate::token_driver::{SmartCardTokenDriver, TokenDriver};
use crate::token_keychain_contents::{TokenObjectId, TokenOperation};
use crate::token_session::{
    TokenAuthOperation, TokenPasswordAuthOperation, TokenSession, TokenSmartCardPinAuthOperation,
};

fn not_implemented(message: &str) -> CryptoTokenKitError {
    CryptoTokenKitError::Unknown {
        code: TKErrorCode::NotImplemented as i32,
        message: message.into(),
    }
}

/// Wraps `TKTokenKeyAlgorithm`.
pub struct TokenKeyAlgorithm {
    raw: *mut c_void,
}

impl TokenKeyAlgorithm {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    /// Wraps the corresponding `TKTokenKeyAlgorithm` operation.
    pub fn is_algorithm(&self, algorithm: &str) -> Result<bool, CryptoTokenKitError> {
        let algorithm = to_cstring(algorithm)?;
        Ok(unsafe {
            ffi::token_delegate::ctk_token_key_algorithm_is_algorithm(self.raw, algorithm.as_ptr())
        })
    }

    /// Wraps the corresponding `TKTokenKeyAlgorithm` operation.
    pub fn supports_algorithm(&self, algorithm: &str) -> Result<bool, CryptoTokenKitError> {
        let algorithm = to_cstring(algorithm)?;
        Ok(unsafe {
            ffi::token_delegate::ctk_token_key_algorithm_supports_algorithm(
                self.raw,
                algorithm.as_ptr(),
            )
        })
    }
}

impl Drop for TokenKeyAlgorithm {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: raw is either null (skipped) or a valid CryptoTokenKit object pointer.
            // It must be released via ctk_object_release exactly once per new() call.
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

/// Wraps `TKTokenKeyExchangeParameters`.
pub struct TokenKeyExchangeParameters {
    raw: *mut c_void,
}

impl TokenKeyExchangeParameters {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    #[must_use]
    /// Wraps the corresponding `TKTokenKeyExchangeParameters` operation.
    pub fn requested_size(&self) -> isize {
        unsafe { ffi::token_delegate::ctk_token_key_exchange_parameters_requested_size(self.raw) }
    }

    /// Wraps the corresponding `TKTokenKeyExchangeParameters` operation.
    pub fn shared_info(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        let ptr = unsafe {
            ffi::token_delegate::ctk_token_key_exchange_parameters_shared_info_json(self.raw)
        };
        decode_optional_json(ptr)
    }
}

impl Drop for TokenKeyExchangeParameters {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: raw is either null (skipped) or a valid CryptoTokenKit object pointer.
            // It must be released via ctk_object_release exactly once per new() call.
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

/// Enum wrapper over the `TKTokenAuthOperation` class cluster.
pub enum TokenAuthOperationHandle {
    /// Variant bridged from `TKTokenAuthOperation`.
    Base(TokenAuthOperation),
    /// Variant bridged from `TKTokenAuthOperation`.
    Password(TokenPasswordAuthOperation),
    /// Variant bridged from `TKTokenAuthOperation`.
    SmartCardPin(TokenSmartCardPinAuthOperation),
}

impl TokenAuthOperationHandle {
    #[must_use]
    fn from_raw(raw: *mut c_void) -> Self {
        match unsafe { ffi::token_delegate::ctk_token_auth_operation_kind(raw) } {
            1 => Self::Password(TokenPasswordAuthOperation::from_raw(raw)),
            2 => Self::SmartCardPin(TokenSmartCardPinAuthOperation::from_raw(raw)),
            _ => Self::Base(TokenAuthOperation::from_raw(raw)),
        }
    }

    fn into_raw(self) -> *mut c_void {
        match self {
            Self::Base(operation) => operation.into_raw(),
            Self::Password(operation) => operation.into_raw(),
            Self::SmartCardPin(operation) => operation.into_raw(),
        }
    }

    /// Invokes the corresponding `TKTokenAuthOperation` operation.
    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        match self {
            Self::Base(operation) => operation.finish(),
            Self::Password(operation) => operation.finish(),
            Self::SmartCardPin(operation) => operation.finish(),
        }
    }
}

impl From<TokenAuthOperation> for TokenAuthOperationHandle {
    fn from(value: TokenAuthOperation) -> Self {
        Self::Base(value)
    }
}

impl From<TokenPasswordAuthOperation> for TokenAuthOperationHandle {
    fn from(value: TokenPasswordAuthOperation) -> Self {
        Self::Password(value)
    }
}

impl From<TokenSmartCardPinAuthOperation> for TokenAuthOperationHandle {
    fn from(value: TokenSmartCardPinAuthOperation) -> Self {
        Self::SmartCardPin(value)
    }
}

/// Rust delegate bridge for `TKTokenSessionDelegate`.
pub trait TokenSessionDelegate: Send {
    /// Handles the corresponding `TKTokenSessionDelegate` callback.
    fn begin_auth_for_operation(
        &mut self,
        _session: &TokenSession,
        _operation: TokenOperation,
        _constraint: &Value,
    ) -> Result<Option<TokenAuthOperationHandle>, CryptoTokenKitError> {
        Ok(None)
    }

    /// Handles the corresponding `TKTokenSessionDelegate` callback.
    fn supports_operation(
        &mut self,
        _session: &TokenSession,
        _operation: TokenOperation,
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> bool {
        false
    }

    /// Handles the corresponding `TKTokenSessionDelegate` callback.
    fn sign_data(
        &mut self,
        _session: &TokenSession,
        _data: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        Err(not_implemented(
            "token session delegate sign_data not implemented",
        ))
    }

    /// Handles the corresponding `TKTokenSessionDelegate` callback.
    fn decrypt_data(
        &mut self,
        _session: &TokenSession,
        _ciphertext: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        Err(not_implemented(
            "token session delegate decrypt_data not implemented",
        ))
    }

    /// Handles the corresponding `TKTokenSessionDelegate` callback.
    fn perform_key_exchange(
        &mut self,
        _session: &TokenSession,
        _other_party_public_key_data: &[u8],
        _object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
        _parameters: &TokenKeyExchangeParameters,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        Err(not_implemented(
            "token session delegate perform_key_exchange not implemented",
        ))
    }
}

/// Rust delegate bridge for `TKTokenDelegate`.
pub trait TokenDelegate: Send {
    /// Handles the corresponding `TKTokenDelegate` callback.
    fn create_session(
        &mut self,
        _token: &Token,
    ) -> Result<Option<TokenSession>, CryptoTokenKitError> {
        Err(not_implemented(
            "token delegate create_session not implemented",
        ))
    }

    /// Handles the corresponding `TKTokenDelegate` callback.
    fn terminate_session(&mut self, _token: &Token, _session: &TokenSession) {}
}

/// Rust delegate bridge for `TKTokenDriverDelegate`.
pub trait TokenDriverDelegate: Send {
    /// Handles the corresponding `TKTokenDriverDelegate` callback.
    fn token_for_configuration(
        &mut self,
        _driver: &TokenDriver,
        _configuration: &TokenConfigurationSnapshot,
    ) -> Result<Option<Token>, CryptoTokenKitError> {
        Err(not_implemented(
            "token-driver delegate token_for_configuration not implemented",
        ))
    }

    /// Handles the corresponding `TKTokenDriverDelegate` callback.
    fn terminate_token(&mut self, _driver: &TokenDriver, _token: &Token) {}
}

/// Rust delegate bridge for `TKSmartCardTokenDriverDelegate`.
pub trait SmartCardTokenDriverDelegate: Send {
    /// Handles the corresponding `TKSmartCardTokenDriverDelegate` callback.
    fn create_token_for_smart_card(
        &mut self,
        _driver: &SmartCardTokenDriver,
        _smart_card: &SmartCard,
        _aid: Option<&[u8]>,
    ) -> Result<Option<SmartCardToken>, CryptoTokenKitError> {
        Err(not_implemented(
            "smart-card token-driver delegate create_token_for_smart_card not implemented",
        ))
    }

    /// Handles the corresponding `TKSmartCardTokenDriverDelegate` callback.
    fn terminate_token(&mut self, _driver: &SmartCardTokenDriver, _token: &SmartCardToken) {}
}

type SessionDelegateCell = Mutex<Box<dyn TokenSessionDelegate>>;
type TokenDelegateCell = Mutex<Box<dyn TokenDelegate>>;
type DriverDelegateCell = Mutex<Box<dyn TokenDriverDelegate>>;
type SmartCardDriverDelegateCell = Mutex<Box<dyn SmartCardTokenDriverDelegate>>;

/// Lifetime token for a bridged `TKTokenSessionDelegate` registration.
pub struct TokenSessionDelegateHandle {
    raw: *mut c_void,
    context: CallbackContext<SessionDelegateCell>,
}

/// Lifetime token for a bridged `TKTokenDelegate` registration.
pub struct TokenDelegateHandle {
    raw: *mut c_void,
    context: CallbackContext<TokenDelegateCell>,
}

/// Lifetime token for a bridged `TKTokenDriverDelegate` registration.
pub struct TokenDriverDelegateHandle {
    raw: *mut c_void,
    context: CallbackContext<DriverDelegateCell>,
}

/// Lifetime token for a bridged `TKSmartCardTokenDriverDelegate` registration.
pub struct SmartCardTokenDriverDelegateHandle {
    raw: *mut c_void,
    context: CallbackContext<SmartCardDriverDelegateCell>,
}

macro_rules! impl_delegate_handle_drop {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                self.context.deactivate();
                if !self.raw.is_null() {
                    // SAFETY: raw is either null (skipped) or a valid CryptoTokenKit delegate handle pointer.
                    // It must be released via ctk_object_release exactly once per creation.
                    unsafe { ffi::ctk_object_release(self.raw) };
                    self.raw = ptr::null_mut();
                }
            }
        }
    };
}

impl_delegate_handle_drop!(TokenSessionDelegateHandle);
impl_delegate_handle_drop!(TokenDelegateHandle);
impl_delegate_handle_drop!(TokenDriverDelegateHandle);
impl_delegate_handle_drop!(SmartCardTokenDriverDelegateHandle);

fn c_string_to_string(ptr: *const c_char, missing: &str) -> Result<String, CryptoTokenKitError> {
    if ptr.is_null() {
        return Err(CryptoTokenKitError::InvalidArgument(missing.into()));
    }
    // SAFETY: ptr is null-checked above and is expected to be a valid C string from the CryptoTokenKit framework.
    Ok(unsafe { CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned())
}

fn lock<T: ?Sized>(cell: &Mutex<Box<T>>) -> MutexGuard<'_, Box<T>> {
    cell.lock().unwrap_or_else(PoisonError::into_inner)
}

unsafe fn callback_bytes<'a>(ptr: *const u8, len: usize) -> Option<&'a [u8]> {
    if ptr.is_null() {
        return (len == 0).then_some(&[]);
    }
    Some(unsafe { std::slice::from_raw_parts(ptr, len) })
}

fn invalid_arguments(error_out: *mut *mut c_char, message: &str) -> i32 {
    write_error_ptr(error_out, message);
    ffi::status::INVALID_ARGUMENT
}

fn delegate_status(
    result: Option<Result<(), CryptoTokenKitError>>,
    error_out: *mut *mut c_char,
    callback: &str,
) -> i32 {
    match result {
        Some(Ok(())) => ffi::status::OK,
        Some(Err(error)) => {
            write_error_ptr(error_out, error.message());
            failure_status(&error)
        }
        None => {
            write_error_ptr(
                error_out,
                &format!("{callback} delegate callback failed: the Rust delegate was dropped or panicked"),
            );
            ffi::status::FRAMEWORK_ERROR
        }
    }
}

unsafe extern "C" fn token_session_begin_auth_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    operation_raw: i32,
    constraint_json: *const c_char,
    out_operation: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    let session = TokenSession::from_raw(session_raw);
    if session_raw.is_null() {
        return invalid_arguments(error_out, "missing token-session delegate callback context");
    }

    let invoke = |delegate: &SessionDelegateCell| -> Result<(), CryptoTokenKitError> {
        let constraint = if constraint_json.is_null() {
            Value::Null
        } else {
            serde_json::from_str::<Value>(&c_string_to_string(
                constraint_json,
                "invalid constraint JSON",
            )?)
            .map_err(|error| {
                CryptoTokenKitError::InvalidArgument(format!(
                    "invalid token-session constraint JSON: {error}"
                ))
            })?
        };
        let operation = TokenOperation::from_raw(operation_raw);
        let operation_handle =
            lock(delegate).begin_auth_for_operation(&session, operation, &constraint)?;
        if !out_operation.is_null() {
            // SAFETY: out_operation is not null as checked, and writing a pointer value is safe.
            unsafe {
                *out_operation =
                    operation_handle.map_or(ptr::null_mut(), TokenAuthOperationHandle::into_raw);
            }
        }
        Ok(())
    };
    let result = unsafe {
        CallbackContext::<SessionDelegateCell>::with(
            user_info,
            "TokenSessionDelegate::begin_auth_for_operation",
            invoke,
        )
    };
    delegate_status(result, error_out, "token-session begin-auth")
}

unsafe extern "C" fn token_session_supports_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    operation_raw: i32,
    object_id_ptr: *const c_char,
    algorithm_raw: *mut c_void,
) -> bool {
    let session = TokenSession::from_raw(session_raw);
    let algorithm = TokenKeyAlgorithm::from_raw(algorithm_raw);
    if session_raw.is_null() || object_id_ptr.is_null() || algorithm_raw.is_null() {
        return false;
    }

    unsafe {
        CallbackContext::<SessionDelegateCell>::with(
            user_info,
            "TokenSessionDelegate::supports_operation",
            |delegate| {
                let object_id = TokenObjectId(
                    c_string_to_string(object_id_ptr, "missing token object identifier")
                        .unwrap_or_default(),
                );
                lock(delegate).supports_operation(
                    &session,
                    TokenOperation::from_raw(operation_raw),
                    &object_id,
                    &algorithm,
                )
            },
        )
    }
    .unwrap_or(false)
}

unsafe extern "C" fn token_session_data_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    data_ptr: *const u8,
    data_len: usize,
    object_id_ptr: *const c_char,
    algorithm_raw: *mut c_void,
    out_reply_json: *mut *mut c_char,
    error_out: *mut *mut c_char,
    mode: i32,
) -> i32 {
    let session = TokenSession::from_raw(session_raw);
    let algorithm = TokenKeyAlgorithm::from_raw(algorithm_raw);
    let data = unsafe { callback_bytes(data_ptr, data_len) };
    let (Some(data), false) = (
        data,
        session_raw.is_null() || object_id_ptr.is_null() || algorithm_raw.is_null(),
    ) else {
        return invalid_arguments(
            error_out,
            "missing token-session delegate callback arguments",
        );
    };

    let (site, callback) = if mode == 0 {
        ("TokenSessionDelegate::sign_data", "token-session sign")
    } else {
        (
            "TokenSessionDelegate::decrypt_data",
            "token-session decrypt",
        )
    };
    let invoke = |delegate: &SessionDelegateCell| -> Result<(), CryptoTokenKitError> {
        let object_id = TokenObjectId(c_string_to_string(
            object_id_ptr,
            "missing token object identifier",
        )?);
        let reply = if mode == 0 {
            lock(delegate).sign_data(&session, data, &object_id, &algorithm)?
        } else {
            lock(delegate).decrypt_data(&session, data, &object_id, &algorithm)?
        };
        if !out_reply_json.is_null() {
            unsafe {
                *out_reply_json = json_to_ptr(&reply)?;
            }
        }
        Ok(())
    };
    let result = unsafe { CallbackContext::<SessionDelegateCell>::with(user_info, site, invoke) };
    delegate_status(result, error_out, callback)
}

unsafe extern "C" fn token_session_sign_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    data_ptr: *const u8,
    data_len: usize,
    object_id_ptr: *const c_char,
    algorithm_raw: *mut c_void,
    out_reply_json: *mut *mut c_char,
    error_out: *mut *mut c_char,
) -> i32 {
    unsafe {
        token_session_data_trampoline(
            user_info,
            session_raw,
            data_ptr,
            data_len,
            object_id_ptr,
            algorithm_raw,
            out_reply_json,
            error_out,
            0,
        )
    }
}

unsafe extern "C" fn token_session_decrypt_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    data_ptr: *const u8,
    data_len: usize,
    object_id_ptr: *const c_char,
    algorithm_raw: *mut c_void,
    out_reply_json: *mut *mut c_char,
    error_out: *mut *mut c_char,
) -> i32 {
    unsafe {
        token_session_data_trampoline(
            user_info,
            session_raw,
            data_ptr,
            data_len,
            object_id_ptr,
            algorithm_raw,
            out_reply_json,
            error_out,
            1,
        )
    }
}

unsafe extern "C" fn token_session_key_exchange_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    public_key_ptr: *const u8,
    public_key_len: usize,
    object_id_ptr: *const c_char,
    algorithm_raw: *mut c_void,
    parameters_raw: *mut c_void,
    out_reply_json: *mut *mut c_char,
    error_out: *mut *mut c_char,
) -> i32 {
    let session = TokenSession::from_raw(session_raw);
    let algorithm = TokenKeyAlgorithm::from_raw(algorithm_raw);
    let parameters = TokenKeyExchangeParameters::from_raw(parameters_raw);
    let public_key = unsafe { callback_bytes(public_key_ptr, public_key_len) };
    let (Some(public_key), false) = (
        public_key,
        session_raw.is_null()
            || object_id_ptr.is_null()
            || algorithm_raw.is_null()
            || parameters_raw.is_null(),
    ) else {
        return invalid_arguments(
            error_out,
            "missing token-session key-exchange callback arguments",
        );
    };

    let invoke = |delegate: &SessionDelegateCell| -> Result<(), CryptoTokenKitError> {
        let object_id = TokenObjectId(c_string_to_string(
            object_id_ptr,
            "missing token object identifier",
        )?);
        let reply = lock(delegate).perform_key_exchange(
            &session,
            public_key,
            &object_id,
            &algorithm,
            &parameters,
        )?;
        if !out_reply_json.is_null() {
            unsafe {
                *out_reply_json = json_to_ptr(&reply)?;
            }
        }
        Ok(())
    };
    let result = unsafe {
        CallbackContext::<SessionDelegateCell>::with(
            user_info,
            "TokenSessionDelegate::perform_key_exchange",
            invoke,
        )
    };
    delegate_status(result, error_out, "token-session key-exchange")
}

unsafe extern "C" fn token_create_session_trampoline(
    user_info: *mut c_void,
    token_raw: *mut c_void,
    out_session: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    let token = Token::from_raw(token_raw);
    if token_raw.is_null() {
        return invalid_arguments(error_out, "missing token delegate callback context");
    }

    let invoke = |delegate: &TokenDelegateCell| -> Result<(), CryptoTokenKitError> {
        let session = lock(delegate).create_session(&token)?;
        if !out_session.is_null() {
            unsafe {
                *out_session = session.map_or(ptr::null_mut(), TokenSession::into_raw);
            }
        }
        Ok(())
    };
    let result = unsafe {
        CallbackContext::<TokenDelegateCell>::with(
            user_info,
            "TokenDelegate::create_session",
            invoke,
        )
    };
    delegate_status(result, error_out, "token create-session")
}

unsafe extern "C" fn token_terminate_session_trampoline(
    user_info: *mut c_void,
    token_raw: *mut c_void,
    session_raw: *mut c_void,
) {
    let token = Token::from_raw(token_raw);
    let session = TokenSession::from_raw(session_raw);
    if token_raw.is_null() || session_raw.is_null() {
        return;
    }

    unsafe {
        CallbackContext::<TokenDelegateCell>::with(
            user_info,
            "TokenDelegate::terminate_session",
            |delegate| lock(delegate).terminate_session(&token, &session),
        )
    };
}

unsafe extern "C" fn token_driver_create_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    configuration_json: *const c_char,
    out_token: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    let driver = TokenDriver::from_raw(driver_raw);
    if driver_raw.is_null() || configuration_json.is_null() {
        return invalid_arguments(
            error_out,
            "missing token-driver delegate callback arguments",
        );
    }

    let invoke = |delegate: &DriverDelegateCell| -> Result<(), CryptoTokenKitError> {
        let configuration: TokenConfigurationSnapshot = serde_json::from_str(&c_string_to_string(
            configuration_json,
            "missing token configuration JSON",
        )?)
        .map_err(|error| {
            CryptoTokenKitError::InvalidArgument(format!(
                "invalid token-driver configuration JSON: {error}"
            ))
        })?;
        let token = lock(delegate).token_for_configuration(&driver, &configuration)?;
        if !out_token.is_null() {
            unsafe {
                *out_token = token.map_or(ptr::null_mut(), Token::into_raw);
            }
        }
        Ok(())
    };
    let result = unsafe {
        CallbackContext::<DriverDelegateCell>::with(
            user_info,
            "TokenDriverDelegate::token_for_configuration",
            invoke,
        )
    };
    delegate_status(result, error_out, "token-driver create-token")
}

unsafe extern "C" fn token_driver_terminate_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    token_raw: *mut c_void,
) {
    let driver = TokenDriver::from_raw(driver_raw);
    let token = Token::from_raw(token_raw);
    if driver_raw.is_null() || token_raw.is_null() {
        return;
    }

    unsafe {
        CallbackContext::<DriverDelegateCell>::with(
            user_info,
            "TokenDriverDelegate::terminate_token",
            |delegate| lock(delegate).terminate_token(&driver, &token),
        )
    };
}

unsafe extern "C" fn smart_card_token_driver_create_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    smart_card_raw: *mut c_void,
    aid_ptr: *const u8,
    aid_len: usize,
    has_aid: bool,
    out_token: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    let driver = SmartCardTokenDriver::from_raw(driver_raw);
    let smart_card = SmartCard::from_raw(smart_card_raw);
    let aid = if has_aid {
        unsafe { callback_bytes(aid_ptr, aid_len) }.map(Some)
    } else {
        Some(None)
    };
    let (Some(aid), false) = (aid, driver_raw.is_null() || smart_card_raw.is_null()) else {
        return invalid_arguments(
            error_out,
            "missing smart-card token-driver delegate callback arguments",
        );
    };

    let invoke = |delegate: &SmartCardDriverDelegateCell| -> Result<(), CryptoTokenKitError> {
        let token = lock(delegate).create_token_for_smart_card(&driver, &smart_card, aid)?;
        if !out_token.is_null() {
            unsafe {
                *out_token = token.map_or(ptr::null_mut(), SmartCardToken::into_raw);
            }
        }
        Ok(())
    };
    let result = unsafe {
        CallbackContext::<SmartCardDriverDelegateCell>::with(
            user_info,
            "SmartCardTokenDriverDelegate::create_token_for_smart_card",
            invoke,
        )
    };
    delegate_status(result, error_out, "smart-card token-driver create-token")
}

unsafe extern "C" fn smart_card_token_driver_terminate_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    token_raw: *mut c_void,
) {
    let driver = SmartCardTokenDriver::from_raw(driver_raw);
    let token = SmartCardToken::from_raw(token_raw);
    if driver_raw.is_null() || token_raw.is_null() {
        return;
    }

    unsafe {
        CallbackContext::<SmartCardDriverDelegateCell>::with(
            user_info,
            "SmartCardTokenDriverDelegate::terminate_token",
            |delegate| lock(delegate).terminate_token(&driver, &token),
        )
    };
}

impl TokenSession {
    /// Returns the corresponding `TKTokenSession` value.
    pub fn token(&self) -> Result<Token, CryptoTokenKitError> {
        let raw = unsafe { ffi::token_delegate::ctk_token_session_token(self.raw()) };
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token-session token".into(),
            ));
        }
        Ok(Token::from_raw(raw))
    }

    /// Sets the corresponding `TKTokenSession` value.
    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<TokenSessionDelegateHandle, CryptoTokenKitError>
    where
        D: TokenSessionDelegate + 'static,
    {
        let cell: SessionDelegateCell = Mutex::new(Box::new(delegate));
        let context = CallbackContext::new(cell);
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_session_set_delegate(
                self.raw(),
                Some(token_session_begin_auth_trampoline),
                Some(token_session_supports_trampoline),
                Some(token_session_sign_trampoline),
                Some(token_session_decrypt_trampoline),
                Some(token_session_key_exchange_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<SessionDelegateCell>::RELEASE),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token-session delegate handle".into(),
            ));
        }
        Ok(TokenSessionDelegateHandle { raw, context })
    }

    #[must_use]
    /// Returns whether `TKTokenSession` currently has the associated bridge state.
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_session_has_delegate(self.raw()) }
    }

    /// Clears the corresponding `TKTokenSession` bridge state.
    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_session_clear_delegate(self.raw()) };
    }

    /// Invokes the bridged `TKTokenSession` delegate callback.
    pub fn invoke_delegate_begin_auth(
        &self,
        operation: TokenOperation,
        constraint: &Value,
    ) -> Result<Option<TokenAuthOperationHandle>, CryptoTokenKitError> {
        let constraint = encode_json_cstring(constraint)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_session_invoke_delegate_begin_auth(
                self.raw(),
                operation.raw(),
                constraint.as_ptr(),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| TokenAuthOperationHandle::from_raw(raw)))
    }

    /// Invokes the bridged `TKTokenSession` delegate callback.
    pub fn invoke_delegate_supports_operation(
        &self,
        operation: TokenOperation,
        key_object_id: &TokenObjectId,
        base_algorithm: &str,
        supported_algorithms: &[&str],
    ) -> Result<bool, CryptoTokenKitError> {
        let object_id = to_cstring(&key_object_id.0)?;
        let base_algorithm = to_cstring(base_algorithm)?;
        let supported_algorithms = encode_json_cstring(supported_algorithms)?;
        Ok(unsafe {
            ffi::token_delegate::ctk_token_session_invoke_delegate_supports(
                self.raw(),
                operation.raw(),
                object_id.as_ptr(),
                base_algorithm.as_ptr(),
                supported_algorithms.as_ptr(),
            )
        })
    }

    fn invoke_delegate_data_operation(
        &self,
        operation: TokenOperation,
        request: &[u8],
        key_object_id: &TokenObjectId,
        base_algorithm: &str,
        supported_algorithms: &[&str],
        callback: unsafe extern "C" fn(
            *mut c_void,
            i32,
            *const u8,
            usize,
            *const c_char,
            *const c_char,
            *const c_char,
            *mut *mut c_char,
            *mut *mut c_char,
        ) -> i32,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        let object_id = to_cstring(&key_object_id.0)?;
        let base_algorithm = to_cstring(base_algorithm)?;
        let supported_algorithms = encode_json_cstring(supported_algorithms)?;
        let mut reply_ptr = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            callback(
                self.raw(),
                operation.raw(),
                request.as_ptr(),
                request.len(),
                object_id.as_ptr(),
                base_algorithm.as_ptr(),
                supported_algorithms.as_ptr(),
                &raw mut reply_ptr,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if reply_ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(reply_ptr)
    }

    /// Invokes the bridged `TKTokenSession` delegate callback.
    pub fn invoke_delegate_sign_data(
        &self,
        data: &[u8],
        key_object_id: &TokenObjectId,
        base_algorithm: &str,
        supported_algorithms: &[&str],
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        self.invoke_delegate_data_operation(
            TokenOperation::SignData,
            data,
            key_object_id,
            base_algorithm,
            supported_algorithms,
            ffi::token_delegate::ctk_token_session_invoke_delegate_sign,
        )
    }

    /// Invokes the bridged `TKTokenSession` delegate callback.
    pub fn invoke_delegate_decrypt_data(
        &self,
        ciphertext: &[u8],
        key_object_id: &TokenObjectId,
        base_algorithm: &str,
        supported_algorithms: &[&str],
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        self.invoke_delegate_data_operation(
            TokenOperation::DecryptData,
            ciphertext,
            key_object_id,
            base_algorithm,
            supported_algorithms,
            ffi::token_delegate::ctk_token_session_invoke_delegate_decrypt,
        )
    }

    /// Invokes the bridged `TKTokenSession` delegate callback.
    pub fn invoke_delegate_perform_key_exchange(
        &self,
        other_party_public_key_data: &[u8],
        object_id: &TokenObjectId,
        base_algorithm: &str,
        supported_algorithms: &[&str],
        requested_size: isize,
        shared_info: Option<&[u8]>,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        let object_id = to_cstring(&object_id.0)?;
        let base_algorithm = to_cstring(base_algorithm)?;
        let supported_algorithms = encode_json_cstring(supported_algorithms)?;
        let mut reply_ptr = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let (shared_info_ptr, shared_info_len, has_shared_info) = shared_info
            .map_or((ptr::null(), 0, false), |bytes| {
                (bytes.as_ptr(), bytes.len(), true)
            });
        let status = unsafe {
            ffi::token_delegate::ctk_token_session_invoke_delegate_key_exchange(
                self.raw(),
                other_party_public_key_data.as_ptr(),
                other_party_public_key_data.len(),
                object_id.as_ptr(),
                base_algorithm.as_ptr(),
                supported_algorithms.as_ptr(),
                requested_size,
                shared_info_ptr,
                shared_info_len,
                has_shared_info,
                &raw mut reply_ptr,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if reply_ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(reply_ptr)
    }
}

impl Token {
    /// Returns the corresponding `TKToken` value.
    pub fn token_driver(&self) -> Result<TokenDriver, CryptoTokenKitError> {
        let raw = unsafe { ffi::token_delegate::ctk_token_token_driver(self.raw()) };
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token driver".into(),
            ));
        }
        Ok(TokenDriver::from_raw(raw))
    }

    /// Sets the corresponding `TKToken` value.
    pub fn set_delegate<D>(&self, delegate: D) -> Result<TokenDelegateHandle, CryptoTokenKitError>
    where
        D: TokenDelegate + 'static,
    {
        let cell: TokenDelegateCell = Mutex::new(Box::new(delegate));
        let context = CallbackContext::new(cell);
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_set_delegate(
                self.raw(),
                Some(token_create_session_trampoline),
                Some(token_terminate_session_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<TokenDelegateCell>::RELEASE),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token delegate handle".into(),
            ));
        }
        Ok(TokenDelegateHandle { raw, context })
    }

    #[must_use]
    /// Returns whether `TKToken` currently has the associated bridge state.
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_has_delegate(self.raw()) }
    }

    /// Clears the corresponding `TKToken` bridge state.
    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_clear_delegate(self.raw()) };
    }

    /// Invokes the bridged `TKToken` delegate callback.
    pub fn invoke_delegate_create_session(
        &self,
    ) -> Result<Option<TokenSession>, CryptoTokenKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_invoke_delegate_create_session(
                self.raw(),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| TokenSession::from_raw(raw)))
    }

    /// Invokes the bridged `TKToken` delegate callback.
    pub fn invoke_delegate_terminate_session(&self, session: &TokenSession) {
        unsafe {
            ffi::token_delegate::ctk_token_invoke_delegate_terminate_session(
                self.raw(),
                session.raw(),
            );
        };
    }
}

impl TokenDriver {
    /// Registers the corresponding `TKTokenDriver` callback.
    pub fn add_token_configuration(
        class_id: &str,
        instance_id: &str,
    ) -> Result<TokenConfigurationSnapshot, CryptoTokenKitError> {
        let class_id = to_cstring(class_id)?;
        let instance_id = to_cstring(instance_id)?;
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token_delegate::ctk_token_driver_add_token_configuration_json(
                class_id.as_ptr(),
                instance_id.as_ptr(),
                &raw mut error_ptr,
            )
        };
        if ptr.is_null() {
            return Err(crate::error::from_swift(
                ffi::status::FRAMEWORK_ERROR,
                error_ptr,
            ));
        }
        decode_json(ptr)
    }

    /// Wraps the corresponding `TKTokenDriver` operation.
    pub fn remove_token_configuration(
        class_id: &str,
        instance_id: &str,
    ) -> Result<(), CryptoTokenKitError> {
        let class_id = to_cstring(class_id)?;
        let instance_id = to_cstring(instance_id)?;
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_driver_remove_token_configuration(
                class_id.as_ptr(),
                instance_id.as_ptr(),
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Sets the corresponding `TKTokenDriver` value.
    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<TokenDriverDelegateHandle, CryptoTokenKitError>
    where
        D: TokenDriverDelegate + 'static,
    {
        let cell: DriverDelegateCell = Mutex::new(Box::new(delegate));
        let context = CallbackContext::new(cell);
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_driver_set_delegate(
                self.raw(),
                Some(token_driver_create_token_trampoline),
                Some(token_driver_terminate_token_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<DriverDelegateCell>::RELEASE),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token-driver delegate handle".into(),
            ));
        }
        Ok(TokenDriverDelegateHandle { raw, context })
    }

    #[must_use]
    /// Returns whether `TKTokenDriver` currently has the associated bridge state.
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_driver_has_delegate(self.raw()) }
    }

    /// Clears the corresponding `TKTokenDriver` bridge state.
    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_driver_clear_delegate(self.raw()) };
    }

    /// Invokes the bridged `TKTokenDriver` delegate callback.
    pub fn invoke_delegate_token_for_configuration(
        &self,
        configuration: &TokenConfigurationSnapshot,
    ) -> Result<Option<Token>, CryptoTokenKitError> {
        let configuration = encode_json_cstring(configuration)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_driver_invoke_delegate_token_for_configuration_json(
                self.raw(),
                configuration.as_ptr(),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| Token::from_raw(raw)))
    }

    /// Invokes the bridged `TKTokenDriver` delegate callback.
    pub fn invoke_delegate_terminate_token(&self, token: &Token) {
        unsafe {
            ffi::token_delegate::ctk_token_driver_invoke_delegate_terminate_token(
                self.raw(),
                token.raw(),
            );
        };
    }
}

impl SmartCardTokenDriver {
    /// Sets the corresponding `TKSmartCardTokenDriver` value.
    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<SmartCardTokenDriverDelegateHandle, CryptoTokenKitError>
    where
        D: SmartCardTokenDriverDelegate + 'static,
    {
        let cell: SmartCardDriverDelegateCell = Mutex::new(Box::new(delegate));
        let context = CallbackContext::new(cell);
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_smart_card_token_driver_set_delegate(
                self.raw(),
                Some(smart_card_token_driver_create_token_trampoline),
                Some(smart_card_token_driver_terminate_token_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<SmartCardDriverDelegateCell>::RELEASE),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card token-driver delegate handle".into(),
            ));
        }
        Ok(SmartCardTokenDriverDelegateHandle { raw, context })
    }

    #[must_use]
    /// Returns whether `TKSmartCardTokenDriver` currently has the associated bridge state.
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_driver_has_delegate(self.raw()) }
    }

    /// Clears the corresponding `TKSmartCardTokenDriver` bridge state.
    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_driver_clear_delegate(self.raw()) };
    }

    /// Invokes the bridged `TKSmartCardTokenDriver` delegate callback.
    pub fn invoke_delegate_create_token(
        &self,
        smart_card: &SmartCard,
        aid: Option<&[u8]>,
    ) -> Result<Option<SmartCardToken>, CryptoTokenKitError> {
        let (aid_ptr, aid_len, has_aid) = aid.map_or((ptr::null(), 0, false), |bytes| {
            (bytes.as_ptr(), bytes.len(), true)
        });
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_smart_card_token_driver_invoke_delegate_create_token(
                self.raw(),
                smart_card.raw(),
                aid_ptr,
                aid_len,
                has_aid,
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| SmartCardToken::from_raw(raw)))
    }

    /// Invokes the bridged `TKSmartCardTokenDriver` delegate callback.
    pub fn invoke_delegate_terminate_token(&self, token: &SmartCardToken) {
        unsafe {
            ffi::token_delegate::ctk_smart_card_token_driver_invoke_delegate_terminate_token(
                self.raw(),
                token.raw(),
            );
        };
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    use super::{TokenKeyAlgorithm, TokenSessionDelegate};
    use crate::error::CryptoTokenKitError;
    use crate::token::Token;
    use crate::token_driver::TokenDriver;
    use crate::token_keychain_contents::TokenObjectId;
    use crate::token_session::TokenSession;

    unsafe extern "C" {
        fn objc_retain(object: *mut c_void) -> *mut c_void;
        fn objc_release(object: *mut c_void);
    }

    struct CountingDelegate {
        calls: Arc<AtomicUsize>,
        dropped: Arc<AtomicBool>,
    }

    impl Drop for CountingDelegate {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    impl TokenSessionDelegate for CountingDelegate {
        fn sign_data(
            &mut self,
            _session: &TokenSession,
            data: &[u8],
            _key_object_id: &TokenObjectId,
            _algorithm: &TokenKeyAlgorithm,
        ) -> Result<Vec<u8>, CryptoTokenKitError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(data.to_vec())
        }
    }

    fn sign(session: &TokenSession, data: &[u8]) -> Result<Vec<u8>, CryptoTokenKitError> {
        session.invoke_delegate_sign_data(data, &TokenObjectId::new("key"), "com.example.base", &[])
    }

    #[test]
    fn callbacks_after_the_handle_is_dropped_never_reach_the_delegate() {
        let driver = TokenDriver::new();
        let token = Token::new(&driver, "com.example.cryptotokenkit.late-callback").expect("token");
        let session = TokenSession::new(&token);
        let calls = Arc::new(AtomicUsize::new(0));
        let dropped = Arc::new(AtomicBool::new(false));
        let handle = session
            .set_delegate(CountingDelegate {
                calls: Arc::clone(&calls),
                dropped: Arc::clone(&dropped),
            })
            .expect("delegate");

        assert_eq!(sign(&session, b"live").expect("live callback"), b"live");
        assert_eq!(sign(&session, b"").expect("empty data callback"), b"");
        assert_eq!(calls.load(Ordering::SeqCst), 2);

        let delegate_box = handle.raw;
        unsafe { objc_retain(delegate_box) };
        drop(handle);

        assert!(session.has_delegate());
        assert!(!dropped.load(Ordering::SeqCst));
        let error = sign(&session, b"late").expect_err("late callback must fail");
        assert!(error.message().contains("dropped"), "{error:?}");
        assert_eq!(calls.load(Ordering::SeqCst), 2);

        unsafe { objc_release(delegate_box) };
        assert!(dropped.load(Ordering::SeqCst));
        assert!(!session.has_delegate());
    }
}
