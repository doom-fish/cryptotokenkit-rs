use core::ffi::c_void;
use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::{decode_json, status_result};
use crate::smart_card_atr::SmartCardProtocol;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Result payload returned from `TKSmartCard.send(ins:p1:p2:data:le:)`.
pub struct ApduResponse {
    #[serde(default, with = "serde_bytes")]
    /// Serialized field bridged from `TKSmartCard.send(ins:p1:p2:data:le:)`.
    pub data: Vec<u8>,
    /// Serialized field bridged from `TKSmartCard.send(ins:p1:p2:data:le:)`.
    pub status_word: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
/// Mirrors the character-set choices stored by `TKSmartCardPINFormat`.
pub enum SmartCardPinCharset {
    /// Variant bridged from `TKSmartCardPINFormat`.
    Numeric = 0,
    /// Variant bridged from `TKSmartCardPINFormat`.
    Alphanumeric = 1,
    /// Variant bridged from `TKSmartCardPINFormat`.
    UpperAlphanumeric = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
/// Mirrors the encoding choices stored by `TKSmartCardPINFormat`.
pub enum SmartCardPinEncoding {
    /// Variant bridged from `TKSmartCardPINFormat`.
    Binary = 0,
    /// Variant bridged from `TKSmartCardPINFormat`.
    Ascii = 1,
    /// Variant bridged from `TKSmartCardPINFormat`.
    Bcd = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
/// Mirrors the justification options stored by `TKSmartCardPINFormat`.
pub enum SmartCardPinJustification {
    /// Variant bridged from `TKSmartCardPINFormat`.
    Left = 0,
    /// Variant bridged from `TKSmartCardPINFormat`.
    Right = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Wraps the completion flags used by `CryptoTokenKit` PIN interactions.
pub struct SmartCardPinCompletion(
    /// Serialized field bridged from `CryptoTokenKit PIN interaction flags`.
    pub u32,
);

impl SmartCardPinCompletion {
    /// Wraps the corresponding `CryptoTokenKit PIN interaction flags` operation.
    pub const MAX_LENGTH: Self = Self(1 << 0);
    /// Wraps the corresponding `CryptoTokenKit PIN interaction flags` operation.
    pub const KEY: Self = Self(1 << 1);
    /// Wraps the corresponding `CryptoTokenKit PIN interaction flags` operation.
    pub const TIMEOUT: Self = Self(1 << 2);

    #[must_use]
    /// Wraps the corresponding `CryptoTokenKit PIN interaction flags` operation.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Wraps the confirmation flags used by `CryptoTokenKit` PIN-change interactions.
pub struct SmartCardPinConfirmation(
    /// Serialized field bridged from `CryptoTokenKit PIN change interaction flags`.
    pub u32,
);

impl SmartCardPinConfirmation {
    /// Wraps the corresponding `CryptoTokenKit PIN change interaction flags` operation.
    pub const NONE: Self = Self(0);
    /// Wraps the corresponding `CryptoTokenKit PIN change interaction flags` operation.
    pub const NEW: Self = Self(1 << 0);
    /// Wraps the corresponding `CryptoTokenKit PIN change interaction flags` operation.
    pub const CURRENT: Self = Self(1 << 1);

    #[must_use]
    /// Wraps the corresponding `CryptoTokenKit PIN change interaction flags` operation.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of the `TKSmartCardPINFormat` configuration.
pub struct SmartCardPinFormat {
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub charset: SmartCardPinCharset,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub encoding: SmartCardPinEncoding,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub min_pin_length: i64,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub max_pin_length: i64,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub pin_block_byte_length: i64,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub pin_justification: SmartCardPinJustification,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub pin_bit_offset: i64,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub pin_length_bit_offset: i64,
    /// Serialized field bridged from `TKSmartCardPINFormat`.
    pub pin_length_bit_size: i64,
}

impl Default for SmartCardPinFormat {
    fn default() -> Self {
        Self {
            charset: SmartCardPinCharset::Numeric,
            encoding: SmartCardPinEncoding::Ascii,
            min_pin_length: 4,
            max_pin_length: 8,
            pin_block_byte_length: 8,
            pin_justification: SmartCardPinJustification::Left,
            pin_bit_offset: 0,
            pin_length_bit_offset: 0,
            pin_length_bit_size: 0,
        }
    }
}

/// Wraps `TKSmartCard`.
pub struct SmartCard {
    raw: *mut c_void,
}

impl SmartCard {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    #[must_use]
    pub(crate) const fn raw(&self) -> *mut c_void {
        self.raw
    }

    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn slot_name(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = unsafe { ffi::smart_card::ctk_smart_card_slot_name(self.raw) };
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null slot name".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn valid(&self) -> bool {
        unsafe { ffi::smart_card::ctk_smart_card_valid(self.raw) }
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn allowed_protocols(&self) -> SmartCardProtocol {
        SmartCardProtocol::from_bits(unsafe {
            ffi::smart_card::ctk_smart_card_allowed_protocols(self.raw)
        })
    }

    /// Sets the corresponding `TKSmartCard` value.
    pub fn set_allowed_protocols(&self, protocols: SmartCardProtocol) {
        unsafe {
            ffi::smart_card::ctk_smart_card_set_allowed_protocols(self.raw, protocols.bits());
        };
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn current_protocol(&self) -> SmartCardProtocol {
        SmartCardProtocol::from_bits(unsafe {
            ffi::smart_card::ctk_smart_card_current_protocol(self.raw)
        })
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn sensitive(&self) -> bool {
        unsafe { ffi::smart_card::ctk_smart_card_sensitive(self.raw) }
    }

    /// Sets the corresponding `TKSmartCard` value.
    pub fn set_sensitive(&self, sensitive: bool) {
        unsafe { ffi::smart_card::ctk_smart_card_set_sensitive(self.raw, sensitive) };
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn cla(&self) -> u8 {
        unsafe { ffi::smart_card::ctk_smart_card_cla(self.raw) }
    }

    /// Sets the corresponding `TKSmartCard` value.
    pub fn set_cla(&self, cla: u8) {
        unsafe { ffi::smart_card::ctk_smart_card_set_cla(self.raw, cla) };
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn use_extended_length(&self) -> bool {
        unsafe { ffi::smart_card::ctk_smart_card_use_extended_length(self.raw) }
    }

    /// Sets the corresponding `TKSmartCard` value.
    pub fn set_use_extended_length(&self, enabled: bool) {
        unsafe { ffi::smart_card::ctk_smart_card_set_use_extended_length(self.raw, enabled) };
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn use_command_chaining(&self) -> bool {
        unsafe { ffi::smart_card::ctk_smart_card_use_command_chaining(self.raw) }
    }

    /// Sets the corresponding `TKSmartCard` value.
    pub fn set_use_command_chaining(&self, enabled: bool) {
        unsafe { ffi::smart_card::ctk_smart_card_set_use_command_chaining(self.raw, enabled) };
    }

    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn context(&self) -> Result<Option<String>, CryptoTokenKitError> {
        let ptr = unsafe { ffi::smart_card::ctk_smart_card_context_json(self.raw) };
        if ptr.is_null() {
            return Ok(None);
        }
        Ok(Some(crate::error::take_owned_c_string(ptr)))
    }

    /// Sets the corresponding `TKSmartCard` value.
    pub fn set_context(&self, json: Option<&str>) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let (status, _storage) = if let Some(json) = json {
            let storage = crate::private::to_cstring(json)?;
            let status = unsafe {
                ffi::smart_card::ctk_smart_card_set_context_json(
                    self.raw,
                    storage.as_ptr(),
                    true,
                    &mut error_ptr,
                )
            };
            (status, Some(storage))
        } else {
            let status = unsafe {
                ffi::smart_card::ctk_smart_card_set_context_json(
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

    /// Invokes the corresponding `TKSmartCard` operation.
    pub fn begin_session(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status =
            unsafe { ffi::smart_card::ctk_smart_card_begin_session(self.raw, &mut error_ptr) };
        status_result(status, error_ptr)
    }

    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn transmit_request(&self, request: &[u8]) -> Result<Vec<u8>, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let mut reply_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card::ctk_smart_card_transmit_request_json(
                self.raw,
                request.as_ptr(),
                request.len(),
                &mut reply_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if reply_ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null transmit reply".into(),
            ));
        }
        decode_json(reply_ptr)
    }

    /// Invokes the corresponding `TKSmartCard` operation.
    pub fn end_session(&self) {
        unsafe { ffi::smart_card::ctk_smart_card_end_session(self.raw) };
    }

    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn send_ins(
        &self,
        ins: u8,
        p1: u8,
        p2: u8,
        data: Option<&[u8]>,
        le: Option<usize>,
    ) -> Result<ApduResponse, CryptoTokenKitError> {
        let (data_ptr, data_len) =
            data.map_or((ptr::null(), 0), |bytes| (bytes.as_ptr(), bytes.len()));
        let mut reply_ptr = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card::ctk_smart_card_send_ins(
                self.raw,
                ins,
                p1,
                p2,
                data_ptr,
                data_len,
                le.is_some(),
                le.unwrap_or_default(),
                &mut reply_ptr,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if reply_ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null APDU reply".into(),
            ));
        }
        decode_json(reply_ptr)
    }

    /// Runs the corresponding `TKSmartCard` workflow around the provided callback.
    pub fn with_session<T>(
        &self,
        callback: impl FnOnce(&Self) -> Result<T, CryptoTokenKitError>,
    ) -> Result<T, CryptoTokenKitError> {
        self.begin_session()?;
        let result = callback(self);
        self.end_session();
        result
    }
}

impl Drop for SmartCard {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
