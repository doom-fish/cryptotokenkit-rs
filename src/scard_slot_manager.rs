use core::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::error::{from_swift, CryptoTokenKitError};
use crate::ffi;
use crate::private::{decode_json, decode_optional_json, status_result, to_cstring};
use crate::smart_card::SmartCard;
use crate::smart_card_atr::SmartCardAtr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
/// Mirrors the `TKSmartCardSlot.State` values reported by `CryptoTokenKit`.
pub enum SlotState {
    /// Variant bridged from `TKSmartCardSlot.State`.
    Missing = 0,
    /// Variant bridged from `TKSmartCardSlot.State`.
    Empty = 1,
    /// Variant bridged from `TKSmartCardSlot.State`.
    Probing = 2,
    /// Variant bridged from `TKSmartCardSlot.State`.
    MuteCard = 3,
    /// Variant bridged from `TKSmartCardSlot.State`.
    ValidCard = 4,
}

impl SlotState {
    #[must_use]
    /// Wraps the corresponding `TKSmartCardSlot.State` operation.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::Empty,
            2 => Self::Probing,
            3 => Self::MuteCard,
            4 => Self::ValidCard,
            _ => Self::Missing,
        }
    }
}

/// Rust callback bridge for observing `TKSmartCardSlot.state` changes.
pub trait SlotStateDelegate: Send {
    /// Handles the corresponding `TKSmartCardSlot.state` callback.
    fn did_change_state(&mut self, state: SlotState) {
        let _ = state;
    }
}

type StateHandler = Box<dyn FnMut(SlotState) + Send + 'static>;

/// Closure-backed adapter for `TKSmartCardSlot.state` observation callbacks.
pub struct SlotStateCallbacks {
    state: Option<StateHandler>,
}

impl SlotStateCallbacks {
    #[must_use]
    /// Creates a new wrapper around `TKSmartCardSlot.state`.
    pub fn new() -> Self {
        Self { state: None }
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCardSlot.state` operation.
    pub fn on_state_change(mut self, callback: impl FnMut(SlotState) + Send + 'static) -> Self {
        self.state = Some(Box::new(callback));
        self
    }
}

impl Default for SlotStateCallbacks {
    fn default() -> Self {
        Self::new()
    }
}

impl SlotStateDelegate for SlotStateCallbacks {
    fn did_change_state(&mut self, state: SlotState) {
        if let Some(callback) = &mut self.state {
            callback(state);
        }
    }
}

struct CallbackState {
    delegate: Mutex<Box<dyn SlotStateDelegate>>,
}

/// Lifetime token for a bridged `TKSmartCardSlot.state` observer.
pub struct SlotStateObserver {
    raw: *mut c_void,
    _callback_state: Box<CallbackState>,
}

/// Wraps `TKSmartCardSlotManager`.
pub struct SmartCardSlotManager {
    raw: *mut c_void,
}

/// Wraps `TKSmartCardSlot`.
pub struct SmartCardSlot {
    raw: *mut c_void,
}

impl SmartCardSlot {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }
}

unsafe extern "C" fn slot_state_trampoline(user_info: *mut c_void, raw_state: i32) {
    if user_info.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<CallbackState>() };
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        delegate.did_change_state(SlotState::from_raw(raw_state));
    }));
}

impl SmartCardSlotManager {
    #[must_use]
    /// Wraps the corresponding `TKSmartCardSlotManager` operation.
    pub fn default_manager() -> Option<Self> {
        let raw = unsafe { ffi::scard_slot_manager::ctk_slot_manager_default() };
        (!raw.is_null()).then_some(Self { raw })
    }

    /// Wraps the corresponding `TKSmartCardSlotManager` operation.
    pub fn slot_names(&self) -> Result<Vec<String>, CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let json = unsafe {
            ffi::scard_slot_manager::ctk_slot_manager_slot_names_json(self.raw, &mut error_ptr)
        };
        if json.is_null() && !error_ptr.is_null() {
            return Err(from_swift(ffi::status::FRAMEWORK_ERROR, error_ptr));
        }
        if json.is_null() {
            return Ok(Vec::new());
        }
        decode_json(json)
    }

    /// Wraps the corresponding `TKSmartCardSlotManager` operation.
    pub fn slot_named(&self, name: &str) -> Result<Option<SmartCardSlot>, CryptoTokenKitError> {
        self.get_slot_impl(name, false)
    }

    /// Returns the corresponding `TKSmartCardSlotManager` value via the reply-based framework entry point.
    pub fn get_slot_with_name(
        &self,
        name: &str,
    ) -> Result<Option<SmartCardSlot>, CryptoTokenKitError> {
        self.get_slot_impl(name, true)
    }

    fn get_slot_impl(
        &self,
        name: &str,
        use_reply_api: bool,
    ) -> Result<Option<SmartCardSlot>, CryptoTokenKitError> {
        let name = to_cstring(name)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            if use_reply_api {
                ffi::scard_slot_manager::ctk_slot_manager_get_slot_with_name(
                    self.raw,
                    name.as_ptr(),
                    &mut raw,
                    &mut error_ptr,
                )
            } else {
                ffi::scard_slot_manager::ctk_slot_manager_slot_named(
                    self.raw,
                    name.as_ptr(),
                    &mut raw,
                    &mut error_ptr,
                )
            }
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then_some(SmartCardSlot { raw }))
    }
}

impl SmartCardSlot {
    /// Wraps the corresponding `TKSmartCardSlot` operation.
    pub fn name(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = unsafe { ffi::scard_slot_manager::ctk_slot_name(self.raw) };
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null slot name".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCardSlot` operation.
    pub fn max_input_length(&self) -> isize {
        unsafe { ffi::scard_slot_manager::ctk_slot_max_input_length(self.raw) }
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCardSlot` operation.
    pub fn max_output_length(&self) -> isize {
        unsafe { ffi::scard_slot_manager::ctk_slot_max_output_length(self.raw) }
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCardSlot` operation.
    pub fn state(&self) -> SlotState {
        SlotState::from_raw(unsafe { ffi::scard_slot_manager::ctk_slot_state(self.raw) })
    }

    /// Wraps the corresponding `TKSmartCardSlot` operation.
    pub fn atr(&self) -> Result<Option<SmartCardAtr>, CryptoTokenKitError> {
        let ptr = unsafe { ffi::scard_slot_manager::ctk_slot_atr_json(self.raw) };
        decode_optional_json(ptr)
    }

    #[must_use]
    /// Creates a value by calling the corresponding `TKSmartCardSlot` operation.
    pub fn make_smart_card(&self) -> Option<SmartCard> {
        let raw = unsafe { ffi::scard_slot_manager::ctk_slot_make_smart_card(self.raw) };
        (!raw.is_null()).then_some(SmartCard::from_raw(raw))
    }

    /// Starts observing the corresponding `TKSmartCardSlot` state changes.
    pub fn observe_state<D>(&self, delegate: D) -> Result<SlotStateObserver, CryptoTokenKitError>
    where
        D: SlotStateDelegate + 'static,
    {
        let callback_state = Box::new(CallbackState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(callback_state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::scard_slot_manager::ctk_slot_observe_state(
                self.raw,
                Some(slot_state_trampoline),
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null slot observer handle".into(),
            ));
        }

        Ok(SlotStateObserver {
            raw,
            _callback_state: callback_state,
        })
    }
}

impl Drop for SlotStateObserver {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Drop for SmartCardSlotManager {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

impl Drop for SmartCardSlot {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}
