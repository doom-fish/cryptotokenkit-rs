use core::ffi::c_char;
use core::fmt;

use libc::free;

use crate::ffi;

/// Mirrors the `TKErrorDomain` string exported by the `CryptoTokenKit` framework.
pub const TK_ERROR_DOMAIN: &str = "CryptoTokenKit";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors the `TKError.Code` values reported by `CryptoTokenKit` APIs.
pub enum TKErrorCode {
    /// Variant bridged from `TKError.Code`.
    NotImplemented = -1,
    /// Variant bridged from `TKError.Code`.
    CommunicationError = -2,
    /// Variant bridged from `TKError.Code`.
    CorruptedData = -3,
    /// Variant bridged from `TKError.Code`.
    CanceledByUser = -4,
    /// Variant bridged from `TKError.Code`.
    AuthenticationFailed = -5,
    /// Variant bridged from `TKError.Code`.
    ObjectNotFound = -6,
    /// Variant bridged from `TKError.Code`.
    TokenNotFound = -7,
    /// Variant bridged from `TKError.Code`.
    BadParameter = -8,
    /// Variant bridged from `TKError.Code`.
    AuthenticationNeeded = -9,
}

impl TKErrorCode {
    #[must_use]
    /// Wraps the corresponding `TKError.Code` operation.
    pub const fn from_raw(raw: i32) -> Option<Self> {
        match raw {
            -1 => Some(Self::NotImplemented),
            -2 => Some(Self::CommunicationError),
            -3 => Some(Self::CorruptedData),
            -4 => Some(Self::CanceledByUser),
            -5 => Some(Self::AuthenticationFailed),
            -6 => Some(Self::ObjectNotFound),
            -7 => Some(Self::TokenNotFound),
            -8 => Some(Self::BadParameter),
            -9 => Some(Self::AuthenticationNeeded),
            _ => None,
        }
    }
}

impl TryFrom<i32> for TKErrorCode {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_raw(value).ok_or(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Safe error wrapper for failures surfaced by the `CryptoTokenKit` framework.
pub enum CryptoTokenKitError {
    /// Variant bridged from `CryptoTokenKit NSError`.
    InvalidArgument(String),
    /// Variant bridged from `CryptoTokenKit NSError`.
    FrameworkError(String),
    /// Variant bridged from `CryptoTokenKit NSError`.
    TimedOut(String),
    /// Variant bridged from `CryptoTokenKit NSError`.
    Unknown {
        /// Raw status code bridged from the framework error payload.
        code: i32,
        /// Human-readable message bridged from the framework error payload.
        message: String,
    },
}

impl CryptoTokenKitError {
    #[must_use]
    /// Wraps the corresponding `CryptoTokenKit NSError` operation.
    pub const fn code(&self) -> i32 {
        match self {
            Self::InvalidArgument(_) => ffi::status::INVALID_ARGUMENT,
            Self::FrameworkError(_) => ffi::status::FRAMEWORK_ERROR,
            Self::TimedOut(_) => ffi::status::TIMED_OUT,
            Self::Unknown { code, .. } => *code,
        }
    }

    #[must_use]
    /// Wraps the corresponding `CryptoTokenKit NSError` operation.
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidArgument(message)
            | Self::FrameworkError(message)
            | Self::TimedOut(message)
            | Self::Unknown { message, .. } => message,
        }
    }

    #[must_use]
    /// Wraps the corresponding `CryptoTokenKit NSError` operation.
    pub fn framework_code(&self) -> Option<TKErrorCode> {
        TKErrorCode::from_raw(self.code())
    }
}

impl fmt::Display for CryptoTokenKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (code {})", self.message(), self.code())
    }
}

impl std::error::Error for CryptoTokenKitError {}

pub(crate) fn take_owned_c_string(ptr: *mut c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    // SAFETY: ptr is null-checked above and is expected to be a valid C string allocated by Swift.
    let string = unsafe { core::ffi::CStr::from_ptr(ptr) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: ptr is allocated by the Swift bridge and must be freed via libc::free().
    unsafe { free(ptr.cast()) };
    string
}

pub(crate) fn from_swift(status: i32, error_str: *mut c_char) -> CryptoTokenKitError {
    from_status_message(status, take_owned_c_string(error_str))
}

pub(crate) fn from_status_message(status: i32, message: String) -> CryptoTokenKitError {
    match status {
        ffi::status::INVALID_ARGUMENT => CryptoTokenKitError::InvalidArgument(message),
        ffi::status::FRAMEWORK_ERROR => CryptoTokenKitError::FrameworkError(message),
        ffi::status::TIMED_OUT => CryptoTokenKitError::TimedOut(message),
        code => CryptoTokenKitError::Unknown { code, message },
    }
}
