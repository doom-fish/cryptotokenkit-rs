use core::ffi::{c_char, c_void};
use std::ffi::CStr;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, CryptoTokenKitError};
use crate::ffi;
use crate::private::{decode_json, decode_optional_json, status_result, to_cstring};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenWatcherTokenInfo {
    pub token_id: String,
    pub slot_name: Option<String>,
    pub driver_name: Option<String>,
}

type TokenHandler = Box<dyn FnMut(String) + Send + 'static>;

struct CallbackState {
    callback: Mutex<TokenHandler>,
}

#[allow(clippy::vec_box)]
pub struct TokenWatcher {
    raw: *mut c_void,
    insertion_state: Option<Box<CallbackState>>,
    removal_states: Vec<Box<CallbackState>>,
}

unsafe extern "C" fn token_watcher_trampoline(user_info: *mut c_void, token_id: *const c_char) {
    if user_info.is_null() || token_id.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<CallbackState>() };
        let token_id = unsafe { CStr::from_ptr(token_id) }
            .to_string_lossy()
            .into_owned();
        let mut callback = match state.callback.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        callback(token_id);
    }));
}

impl TokenWatcher {
    #[must_use]
    pub fn new() -> Self {
        let raw = unsafe { ffi::token_watcher::ctk_token_watcher_new() };
        assert!(!raw.is_null(), "Swift bridge returned a null token watcher");
        Self {
            raw,
            insertion_state: None,
            removal_states: Vec::new(),
        }
    }

    pub fn token_ids(&self) -> Result<Vec<String>, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let ptr = unsafe {
            ffi::token_watcher::ctk_token_watcher_token_ids_json(self.raw, &mut error_ptr)
        };
        if ptr.is_null() && !error_ptr.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        if ptr.is_null() {
            return Ok(Vec::new());
        }
        decode_json(ptr)
    }

    pub fn set_insertion_handler(
        &mut self,
        callback: impl FnMut(String) + Send + 'static,
    ) -> Result<(), CryptoTokenKitError> {
        let state = Box::new(CallbackState {
            callback: Mutex::new(Box::new(callback)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_watcher::ctk_token_watcher_set_insertion_handler(
                self.raw,
                Some(token_watcher_trampoline),
                user_info,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        self.insertion_state = Some(state);
        Ok(())
    }

    pub fn add_removal_handler(
        &mut self,
        token_id: &str,
        callback: impl FnMut(String) + Send + 'static,
    ) -> Result<(), CryptoTokenKitError> {
        let token_id = to_cstring(token_id)?;
        let state = Box::new(CallbackState {
            callback: Mutex::new(Box::new(callback)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::token_watcher::ctk_token_watcher_add_removal_handler(
                self.raw,
                token_id.as_ptr(),
                Some(token_watcher_trampoline),
                user_info,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        self.removal_states.push(state);
        Ok(())
    }

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
                &mut error_ptr,
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
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
