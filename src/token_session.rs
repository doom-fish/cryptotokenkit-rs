use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::{decode_json, encode_json_cstring, status_result};
use crate::smart_card::{SmartCard, SmartCardPinFormat};
use crate::token::{SmartCardToken, Token};

pub struct TokenSession {
    raw: *mut c_void,
}

pub struct SmartCardTokenSession {
    raw: *mut c_void,
}

pub struct TokenAuthOperation {
    raw: *mut c_void,
}

pub struct TokenPasswordAuthOperation {
    raw: *mut c_void,
}

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
    pub pin: Option<String>,
}

impl TokenSession {
    #[must_use]
    pub fn new(token: &Token) -> Self {
        let raw = unsafe { ffi::token_session::ctk_token_session_new(token.raw()) };
        assert!(!raw.is_null(), "Swift bridge returned a null token session");
        Self { raw }
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

    pub fn token_instance_id(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = unsafe { ffi::token_session::ctk_token_session_token_instance_id(self.raw) };
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null session token identifier".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

}

impl SmartCardTokenSession {
    #[must_use]
    pub fn new(token: &SmartCardToken) -> Self {
        let raw = unsafe { ffi::token_session::ctk_smart_card_token_session_new(token.raw()) };
        assert!(
            !raw.is_null(),
            "Swift bridge returned a null smart-card token session"
        );
        Self { raw }
    }

    pub fn token_instance_id(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = unsafe { ffi::token_session::ctk_token_session_token_instance_id(self.raw) };
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null session token identifier".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    pub fn smart_card(&self) -> Option<SmartCard> {
        let raw = unsafe { ffi::token_session::ctk_smart_card_token_session_smart_card(self.raw) };
        (!raw.is_null()).then_some(SmartCard::from_raw(raw))
    }

    pub fn get_smart_card(&self) -> Result<Option<SmartCard>, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let raw = unsafe {
            ffi::token_session::ctk_smart_card_token_session_get_smart_card(
                self.raw,
                &mut error_ptr,
            )
        };
        if raw.is_null() && !error_ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        Ok((!raw.is_null()).then_some(SmartCard::from_raw(raw)))
    }
}

impl TokenAuthOperation {
    #[must_use]
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

    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_auth_operation_finish(self.raw, &mut error_ptr)
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

    pub fn password(&self) -> Result<Option<String>, CryptoTokenKitError> {
        let ptr = unsafe { ffi::token_session::ctk_token_password_auth_operation_password(self.raw) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::error::take_owned_c_string(ptr)))
    }

    pub fn set_password(&self, password: Option<&str>) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let (status, _storage) = if let Some(password) = password {
            let storage = crate::private::to_cstring(password)?;
            let status = unsafe {
                ffi::token_session::ctk_token_password_auth_operation_set_password(
                    self.raw,
                    storage.as_ptr(),
                    true,
                    &mut error_ptr,
                )
            };
            (status, Some(storage))
        } else {
            let status = unsafe {
                ffi::token_session::ctk_token_password_auth_operation_set_password(
                    self.raw,
                    ptr::null(),
                    false,
                    &mut error_ptr,
                )
            };
            (status, None)
        };
        status_result(status, error_ptr)
    }

    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_auth_operation_finish(self.raw, &mut error_ptr)
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
        let ptr = unsafe { ffi::token_session::ctk_token_smart_card_pin_auth_operation_json(self.raw) };
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
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    pub fn pin_format(&self) -> Result<SmartCardPinFormat, CryptoTokenKitError> {
        Ok(self.snapshot()?.pin_format)
    }

    pub fn set_pin_format(&self, pin_format: SmartCardPinFormat) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.pin_format = pin_format;
        self.update(&snapshot)
    }

    pub fn apdu_template(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        Ok(self.snapshot()?.apdu_template)
    }

    pub fn set_apdu_template(&self, apdu_template: Option<Vec<u8>>) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.apdu_template = apdu_template;
        self.update(&snapshot)
    }

    pub fn pin_byte_offset(&self) -> Result<i64, CryptoTokenKitError> {
        Ok(self.snapshot()?.pin_byte_offset)
    }

    pub fn set_pin_byte_offset(&self, pin_byte_offset: i64) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.pin_byte_offset = pin_byte_offset;
        self.update(&snapshot)
    }

    pub fn has_smart_card(&self) -> Result<bool, CryptoTokenKitError> {
        Ok(self.snapshot()?.has_smart_card)
    }

    pub fn set_smart_card(&self, smart_card: Option<&SmartCard>) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let raw = smart_card.map_or(ptr::null_mut(), SmartCard::raw);
        let status = unsafe {
            ffi::token_session::ctk_token_smart_card_pin_auth_operation_set_smart_card(
                self.raw,
                raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    pub fn pin(&self) -> Result<Option<String>, CryptoTokenKitError> {
        Ok(self.snapshot()?.pin)
    }

    pub fn set_pin(&self, pin: Option<&str>) -> Result<(), CryptoTokenKitError> {
        let mut snapshot = self.snapshot()?;
        snapshot.pin = pin.map(str::to_owned);
        self.update(&snapshot)
    }

    pub fn finish(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_session::ctk_token_auth_operation_finish(self.raw, &mut error_ptr)
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
