use std::ffi::CString;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::CryptoTokenKitError;
use crate::ffi;

pub fn to_cstring(value: &str) -> Result<CString, CryptoTokenKitError> {
    CString::new(value).map_err(|_| {
        CryptoTokenKitError::InvalidArgument("strings must not contain interior NUL bytes".into())
    })
}

pub fn decode_json<T: DeserializeOwned>(
    ptr: *mut core::ffi::c_char,
) -> Result<T, CryptoTokenKitError> {
    let json = crate::error::take_owned_c_string(ptr);
    serde_json::from_str(&json).map_err(|error| {
        CryptoTokenKitError::FrameworkError(format!(
            "failed to decode bridge JSON payload: {error}"
        ))
    })
}

pub fn decode_optional_json<T: DeserializeOwned>(
    ptr: *mut core::ffi::c_char,
) -> Result<Option<T>, CryptoTokenKitError> {
    if ptr.is_null() {
        return Ok(None);
    }

    decode_json(ptr).map(Some)
}

pub fn encode_json_cstring<T: Serialize + ?Sized>(value: &T) -> Result<CString, CryptoTokenKitError> {
    let json = serde_json::to_string(value).map_err(|error| {
        CryptoTokenKitError::FrameworkError(format!(
            "failed to encode bridge JSON payload: {error}"
        ))
    })?;
    to_cstring(&json)
}

pub fn status_result(status: i32, error_ptr: *mut core::ffi::c_char) -> Result<(), CryptoTokenKitError> {
    if status == ffi::status::OK {
        Ok(())
    } else {
        Err(crate::error::from_swift(status, error_ptr))
    }
}
