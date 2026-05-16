use std::ffi::CString;

use serde::de::DeserializeOwned;

use crate::error::CryptoTokenKitError;

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
