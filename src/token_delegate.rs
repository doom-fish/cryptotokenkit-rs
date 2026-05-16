use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::Mutex;

use serde_json::Value;

use crate::error::{CryptoTokenKitError, TKErrorCode};
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

pub struct TokenKeyAlgorithm {
    raw: *mut c_void,
}

impl TokenKeyAlgorithm {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    pub fn is_algorithm(&self, algorithm: &str) -> Result<bool, CryptoTokenKitError> {
        let algorithm = to_cstring(algorithm)?;
        Ok(unsafe {
            ffi::token_delegate::ctk_token_key_algorithm_is_algorithm(self.raw, algorithm.as_ptr())
        })
    }

    pub fn supports_algorithm(&self, algorithm: &str) -> Result<bool, CryptoTokenKitError> {
        let algorithm = to_cstring(algorithm)?;
        Ok(unsafe {
            ffi::token_delegate::ctk_token_key_algorithm_supports_algorithm(self.raw, algorithm.as_ptr())
        })
    }
}

impl Drop for TokenKeyAlgorithm {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

pub struct TokenKeyExchangeParameters {
    raw: *mut c_void,
}

impl TokenKeyExchangeParameters {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    #[must_use]
    pub fn requested_size(&self) -> isize {
        unsafe { ffi::token_delegate::ctk_token_key_exchange_parameters_requested_size(self.raw) }
    }

    pub fn shared_info(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        let ptr = unsafe { ffi::token_delegate::ctk_token_key_exchange_parameters_shared_info_json(self.raw) };
        decode_optional_json(ptr)
    }
}

impl Drop for TokenKeyExchangeParameters {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

pub enum TokenAuthOperationHandle {
    Base(TokenAuthOperation),
    Password(TokenPasswordAuthOperation),
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

pub trait TokenSessionDelegate: Send {
    fn begin_auth_for_operation(
        &mut self,
        _session: &TokenSession,
        _operation: TokenOperation,
        _constraint: &Value,
    ) -> Result<Option<TokenAuthOperationHandle>, CryptoTokenKitError> {
        Ok(None)
    }

    fn supports_operation(
        &mut self,
        _session: &TokenSession,
        _operation: TokenOperation,
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> bool {
        false
    }

    fn sign_data(
        &mut self,
        _session: &TokenSession,
        _data: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        Err(not_implemented("token session delegate sign_data not implemented"))
    }

    fn decrypt_data(
        &mut self,
        _session: &TokenSession,
        _ciphertext: &[u8],
        _key_object_id: &TokenObjectId,
        _algorithm: &TokenKeyAlgorithm,
    ) -> Result<Vec<u8>, CryptoTokenKitError> {
        Err(not_implemented("token session delegate decrypt_data not implemented"))
    }

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

pub trait TokenDelegate: Send {
    fn create_session(&mut self, _token: &Token) -> Result<Option<TokenSession>, CryptoTokenKitError> {
        Err(not_implemented("token delegate create_session not implemented"))
    }

    fn terminate_session(&mut self, _token: &Token, _session: &TokenSession) {}
}

pub trait TokenDriverDelegate: Send {
    fn token_for_configuration(
        &mut self,
        _driver: &TokenDriver,
        _configuration: &TokenConfigurationSnapshot,
    ) -> Result<Option<Token>, CryptoTokenKitError> {
        Err(not_implemented(
            "token-driver delegate token_for_configuration not implemented",
        ))
    }

    fn terminate_token(&mut self, _driver: &TokenDriver, _token: &Token) {}
}

pub trait SmartCardTokenDriverDelegate: Send {
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

    fn terminate_token(&mut self, _driver: &SmartCardTokenDriver, _token: &SmartCardToken) {}
}

struct TokenSessionDelegateState {
    delegate: Mutex<Box<dyn TokenSessionDelegate>>,
}

struct TokenDelegateState {
    delegate: Mutex<Box<dyn TokenDelegate>>,
}

struct TokenDriverDelegateState {
    delegate: Mutex<Box<dyn TokenDriverDelegate>>,
}

struct SmartCardTokenDriverDelegateState {
    delegate: Mutex<Box<dyn SmartCardTokenDriverDelegate>>,
}

pub struct TokenSessionDelegateHandle {
    raw: *mut c_void,
    _state: Box<TokenSessionDelegateState>,
}

pub struct TokenDelegateHandle {
    raw: *mut c_void,
    _state: Box<TokenDelegateState>,
}

pub struct TokenDriverDelegateHandle {
    raw: *mut c_void,
    _state: Box<TokenDriverDelegateState>,
}

pub struct SmartCardTokenDriverDelegateHandle {
    raw: *mut c_void,
    _state: Box<SmartCardTokenDriverDelegateState>,
}

macro_rules! impl_delegate_handle_drop {
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

impl_delegate_handle_drop!(TokenSessionDelegateHandle);
impl_delegate_handle_drop!(TokenDelegateHandle);
impl_delegate_handle_drop!(TokenDriverDelegateHandle);
impl_delegate_handle_drop!(SmartCardTokenDriverDelegateHandle);

fn c_string_to_string(ptr: *const c_char, missing: &str) -> Result<String, CryptoTokenKitError> {
    if ptr.is_null() {
        return Err(CryptoTokenKitError::InvalidArgument(missing.into()));
    }
    Ok(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned())
}

unsafe extern "C" fn token_session_begin_auth_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    operation_raw: i32,
    constraint_json: *const c_char,
    out_operation: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    if user_info.is_null() || session_raw.is_null() {
        write_error_ptr(error_out, "missing token-session delegate callback context");
        return ffi::status::INVALID_ARGUMENT;
    }

    match catch_unwind(AssertUnwindSafe(|| -> Result<i32, CryptoTokenKitError> {
        let state = unsafe { &*user_info.cast::<TokenSessionDelegateState>() };
        let session = TokenSession::from_raw(session_raw);
        let constraint = if constraint_json.is_null() {
            Value::Null
        } else {
            serde_json::from_str::<Value>(&c_string_to_string(constraint_json, "invalid constraint JSON")?)
                .map_err(|error| CryptoTokenKitError::InvalidArgument(format!(
                    "invalid token-session constraint JSON: {error}"
                )))?
        };
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let operation = TokenOperation::from_raw(operation_raw);
        let operation_handle = delegate.begin_auth_for_operation(&session, operation, &constraint)?;
        drop(delegate);
        if !out_operation.is_null() {
            unsafe {
                *out_operation = operation_handle.map_or(ptr::null_mut(), TokenAuthOperationHandle::into_raw);
            }
        }
        Ok(ffi::status::OK)
    })) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            write_error_ptr(error_out, error.message());
            error.code()
        }
        Err(_) => {
            write_error_ptr(error_out, "panic in token-session begin-auth delegate callback");
            ffi::status::FRAMEWORK_ERROR
        }
    }
}

unsafe extern "C" fn token_session_supports_trampoline(
    user_info: *mut c_void,
    session_raw: *mut c_void,
    operation_raw: i32,
    object_id_ptr: *const c_char,
    algorithm_raw: *mut c_void,
) -> bool {
    if user_info.is_null() || session_raw.is_null() || object_id_ptr.is_null() || algorithm_raw.is_null() {
        return false;
    }

    catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<TokenSessionDelegateState>() };
        let session = TokenSession::from_raw(session_raw);
        let object_id = TokenObjectId(c_string_to_string(object_id_ptr, "missing token object identifier")
            .unwrap_or_default());
        let algorithm = TokenKeyAlgorithm::from_raw(algorithm_raw);
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        delegate.supports_operation(
            &session,
            TokenOperation::from_raw(operation_raw),
            &object_id,
            &algorithm,
        )
    }))
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
    if user_info.is_null()
        || session_raw.is_null()
        || data_ptr.is_null()
        || object_id_ptr.is_null()
        || algorithm_raw.is_null()
    {
        write_error_ptr(error_out, "missing token-session delegate callback arguments");
        return ffi::status::INVALID_ARGUMENT;
    }

    match catch_unwind(AssertUnwindSafe(|| -> Result<i32, CryptoTokenKitError> {
        let state = unsafe { &*user_info.cast::<TokenSessionDelegateState>() };
        let session = TokenSession::from_raw(session_raw);
        let object_id = TokenObjectId(c_string_to_string(object_id_ptr, "missing token object identifier")?);
        let algorithm = TokenKeyAlgorithm::from_raw(algorithm_raw);
        let data = unsafe { std::slice::from_raw_parts(data_ptr, data_len) };
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let reply = match mode {
            0 => delegate.sign_data(&session, data, &object_id, &algorithm)?,
            _ => delegate.decrypt_data(&session, data, &object_id, &algorithm)?,
        };
        drop(delegate);
        if !out_reply_json.is_null() {
            unsafe {
                *out_reply_json = json_to_ptr(&reply)?;
            }
        }
        Ok(ffi::status::OK)
    })) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            write_error_ptr(error_out, error.message());
            error.code()
        }
        Err(_) => {
            write_error_ptr(error_out, "panic in token-session data delegate callback");
            ffi::status::FRAMEWORK_ERROR
        }
    }
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
    if user_info.is_null()
        || session_raw.is_null()
        || public_key_ptr.is_null()
        || object_id_ptr.is_null()
        || algorithm_raw.is_null()
        || parameters_raw.is_null()
    {
        write_error_ptr(error_out, "missing token-session key-exchange callback arguments");
        return ffi::status::INVALID_ARGUMENT;
    }

    match catch_unwind(AssertUnwindSafe(|| -> Result<i32, CryptoTokenKitError> {
        let state = unsafe { &*user_info.cast::<TokenSessionDelegateState>() };
        let session = TokenSession::from_raw(session_raw);
        let object_id = TokenObjectId(c_string_to_string(object_id_ptr, "missing token object identifier")?);
        let algorithm = TokenKeyAlgorithm::from_raw(algorithm_raw);
        let parameters = TokenKeyExchangeParameters::from_raw(parameters_raw);
        let public_key = unsafe { std::slice::from_raw_parts(public_key_ptr, public_key_len) };
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let reply = delegate.perform_key_exchange(
            &session,
            public_key,
            &object_id,
            &algorithm,
            &parameters,
        )?;
        drop(delegate);
        if !out_reply_json.is_null() {
            unsafe {
                *out_reply_json = json_to_ptr(&reply)?;
            }
        }
        Ok(ffi::status::OK)
    })) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            write_error_ptr(error_out, error.message());
            error.code()
        }
        Err(_) => {
            write_error_ptr(error_out, "panic in token-session key-exchange delegate callback");
            ffi::status::FRAMEWORK_ERROR
        }
    }
}

unsafe extern "C" fn token_create_session_trampoline(
    user_info: *mut c_void,
    token_raw: *mut c_void,
    out_session: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    if user_info.is_null() || token_raw.is_null() {
        write_error_ptr(error_out, "missing token delegate callback context");
        return ffi::status::INVALID_ARGUMENT;
    }

    match catch_unwind(AssertUnwindSafe(|| -> Result<i32, CryptoTokenKitError> {
        let state = unsafe { &*user_info.cast::<TokenDelegateState>() };
        let token = Token::from_raw(token_raw);
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let session = delegate.create_session(&token)?;
        drop(delegate);
        if !out_session.is_null() {
            unsafe {
                *out_session = session.map_or(ptr::null_mut(), TokenSession::into_raw);
            }
        }
        Ok(ffi::status::OK)
    })) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            write_error_ptr(error_out, error.message());
            error.code()
        }
        Err(_) => {
            write_error_ptr(error_out, "panic in token create-session delegate callback");
            ffi::status::FRAMEWORK_ERROR
        }
    }
}

unsafe extern "C" fn token_terminate_session_trampoline(
    user_info: *mut c_void,
    token_raw: *mut c_void,
    session_raw: *mut c_void,
) {
    if user_info.is_null() || token_raw.is_null() || session_raw.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<TokenDelegateState>() };
        let token = Token::from_raw(token_raw);
        let session = TokenSession::from_raw(session_raw);
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        delegate.terminate_session(&token, &session);
    }));
}

unsafe extern "C" fn token_driver_create_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    configuration_json: *const c_char,
    out_token: *mut *mut c_void,
    error_out: *mut *mut c_char,
) -> i32 {
    if user_info.is_null() || driver_raw.is_null() || configuration_json.is_null() {
        write_error_ptr(error_out, "missing token-driver delegate callback arguments");
        return ffi::status::INVALID_ARGUMENT;
    }

    match catch_unwind(AssertUnwindSafe(|| -> Result<i32, CryptoTokenKitError> {
        let state = unsafe { &*user_info.cast::<TokenDriverDelegateState>() };
        let driver = TokenDriver::from_raw(driver_raw);
        let configuration: TokenConfigurationSnapshot = serde_json::from_str(&c_string_to_string(
            configuration_json,
            "missing token configuration JSON",
        )?)
        .map_err(|error| {
            CryptoTokenKitError::InvalidArgument(format!(
                "invalid token-driver configuration JSON: {error}"
            ))
        })?;
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let token = delegate.token_for_configuration(&driver, &configuration)?;
        drop(delegate);
        if !out_token.is_null() {
            unsafe {
                *out_token = token.map_or(ptr::null_mut(), Token::into_raw);
            }
        }
        Ok(ffi::status::OK)
    })) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            write_error_ptr(error_out, error.message());
            error.code()
        }
        Err(_) => {
            write_error_ptr(error_out, "panic in token-driver create-token delegate callback");
            ffi::status::FRAMEWORK_ERROR
        }
    }
}

unsafe extern "C" fn token_driver_terminate_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    token_raw: *mut c_void,
) {
    if user_info.is_null() || driver_raw.is_null() || token_raw.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<TokenDriverDelegateState>() };
        let driver = TokenDriver::from_raw(driver_raw);
        let token = Token::from_raw(token_raw);
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        delegate.terminate_token(&driver, &token);
    }));
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
    if user_info.is_null() || driver_raw.is_null() || smart_card_raw.is_null() {
        write_error_ptr(error_out, "missing smart-card token-driver delegate callback arguments");
        return ffi::status::INVALID_ARGUMENT;
    }

    match catch_unwind(AssertUnwindSafe(|| -> Result<i32, CryptoTokenKitError> {
        let state = unsafe { &*user_info.cast::<SmartCardTokenDriverDelegateState>() };
        let driver = SmartCardTokenDriver::from_raw(driver_raw);
        let smart_card = SmartCard::from_raw(smart_card_raw);
        let aid = if has_aid {
            Some(unsafe { std::slice::from_raw_parts(aid_ptr, aid_len) })
        } else {
            None
        };
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let token = delegate.create_token_for_smart_card(&driver, &smart_card, aid)?;
        drop(delegate);
        if !out_token.is_null() {
            unsafe {
                *out_token = token.map_or(ptr::null_mut(), SmartCardToken::into_raw);
            }
        }
        Ok(ffi::status::OK)
    })) {
        Ok(Ok(status)) => status,
        Ok(Err(error)) => {
            write_error_ptr(error_out, error.message());
            error.code()
        }
        Err(_) => {
            write_error_ptr(
                error_out,
                "panic in smart-card token-driver create-token delegate callback",
            );
            ffi::status::FRAMEWORK_ERROR
        }
    }
}

unsafe extern "C" fn smart_card_token_driver_terminate_token_trampoline(
    user_info: *mut c_void,
    driver_raw: *mut c_void,
    token_raw: *mut c_void,
) {
    if user_info.is_null() || driver_raw.is_null() || token_raw.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<SmartCardTokenDriverDelegateState>() };
        let driver = SmartCardTokenDriver::from_raw(driver_raw);
        let token = SmartCardToken::from_raw(token_raw);
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        delegate.terminate_token(&driver, &token);
    }));
}

impl TokenSession {
    pub fn token(&self) -> Result<Token, CryptoTokenKitError> {
        let raw = unsafe { ffi::token_delegate::ctk_token_session_token(self.raw()) };
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token-session token".into(),
            ));
        }
        Ok(Token::from_raw(raw))
    }

    pub fn set_delegate<D>(&self, delegate: D) -> Result<TokenSessionDelegateHandle, CryptoTokenKitError>
    where
        D: TokenSessionDelegate + 'static,
    {
        let state = Box::new(TokenSessionDelegateState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
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
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token-session delegate handle".into(),
            ));
        }
        Ok(TokenSessionDelegateHandle { raw, _state: state })
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_session_has_delegate(self.raw()) }
    }

    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_session_clear_delegate(self.raw()) };
    }

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
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| TokenAuthOperationHandle::from_raw(raw)))
    }

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
                &mut reply_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if reply_ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(reply_ptr)
    }

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
            .map_or((ptr::null(), 0, false), |bytes| (bytes.as_ptr(), bytes.len(), true));
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
                &mut reply_ptr,
                &mut error_ptr,
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
    pub fn token_driver(&self) -> Result<TokenDriver, CryptoTokenKitError> {
        let raw = unsafe { ffi::token_delegate::ctk_token_token_driver(self.raw()) };
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token driver".into(),
            ));
        }
        Ok(TokenDriver::from_raw(raw))
    }

    pub fn set_delegate<D>(&self, delegate: D) -> Result<TokenDelegateHandle, CryptoTokenKitError>
    where
        D: TokenDelegate + 'static,
    {
        let state = Box::new(TokenDelegateState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_set_delegate(
                self.raw(),
                Some(token_create_session_trampoline),
                Some(token_terminate_session_trampoline),
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token delegate handle".into(),
            ));
        }
        Ok(TokenDelegateHandle { raw, _state: state })
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_has_delegate(self.raw()) }
    }

    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_clear_delegate(self.raw()) };
    }

    pub fn invoke_delegate_create_session(&self) -> Result<Option<TokenSession>, CryptoTokenKitError> {
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_invoke_delegate_create_session(
                self.raw(),
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| TokenSession::from_raw(raw)))
    }

    pub fn invoke_delegate_terminate_session(&self, session: &TokenSession) {
        unsafe { ffi::token_delegate::ctk_token_invoke_delegate_terminate_session(self.raw(), session.raw()) };
    }
}

impl TokenDriver {
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
                &mut error_ptr,
            )
        };
        if ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(ptr)
    }

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
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    pub fn set_delegate<D>(&self, delegate: D) -> Result<TokenDriverDelegateHandle, CryptoTokenKitError>
    where
        D: TokenDriverDelegate + 'static,
    {
        let state = Box::new(TokenDriverDelegateState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_token_driver_set_delegate(
                self.raw(),
                Some(token_driver_create_token_trampoline),
                Some(token_driver_terminate_token_trampoline),
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token-driver delegate handle".into(),
            ));
        }
        Ok(TokenDriverDelegateHandle { raw, _state: state })
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_driver_has_delegate(self.raw()) }
    }

    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_driver_clear_delegate(self.raw()) };
    }

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
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| Token::from_raw(raw)))
    }

    pub fn invoke_delegate_terminate_token(&self, token: &Token) {
        unsafe { ffi::token_delegate::ctk_token_driver_invoke_delegate_terminate_token(self.raw(), token.raw()) };
    }
}

impl SmartCardTokenDriver {
    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<SmartCardTokenDriverDelegateHandle, CryptoTokenKitError>
    where
        D: SmartCardTokenDriverDelegate + 'static,
    {
        let state = Box::new(SmartCardTokenDriverDelegateState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_smart_card_token_driver_set_delegate(
                self.raw(),
                Some(smart_card_token_driver_create_token_trampoline),
                Some(smart_card_token_driver_terminate_token_trampoline),
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card token-driver delegate handle".into(),
            ));
        }
        Ok(SmartCardTokenDriverDelegateHandle { raw, _state: state })
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::token_delegate::ctk_token_driver_has_delegate(self.raw()) }
    }

    pub fn clear_delegate(&self) {
        unsafe { ffi::token_delegate::ctk_token_driver_clear_delegate(self.raw()) };
    }

    pub fn invoke_delegate_create_token(
        &self,
        smart_card: &SmartCard,
        aid: Option<&[u8]>,
    ) -> Result<Option<SmartCardToken>, CryptoTokenKitError> {
        let (aid_ptr, aid_len, has_aid) = aid
            .map_or((ptr::null(), 0, false), |bytes| (bytes.as_ptr(), bytes.len(), true));
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_delegate::ctk_smart_card_token_driver_invoke_delegate_create_token(
                self.raw(),
                smart_card.raw(),
                aid_ptr,
                aid_len,
                has_aid,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| SmartCardToken::from_raw(raw)))
    }

    pub fn invoke_delegate_terminate_token(&self, token: &SmartCardToken) {
        unsafe {
            ffi::token_delegate::ctk_smart_card_token_driver_invoke_delegate_terminate_token(
                self.raw(),
                token.raw(),
            );
        };
    }
}
