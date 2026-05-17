use core::ffi::c_void;
use std::ops::{Deref, DerefMut};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::sync::Mutex;

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::{decode_optional_json, encode_json_cstring, status_result, to_cstring};
use crate::scard_slot_manager::SmartCardSlot;
use crate::smart_card::{
    SmartCard, SmartCardPinCompletion, SmartCardPinConfirmation, SmartCardPinFormat,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SmartCardUserInteractionEvent {
    CharacterEntered = 0,
    CorrectionKeyPressed = 1,
    ValidationKeyPressed = 2,
    InvalidCharacterEntered = 3,
    OldPinRequested = 4,
    NewPinRequested = 5,
    NewPinConfirmationRequested = 6,
}

impl SmartCardUserInteractionEvent {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::CorrectionKeyPressed,
            2 => Self::ValidationKeyPressed,
            3 => Self::InvalidCharacterEntered,
            4 => Self::OldPinRequested,
            5 => Self::NewPinRequested,
            6 => Self::NewPinConfirmationRequested,
            _ => Self::CharacterEntered,
        }
    }
}

pub trait SmartCardUserInteractionDelegate: Send {
    fn character_entered(&mut self, _interaction: &SmartCardUserInteraction) {}
    fn correction_key_pressed(&mut self, _interaction: &SmartCardUserInteraction) {}
    fn validation_key_pressed(&mut self, _interaction: &SmartCardUserInteraction) {}
    fn invalid_character_entered(&mut self, _interaction: &SmartCardUserInteraction) {}
    fn old_pin_requested(&mut self, _interaction: &SmartCardUserInteraction) {}
    fn new_pin_requested(&mut self, _interaction: &SmartCardUserInteraction) {}
    fn new_pin_confirmation_requested(&mut self, _interaction: &SmartCardUserInteraction) {}
}

struct SmartCardUserInteractionDelegateState {
    delegate: Mutex<Box<dyn SmartCardUserInteractionDelegate>>,
}

pub struct SmartCardUserInteractionDelegateHandle {
    raw: *mut c_void,
    _state: Box<SmartCardUserInteractionDelegateState>,
}

impl Drop for SmartCardUserInteractionDelegateHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

pub struct SmartCardUserInteraction {
    raw: *mut c_void,
}

impl SmartCardUserInteraction {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self { raw }
    }

    #[must_use]
    fn raw(&self) -> *mut c_void {
        self.raw
    }

    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<SmartCardUserInteractionDelegateHandle, CryptoTokenKitError>
    where
        D: SmartCardUserInteractionDelegate + 'static,
    {
        let state = Box::new(SmartCardUserInteractionDelegateState {
            delegate: Mutex::new(Box::new(delegate)),
        });
        let user_info = std::ptr::from_ref(state.as_ref())
            .cast_mut()
            .cast::<c_void>();
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_set_delegate(
                self.raw,
                Some(smart_card_user_interaction_trampoline),
                user_info,
                &mut raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card user-interaction delegate handle".into(),
            ));
        }
        Ok(SmartCardUserInteractionDelegateHandle { raw, _state: state })
    }

    #[must_use]
    pub fn has_delegate(&self) -> bool {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_has_delegate(self.raw)
        }
    }

    pub fn clear_delegate(&self) {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_clear_delegate(self.raw);
        };
    }

    pub fn simulate_delegate_event(&self, event: SmartCardUserInteractionEvent) {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_emit_delegate_event(
                self.raw,
                event as i32,
            );
        };
    }

    #[must_use]
    pub fn initial_timeout(&self) -> f64 {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_initial_timeout(self.raw)
        }
    }

    pub fn set_initial_timeout(&self, timeout: f64) {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_set_initial_timeout(
                self.raw, timeout,
            );
        };
    }

    #[must_use]
    pub fn interaction_timeout(&self) -> f64 {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_interaction_timeout(
                self.raw,
            )
        }
    }

    pub fn set_interaction_timeout(&self, timeout: f64) {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_set_interaction_timeout(
                self.raw, timeout,
            );
        };
    }

    pub fn run(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_run(
                self.raw,
                &mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    #[must_use]
    pub fn cancel(&self) -> bool {
        unsafe { ffi::smart_card_interaction::ctk_smart_card_user_interaction_cancel(self.raw) }
    }
}

impl Drop for SmartCardUserInteraction {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

pub struct SmartCardUserInteractionForPinOperation {
    inner: SmartCardUserInteraction,
}

impl SmartCardUserInteractionForPinOperation {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self {
            inner: SmartCardUserInteraction::from_raw(raw),
        }
    }

    #[must_use]
    pub fn pin_completion(&self) -> SmartCardPinCompletion {
        SmartCardPinCompletion(unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_completion(self.inner.raw())
        })
    }

    pub fn set_pin_completion(&self, completion: SmartCardPinCompletion) {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_completion(
                self.inner.raw(),
                completion.bits(),
            );
        };
    }

    pub fn pin_message_indices(&self) -> Result<Option<Vec<i64>>, CryptoTokenKitError> {
        let ptr = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_message_indices_json(
                self.inner.raw(),
            )
        };
        decode_optional_json(ptr)
    }

    pub fn set_pin_message_indices(
        &self,
        message_indices: Option<&[i64]>,
    ) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = if let Some(message_indices) = message_indices {
            let payload = encode_json_cstring(message_indices)?;
            unsafe {
                ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_message_indices_json(
                    self.inner.raw(),
                    payload.as_ptr(),
                    true,
                    &mut error_ptr,
                )
            }
        } else {
            unsafe {
                ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_message_indices_json(
                    self.inner.raw(),
                    ptr::null(),
                    false,
                    &mut error_ptr,
                )
            }
        };
        status_result(status, error_ptr)
    }

    pub fn locale_identifier(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_locale_identifier(
                self.inner.raw(),
            )
        };
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null locale identifier".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    pub fn set_locale_identifier(
        &self,
        identifier: Option<&str>,
    ) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = if let Some(identifier) = identifier {
            let identifier = to_cstring(identifier)?;
            unsafe {
                ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_locale_identifier(
                    self.inner.raw(),
                    identifier.as_ptr(),
                    true,
                    &mut error_ptr,
                )
            }
        } else {
            unsafe {
                ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_locale_identifier(
                    self.inner.raw(),
                    ptr::null(),
                    false,
                    &mut error_ptr,
                )
            }
        };
        status_result(status, error_ptr)
    }

    #[must_use]
    pub fn result_status_word(&self) -> u16 {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_result_sw(self.inner.raw())
        }
    }

    pub fn result_data(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        let ptr = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_result_data_json(
                self.inner.raw(),
            )
        };
        decode_optional_json(ptr)
    }
}

impl Deref for SmartCardUserInteractionForPinOperation {
    type Target = SmartCardUserInteraction;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SmartCardUserInteractionForPinOperation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct SmartCardUserInteractionForSecurePinVerification {
    inner: SmartCardUserInteractionForPinOperation,
}

impl SmartCardUserInteractionForSecurePinVerification {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self {
            inner: SmartCardUserInteractionForPinOperation::from_raw(raw),
        }
    }
}

impl Deref for SmartCardUserInteractionForSecurePinVerification {
    type Target = SmartCardUserInteractionForPinOperation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SmartCardUserInteractionForSecurePinVerification {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub struct SmartCardUserInteractionForSecurePinChange {
    inner: SmartCardUserInteractionForPinOperation,
}

impl SmartCardUserInteractionForSecurePinChange {
    #[must_use]
    pub(crate) const fn from_raw(raw: *mut c_void) -> Self {
        Self {
            inner: SmartCardUserInteractionForPinOperation::from_raw(raw),
        }
    }

    #[must_use]
    pub fn pin_confirmation(&self) -> SmartCardPinConfirmation {
        SmartCardPinConfirmation(unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_change_interaction_confirmation(
                self.inner.raw(),
            )
        })
    }

    pub fn set_pin_confirmation(&self, confirmation: SmartCardPinConfirmation) {
        unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_change_interaction_set_confirmation(
                self.inner.raw(),
                confirmation.bits(),
            );
        };
    }
}

impl Deref for SmartCardUserInteractionForSecurePinChange {
    type Target = SmartCardUserInteractionForPinOperation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SmartCardUserInteractionForSecurePinChange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

unsafe extern "C" fn smart_card_user_interaction_trampoline(
    user_info: *mut c_void,
    interaction_raw: *mut c_void,
    event_raw: i32,
) {
    if user_info.is_null() || interaction_raw.is_null() {
        return;
    }

    let _ = catch_unwind(AssertUnwindSafe(|| {
        let state = unsafe { &*user_info.cast::<SmartCardUserInteractionDelegateState>() };
        let interaction = SmartCardUserInteraction::from_raw(interaction_raw);
        let mut delegate = match state.delegate.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        match SmartCardUserInteractionEvent::from_raw(event_raw) {
            SmartCardUserInteractionEvent::CharacterEntered => {
                delegate.character_entered(&interaction);
            }
            SmartCardUserInteractionEvent::CorrectionKeyPressed => {
                delegate.correction_key_pressed(&interaction);
            }
            SmartCardUserInteractionEvent::ValidationKeyPressed => {
                delegate.validation_key_pressed(&interaction);
            }
            SmartCardUserInteractionEvent::InvalidCharacterEntered => {
                delegate.invalid_character_entered(&interaction);
            }
            SmartCardUserInteractionEvent::OldPinRequested => {
                delegate.old_pin_requested(&interaction);
            }
            SmartCardUserInteractionEvent::NewPinRequested => {
                delegate.new_pin_requested(&interaction);
            }
            SmartCardUserInteractionEvent::NewPinConfirmationRequested => {
                delegate.new_pin_confirmation_requested(&interaction);
            }
        }
    }));
}

impl SmartCard {
    pub fn slot(&self) -> Result<SmartCardSlot, CryptoTokenKitError> {
        let raw = unsafe { ffi::smart_card_interaction::ctk_smart_card_slot(self.raw()) };
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card slot".into(),
            ));
        }
        Ok(SmartCardSlot::from_raw(raw))
    }

    pub fn mock(slot_name: &str) -> Result<Self, CryptoTokenKitError> {
        let slot_name = to_cstring(slot_name)?;
        let raw =
            unsafe { ffi::smart_card_interaction::ctk_mock_smart_card_new(slot_name.as_ptr()) };
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null mock smart-card".into(),
            ));
        }
        Ok(Self::from_raw(raw))
    }

    pub fn user_interaction_for_secure_pin_verification(
        &self,
        pin_format: &SmartCardPinFormat,
        apdu: &[u8],
        pin_byte_offset: isize,
    ) -> Result<Option<SmartCardUserInteractionForSecurePinVerification>, CryptoTokenKitError> {
        let pin_format = encode_json_cstring(pin_format)?;
        let mut error_ptr = ptr::null_mut();
        let raw = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_for_secure_pin_verification(
                self.raw(),
                pin_format.as_ptr(),
                apdu.as_ptr(),
                apdu.len(),
                pin_byte_offset,
                &mut error_ptr,
            )
        };
        if raw.is_null() && !error_ptr.is_null() {
            return Err(crate::error::from_swift(
                ffi::status::FRAMEWORK_ERROR,
                error_ptr,
            ));
        }
        Ok((!raw.is_null())
            .then(|| SmartCardUserInteractionForSecurePinVerification::from_raw(raw)))
    }

    pub fn user_interaction_for_secure_pin_change(
        &self,
        pin_format: &SmartCardPinFormat,
        apdu: &[u8],
        current_pin_byte_offset: isize,
        new_pin_byte_offset: isize,
    ) -> Result<Option<SmartCardUserInteractionForSecurePinChange>, CryptoTokenKitError> {
        let pin_format = encode_json_cstring(pin_format)?;
        let mut error_ptr = ptr::null_mut();
        let raw = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_for_secure_pin_change(
                self.raw(),
                pin_format.as_ptr(),
                apdu.as_ptr(),
                apdu.len(),
                current_pin_byte_offset,
                new_pin_byte_offset,
                &mut error_ptr,
            )
        };
        if raw.is_null() && !error_ptr.is_null() {
            return Err(crate::error::from_swift(
                ffi::status::FRAMEWORK_ERROR,
                error_ptr,
            ));
        }
        Ok((!raw.is_null()).then(|| SmartCardUserInteractionForSecurePinChange::from_raw(raw)))
    }
}
