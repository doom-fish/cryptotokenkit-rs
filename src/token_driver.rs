use core::ffi::c_void;
use std::collections::BTreeMap;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::decode_json;
use crate::token::TokenConfigurationSnapshot;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDriverConfigurationSnapshot {
    pub class_id: String,
    pub token_configurations: BTreeMap<String, TokenConfigurationSnapshot>,
}

pub struct TokenDriver {
    raw: *mut c_void,
}

pub struct SmartCardTokenDriver {
    raw: *mut c_void,
}

impl TokenDriver {
    #[must_use]
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_driver::ctk_token_driver_new() };
        assert!(!raw.is_null(), "Swift bridge returned a null token driver");
        Self { raw }
    }

    pub fn driver_configurations(
    ) -> Result<BTreeMap<String, TokenDriverConfigurationSnapshot>, CryptoTokenKitError> {
        let ptr = unsafe { ffi::token_driver::ctk_driver_configurations_json() };
        if ptr.is_null() {
            return Ok(BTreeMap::new());
        }
        decode_json(ptr)
    }

    #[must_use]
    pub(crate) const fn raw(&self) -> *mut c_void {
        self.raw
    }

    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }
}

impl Default for TokenDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl SmartCardTokenDriver {
    #[must_use]
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_driver::ctk_smart_card_token_driver_new() };
        assert!(
            !raw.is_null(),
            "Swift bridge returned a null smart-card token driver"
        );
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
}

impl Default for SmartCardTokenDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TokenDriver {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Drop for SmartCardTokenDriver {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
