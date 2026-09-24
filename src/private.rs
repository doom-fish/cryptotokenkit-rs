use std::ffi::CString;
use std::ptr;

use core::ffi::c_char;
use libc::strdup;
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

pub fn encode_json_cstring<T: Serialize + ?Sized>(
    value: &T,
) -> Result<CString, CryptoTokenKitError> {
    let json = serde_json::to_string(value).map_err(|error| {
        CryptoTokenKitError::FrameworkError(format!(
            "failed to encode bridge JSON payload: {error}"
        ))
    })?;
    to_cstring(&json)
}

pub fn status_result(
    status: i32,
    error_ptr: *mut core::ffi::c_char,
) -> Result<(), CryptoTokenKitError> {
    if status == ffi::status::OK {
        Ok(())
    } else {
        Err(crate::error::from_swift(status, error_ptr))
    }
}

pub fn checked<T>(call: impl FnOnce(*mut *mut c_char) -> T) -> Result<T, CryptoTokenKitError> {
    let mut error = ptr::null_mut();
    let value = call(&raw mut error);
    if error.is_null() {
        Ok(value)
    } else {
        Err(CryptoTokenKitError::InvalidArgument(
            crate::error::take_owned_c_string(error),
        ))
    }
}

#[must_use]
pub fn clone_cstring_ptr(value: &CString) -> *mut c_char {
    unsafe { strdup(value.as_ptr()) }
}

#[must_use]
pub fn string_to_ptr(value: &str) -> *mut c_char {
    to_cstring(value).map_or(ptr::null_mut(), |cstring| clone_cstring_ptr(&cstring))
}

pub fn write_error_ptr(error_out: *mut *mut c_char, message: &str) {
    if error_out.is_null() {
        return;
    }
    unsafe {
        *error_out = string_to_ptr(message);
    }
}

pub fn json_to_ptr<T: Serialize + ?Sized>(value: &T) -> Result<*mut c_char, CryptoTokenKitError> {
    encode_json_cstring(value).map(|cstring| clone_cstring_ptr(&cstring))
}

#[cfg(test)]
pub mod test_support {
    use core::ffi::c_void;

    use crate::error::CryptoTokenKitError;

    unsafe extern "C" {
        fn objc_retain(object: *mut c_void) -> *mut c_void;
    }

    pub fn retained(raw: *mut c_void) -> *mut c_void {
        unsafe { objc_retain(raw) }
    }

    pub fn assert_wrong_handle<T>(result: Result<T, CryptoTokenKitError>) {
        match result.err() {
            Some(CryptoTokenKitError::InvalidArgument(message)) => {
                assert!(message.contains("is not a"), "{message}");
            }
            other => panic!("expected a handle-type error, got {other:?}"),
        }
    }
}
