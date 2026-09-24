use core::ffi::c_void;
use std::ops::Deref;
use std::ptr;
use std::sync::{Mutex, PoisonError};

use doom_fish_utils::callback_context::CallbackContext;

use crate::error::CryptoTokenKitError;
use crate::ffi;
use crate::private::{
    checked, decode_optional_json, encode_json_cstring, status_result, to_cstring,
};
use crate::scard_slot_manager::SmartCardSlot;
use crate::smart_card::{
    SmartCard, SmartCardPinCompletion, SmartCardPinConfirmation, SmartCardPinFormat,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
/// Mirrors delegate events emitted by `TKSmartCardUserInteraction`.
pub enum SmartCardUserInteractionEvent {
    /// Variant bridged from `TKSmartCardUserInteraction`.
    CharacterEntered = 0,
    /// Variant bridged from `TKSmartCardUserInteraction`.
    CorrectionKeyPressed = 1,
    /// Variant bridged from `TKSmartCardUserInteraction`.
    ValidationKeyPressed = 2,
    /// Variant bridged from `TKSmartCardUserInteraction`.
    InvalidCharacterEntered = 3,
    /// Variant bridged from `TKSmartCardUserInteraction`.
    OldPinRequested = 4,
    /// Variant bridged from `TKSmartCardUserInteraction`.
    NewPinRequested = 5,
    /// Variant bridged from `TKSmartCardUserInteraction`.
    NewPinConfirmationRequested = 6,
}

impl SmartCardUserInteractionEvent {
    #[must_use]
    /// Wraps the corresponding `TKSmartCardUserInteraction` operation.
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

/// Rust delegate bridge for `TKSmartCardUserInteractionDelegate`.
pub trait SmartCardUserInteractionDelegate: Send {
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn character_entered(&mut self, _interaction: &SmartCardUserInteraction) {}
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn correction_key_pressed(&mut self, _interaction: &SmartCardUserInteraction) {}
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn validation_key_pressed(&mut self, _interaction: &SmartCardUserInteraction) {}
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn invalid_character_entered(&mut self, _interaction: &SmartCardUserInteraction) {}
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn old_pin_requested(&mut self, _interaction: &SmartCardUserInteraction) {}
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn new_pin_requested(&mut self, _interaction: &SmartCardUserInteraction) {}
    /// Handles the corresponding `TKSmartCardUserInteractionDelegate` callback.
    fn new_pin_confirmation_requested(&mut self, _interaction: &SmartCardUserInteraction) {}
}

type SmartCardUserInteractionDelegateCell = Mutex<Box<dyn SmartCardUserInteractionDelegate>>;

/// Lifetime token for a bridged `TKSmartCardUserInteraction` delegate.
pub struct SmartCardUserInteractionDelegateHandle {
    raw: *mut c_void,
    context: CallbackContext<SmartCardUserInteractionDelegateCell>,
}

impl Drop for SmartCardUserInteractionDelegateHandle {
    fn drop(&mut self) {
        self.context.deactivate();
        if !self.raw.is_null() {
            unsafe { ffi::ctk_object_release(self.raw) };
            self.raw = ptr::null_mut();
        }
    }
}

/// Wraps `TKSmartCardUserInteraction`.
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

    /// Sets the corresponding `TKSmartCardUserInteraction` value.
    pub fn set_delegate<D>(
        &self,
        delegate: D,
    ) -> Result<SmartCardUserInteractionDelegateHandle, CryptoTokenKitError>
    where
        D: SmartCardUserInteractionDelegate + 'static,
    {
        let cell: SmartCardUserInteractionDelegateCell = Mutex::new(Box::new(delegate));
        let context = CallbackContext::new(cell);
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_set_delegate(
                self.raw,
                Some(smart_card_user_interaction_trampoline),
                context.retained_ptr(),
                Some(CallbackContext::<SmartCardUserInteractionDelegateCell>::RELEASE),
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card user-interaction delegate handle".into(),
            ));
        }
        Ok(SmartCardUserInteractionDelegateHandle { raw, context })
    }

    /// Returns whether `TKSmartCardUserInteraction` currently has the associated bridge state.
    pub fn has_delegate(&self) -> Result<bool, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_has_delegate(
                self.raw, error,
            )
        })
    }

    /// Clears the corresponding `TKSmartCardUserInteraction` bridge state.
    pub fn clear_delegate(&self) -> Result<(), CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_clear_delegate(
                self.raw, error,
            );
        })
    }

    /// Wraps the corresponding `TKSmartCardUserInteraction` operation.
    pub fn simulate_delegate_event(
        &self,
        event: SmartCardUserInteractionEvent,
    ) -> Result<(), CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_emit_delegate_event(
                self.raw,
                event as i32,
                error,
            );
        })
    }

    /// Wraps the corresponding `TKSmartCardUserInteraction` operation.
    pub fn initial_timeout(&self) -> Result<f64, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_initial_timeout(
                self.raw, error,
            )
        })
    }

    /// Sets the corresponding `TKSmartCardUserInteraction` value.
    pub fn set_initial_timeout(&self, timeout: f64) -> Result<(), CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_set_initial_timeout(
                self.raw, timeout, error,
            );
        })
    }

    /// Wraps the corresponding `TKSmartCardUserInteraction` operation.
    pub fn interaction_timeout(&self) -> Result<f64, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_interaction_timeout(
                self.raw, error,
            )
        })
    }

    /// Sets the corresponding `TKSmartCardUserInteraction` value.
    pub fn set_interaction_timeout(&self, timeout: f64) -> Result<(), CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_set_interaction_timeout(
                self.raw, timeout, error,
            );
        })
    }

    /// Invokes the corresponding `TKSmartCardUserInteraction` operation.
    pub fn run(&self) -> Result<(), CryptoTokenKitError> {
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_run(
                self.raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)
    }

    /// Invokes the corresponding `TKSmartCardUserInteraction` operation.
    pub fn cancel(&self) -> Result<bool, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_cancel(self.raw, error)
        })
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

/// Wraps `TKSmartCardUserInteractionForPINOperation`.
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

    /// Wraps the corresponding `TKSmartCardUserInteractionForPINOperation` operation.
    pub fn pin_completion(&self) -> Result<SmartCardPinCompletion, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_completion(
                self.inner.raw(),
                error,
            )
        })
        .map(SmartCardPinCompletion)
    }

    /// Sets the corresponding `TKSmartCardUserInteractionForPINOperation` value.
    pub fn set_pin_completion(
        &self,
        completion: SmartCardPinCompletion,
    ) -> Result<(), CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_completion(
                self.inner.raw(),
                completion.bits(),
                error,
            );
        })
    }

    /// Wraps the corresponding `TKSmartCardUserInteractionForPINOperation` operation.
    pub fn pin_message_indices(&self) -> Result<Option<Vec<i64>>, CryptoTokenKitError> {
        let ptr = checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_message_indices_json(
                self.inner.raw(),
                error,
            )
        })?;
        decode_optional_json(ptr)
    }

    /// Sets the corresponding `TKSmartCardUserInteractionForPINOperation` value.
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
                    &raw mut error_ptr,
                )
            }
        } else {
            unsafe {
                ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_message_indices_json(
                    self.inner.raw(),
                    ptr::null(),
                    false,
                    &raw mut error_ptr,
                )
            }
        };
        status_result(status, error_ptr)
    }

    /// Wraps the corresponding `TKSmartCardUserInteractionForPINOperation` operation.
    pub fn locale_identifier(&self) -> Result<String, CryptoTokenKitError> {
        let ptr = checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_locale_identifier(
                self.inner.raw(),
                error,
            )
        })?;
        if ptr.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null locale identifier".into(),
            ));
        }
        Ok(crate::error::take_owned_c_string(ptr))
    }

    /// Sets the corresponding `TKSmartCardUserInteractionForPINOperation` value.
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
                    &raw mut error_ptr,
                )
            }
        } else {
            unsafe {
                ffi::smart_card_interaction::ctk_smart_card_pin_interaction_set_locale_identifier(
                    self.inner.raw(),
                    ptr::null(),
                    false,
                    &raw mut error_ptr,
                )
            }
        };
        status_result(status, error_ptr)
    }

    /// Wraps the corresponding `TKSmartCardUserInteractionForPINOperation` operation.
    pub fn result_status_word(&self) -> Result<u16, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_result_sw(
                self.inner.raw(),
                error,
            )
        })
    }

    /// Wraps the corresponding `TKSmartCardUserInteractionForPINOperation` operation.
    pub fn result_data(&self) -> Result<Option<Vec<u8>>, CryptoTokenKitError> {
        let ptr = checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_interaction_result_data_json(
                self.inner.raw(),
                error,
            )
        })?;
        decode_optional_json(ptr)
    }
}

impl Deref for SmartCardUserInteractionForPinOperation {
    type Target = SmartCardUserInteraction;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Wraps `TKSmartCardUserInteractionForSecurePINVerification`.
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

/// Wraps `TKSmartCardUserInteractionForSecurePINChange`.
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

    /// Wraps the corresponding `TKSmartCardUserInteractionForSecurePINChange` operation.
    pub fn pin_confirmation(&self) -> Result<SmartCardPinConfirmation, CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_change_interaction_confirmation(
                self.inner.raw(),
                error,
            )
        })
        .map(SmartCardPinConfirmation)
    }

    /// Sets the corresponding `TKSmartCardUserInteractionForSecurePINChange` value.
    pub fn set_pin_confirmation(
        &self,
        confirmation: SmartCardPinConfirmation,
    ) -> Result<(), CryptoTokenKitError> {
        checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_pin_change_interaction_set_confirmation(
                self.inner.raw(),
                confirmation.bits(),
                error,
            );
        })
    }
}

impl Deref for SmartCardUserInteractionForSecurePinChange {
    type Target = SmartCardUserInteractionForPinOperation;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe extern "C" fn smart_card_user_interaction_trampoline(
    user_info: *mut c_void,
    interaction_raw: *mut c_void,
    event_raw: i32,
) {
    let interaction = SmartCardUserInteraction::from_raw(interaction_raw);
    if interaction_raw.is_null() {
        return;
    }

    let dispatch = |delegate: &SmartCardUserInteractionDelegateCell| {
        let mut delegate = delegate.lock().unwrap_or_else(PoisonError::into_inner);
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
    };
    unsafe {
        CallbackContext::<SmartCardUserInteractionDelegateCell>::with(
            user_info,
            "SmartCardUserInteractionDelegate",
            dispatch,
        )
    };
}

impl SmartCard {
    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn slot(&self) -> Result<SmartCardSlot, CryptoTokenKitError> {
        let raw = checked(|error| unsafe {
            ffi::smart_card_interaction::ctk_smart_card_slot(self.raw(), error)
        })?;
        if raw.is_null() {
            return Err(CryptoTokenKitError::FrameworkError(
                "Swift bridge returned a null smart-card slot".into(),
            ));
        }
        Ok(SmartCardSlot::from_raw(raw))
    }

    /// Wraps the corresponding `TKSmartCard` operation.
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

    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn user_interaction_for_secure_pin_verification(
        &self,
        pin_format: &SmartCardPinFormat,
        apdu: &[u8],
        pin_byte_offset: isize,
    ) -> Result<Option<SmartCardUserInteractionForSecurePinVerification>, CryptoTokenKitError> {
        let pin_format = encode_json_cstring(pin_format)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_for_secure_pin_verification(
                self.raw(),
                pin_format.as_ptr(),
                apdu.as_ptr(),
                apdu.len(),
                pin_byte_offset,
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null())
            .then(|| SmartCardUserInteractionForSecurePinVerification::from_raw(raw)))
    }

    /// Wraps the corresponding `TKSmartCard` operation.
    pub fn user_interaction_for_secure_pin_change(
        &self,
        pin_format: &SmartCardPinFormat,
        apdu: &[u8],
        current_pin_byte_offset: isize,
        new_pin_byte_offset: isize,
    ) -> Result<Option<SmartCardUserInteractionForSecurePinChange>, CryptoTokenKitError> {
        let pin_format = encode_json_cstring(pin_format)?;
        let mut raw = ptr::null_mut();
        let mut error_ptr = ptr::null_mut();
        let status = unsafe {
            ffi::smart_card_interaction::ctk_smart_card_user_interaction_for_secure_pin_change(
                self.raw(),
                pin_format.as_ptr(),
                apdu.as_ptr(),
                apdu.len(),
                current_pin_byte_offset,
                new_pin_byte_offset,
                &raw mut raw,
                &raw mut error_ptr,
            )
        };
        status_result(status, error_ptr)?;
        Ok((!raw.is_null()).then(|| SmartCardUserInteractionForSecurePinChange::from_raw(raw)))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        SmartCardUserInteractionForPinOperation, SmartCardUserInteractionForSecurePinChange,
    };
    use crate::private::test_support::{assert_wrong_handle, retained};
    use crate::smart_card::{SmartCard, SmartCardPinConfirmation, SmartCardPinFormat};

    #[test]
    fn interaction_handles_of_another_class_are_rejected() {
        let card = SmartCard::mock("Handle Type Reader").expect("mock card");
        let verification = card
            .user_interaction_for_secure_pin_verification(
                &SmartCardPinFormat::default(),
                &[0x00, 0x20, 0x00, 0x00],
                0,
            )
            .expect("verification")
            .expect("mock verification");

        let change =
            SmartCardUserInteractionForSecurePinChange::from_raw(retained(verification.raw()));
        assert_wrong_handle(change.pin_confirmation());
        assert_wrong_handle(change.set_pin_confirmation(SmartCardPinConfirmation::CURRENT));
        change
            .pin_completion()
            .expect("a verification interaction is still a PIN operation");

        let from_card = SmartCardUserInteractionForPinOperation::from_raw(retained(card.raw()));
        assert_wrong_handle(from_card.pin_completion());
        assert_wrong_handle(from_card.set_locale_identifier(Some("en-US")));
        assert_wrong_handle(from_card.initial_timeout());
        assert_wrong_handle(from_card.has_delegate());
        assert_wrong_handle(from_card.run());

        let card_from_interaction = SmartCard::from_raw(retained(verification.raw()));
        assert_wrong_handle(card_from_interaction.slot());
        assert_wrong_handle(
            card_from_interaction.user_interaction_for_secure_pin_change(
                &SmartCardPinFormat::default(),
                &[0x00, 0x24, 0x00, 0x00],
                0,
                8,
            ),
        );
    }
}
