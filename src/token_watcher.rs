use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::ptr;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;
use serde::{Deserialize, Serialize};

use crate::error::{from_swift, CryptoTokenKitError};
use crate::ffi;
use crate::private::{decode_json, decode_optional_json, status_result, to_cstring};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of token metadata returned by `TKTokenWatcher`.
pub struct TokenWatcherTokenInfo {
    /// Serialized field bridged from `TKTokenWatcher`.
    pub token_id: String,
    /// Serialized field bridged from `TKTokenWatcher`.
    pub slot_name: Option<String>,
    /// Serialized field bridged from `TKTokenWatcher`.
    pub driver_name: Option<String>,
}

type TokenHandlerCell = Mutex<Box<dyn FnMut(String) + Send + 'static>>;

/// Wraps `TKTokenWatcher`.
pub struct TokenWatcher {
    raw: *mut c_void,
    insertion_context: Option<CallbackContext<TokenHandlerCell>>,
    removal_contexts: Vec<CallbackContext<TokenHandlerCell>>,
}

unsafe extern "C" fn token_watcher_trampoline(user_info: *mut c_void, token_id: *const c_char) {
    if token_id.is_null() {
        return;
    }

    let deliver = |callback: &TokenHandlerCell| {
        let token_id = unsafe { CStr::from_ptr(token_id) }
            .to_string_lossy()
            .into_owned();
        let mut callback = callback.lock().unwrap_or_else(PoisonError::into_inner);
        callback(token_id);
    };
    unsafe {
        CallbackContext::<TokenHandlerCell>::with(user_info, "TokenWatcher handler", deliver)
    };
}

impl TokenWatcher {
    #[must_use]
    /// Creates a new wrapper around `TKTokenWatcher`.
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_watcher::ctk_token_watcher_new() };
        assert!(!raw.is_null(), "Swift bridge returned a null token watcher");
        Self {
            raw,
            insertion_context: None,
            removal_contexts: Vec::new(),
        }
    }

    /// Returns the corresponding `TKTokenWatcher` value.
    pub fn token_ids(&self) -> Result<Vec<String>, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token_watcher::ctk_token_watcher_token_ids_json(self.raw, &raw mut error_ptr)
        };
        if ptr.is_null() && !error_ptr.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        if ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(ptr)
    }

    /// Sets the corresponding `TKTokenWatcher` value.
    pub fn set_insertion_handler(
        &mut self,
        callback: impl FnMut(String) + Send + 'static,
    ) -> Result<(), CryptoTokenKitError> {
        let cell: TokenHandlerCell = Mutex::new(Box::new(callback));
        let context = CallbackContext::new(cell);
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_watcher::ctk_token_watcher_set_insertion_handler(
                self.raw,
                Some(token_watcher_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<TokenHandlerCell>::RELEASE),
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        self.insertion_context = Some(context);
        Ok(())
    }

    /// Registers the corresponding `TKTokenWatcher` callback.
    pub fn add_removal_handler(
        &mut self,
        token_id: &str,
        callback: impl FnMut(String) + Send + 'static,
    ) -> Result<(), CryptoTokenKitError> {
        let token_id = to_cstring(token_id)?;
        let cell: TokenHandlerCell = Mutex::new(Box::new(callback));
        let context = CallbackContext::new(cell);
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_watcher::ctk_token_watcher_add_removal_handler(
                self.raw,
                token_id.as_ptr(),
                Some(token_watcher_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<TokenHandlerCell>::RELEASE),
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        self.removal_contexts.push(context);
        Ok(())
    }

    /// Returns the corresponding `TKTokenWatcher` value.
    pub fn token_info(
        &self,
        token_id: &str,
    ) -> Result<Option<TokenWatcherTokenInfo>, CryptoTokenKitError> {
        let token_id = to_cstring(token_id)?;
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token_watcher::ctk_token_watcher_token_info_json(
                self.raw,
                token_id.as_ptr(),
                &raw mut error_ptr,
            )
        };
        if ptr.is_null() && !error_ptr.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        decode_optional_json(ptr)
    }
}

impl Default for TokenWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TokenWatcher {
    fn drop(&mut self) {
        for context in self.insertion_context.iter().chain(&self.removal_contexts) {
            context.deactivate();
        }
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
