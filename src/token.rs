use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::{decode_json, decode_optional_json, encode_json_cstring, status_result, to_cstring};
use crate::smart_card::SmartCard;
use crate::token_driver::{SmartCardTokenDriver, TokenDriver};
use crate::token_keychain_contents::{
    TokenKeychainCertificate, TokenKeychainEntry, TokenKeychainKey, TokenObjectId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenConfigurationSnapshot {
    pub instance_id: String,
    pub configuration_data: Option<Vec<u8>>,
    pub keychain_items: Vec<TokenKeychainEntry>,
    pub keychain_contents_items: Option<Vec<TokenKeychainEntry>>,
}

pub struct Token {
    raw: *mut c_void,
}

pub struct SmartCardToken {
    raw: *mut c_void,
}

impl Token {
    pub fn new(driver: &TokenDriver, instance_id: &str) -> Result<Self, CryptoTokenKitError> {
        let instance_id = to_cstring(instance_id)?;
        let mut error_ptr = ptr::null_mut();
        let raw = unsafe {
            ffi::token::ctk_token_new(driver.raw(), instance_id.as_ptr(), &mut error_ptr)
        };
        if raw.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
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

    pub fn configuration(&self) -> Result<TokenConfigurationSnapshot, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe { ffi::token::ctk_token_configuration_json(self.raw, &mut error_ptr) };
        if ptr.is_null() && !error_ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null token configuration".into(),
            ));
        }
        decode_json(ptr)
    }

    pub fn instance_id(&self) -> Result<String, CryptoTokenKitError> {
        Ok(self.configuration()?.instance_id)
    }

    pub fn set_configuration_data(&self, data: Option<&[u8]>) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let (data_ptr, data_len, has_data) = data
            .map_or((ptr::null(), 0, false), |bytes| (bytes.as_ptr(), bytes.len(), true));
        let status = unsafe {
            ffi::token::ctk_token_set_configuration_data(
                self.raw,
                data_ptr,
                data_len,
                has_data,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    pub fn set_keychain_items(
        &self,
        items: &[TokenKeychainEntry],
    ) -> Result<(), CryptoTokenKitError> {
        let payload = encode_json_cstring(items)?;
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token::ctk_token_set_keychain_items_json(
                self.raw,
                payload.as_ptr(),
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    pub fn key_for_object_id(
        &self,
        object_id: &TokenObjectId,
    ) -> Result<TokenKeychainKey, CryptoTokenKitError> {
        let object_id = to_cstring(&object_id.0)?;
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token::ctk_token_key_for_object_id_json(self.raw, object_id.as_ptr(), &mut error_ptr)
        };
        if ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(ptr)
    }

    pub fn certificate_for_object_id(
        &self,
        object_id: &TokenObjectId,
    ) -> Result<TokenKeychainCertificate, CryptoTokenKitError> {
        let object_id = to_cstring(&object_id.0)?;
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token::ctk_token_certificate_for_object_id_json(
                self.raw,
                object_id.as_ptr(),
                &mut error_ptr,
            )
        };
        if ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_json(ptr)
    }

    pub fn keychain_contents_items(&self) -> Result<Option<Vec<TokenKeychainEntry>>, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token::ctk_token_keychain_contents_items_json(self.raw, &mut error_ptr)
        };
        if ptr.is_null() && !error_ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_optional_json(ptr)
    }
}

impl SmartCardToken {
    pub fn new(
        smart_card: &SmartCard,
        aid: Option<&[u8]>,
        instance_id: &str,
        driver: &SmartCardTokenDriver,
    ) -> Result<Self, CryptoTokenKitError> {
        let instance_id = to_cstring(instance_id)?;
        let (aid_ptr, aid_len, has_aid) = aid
            .map_or((ptr::null(), 0, false), |bytes| (bytes.as_ptr(), bytes.len(), true));
        let mut error_ptr = ptr::null_mut();
        let raw = unsafe {
            ffi::token::ctk_smart_card_token_new(
                smart_card.raw(),
                aid_ptr,
                aid_len,
                has_aid,
                instance_id.as_ptr(),
                driver.raw(),
                &mut error_ptr,
            )
        };
        if raw.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
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

    pub fn aid(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        let ptr = unsafe { ffi::token::ctk_smart_card_token_aid_json(self.raw) };
        decode_optional_json(ptr)
    }

    pub fn configuration(&self) -> Result<TokenConfigurationSnapshot, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe { ffi::token::ctk_token_configuration_json(self.raw, &mut error_ptr) };
        if ptr.is_null() && !error_ptr.is_null() {
            return Err(crate::error::from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card token configuration".into(),
            ));
        }
        decode_json(ptr)
    }
}

impl Drop for Token {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Drop for SmartCardToken {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
