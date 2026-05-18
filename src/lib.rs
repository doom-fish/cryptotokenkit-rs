#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [CryptoTokenKit](https://developer.apple.com/documentation/cryptotokenkit)
//! framework.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default,
    clippy::unsafe_derive_deserialize
)]

/// Error-domain and status-code wrappers for `CryptoTokenKit` failures.
pub mod error;
#[doc(hidden)]
pub mod ffi;
/// Compatibility re-exports for `TKTokenKeychainContents` model wrappers.
pub mod keychain;
mod private;
/// Wrappers for `TKSmartCardSlotManager` and `TKSmartCardSlot`.
pub mod scard_slot_manager;
/// Wrappers for `TKSmartCard` and related PIN-format types.
pub mod smart_card;
/// Wrappers for `TKSmartCardATR`, `TKSmartCardProtocol`, and TLV helpers.
pub mod smart_card_atr;
/// Wrappers for `TKSmartCardUserInteraction` and secure-PIN subclasses.
pub mod smart_card_interaction;
/// Compatibility re-exports for the `TKSmartCard*` wrapper modules.
pub mod smartcard;
/// Wrappers for `TKToken` and `TKSmartCardToken`.
pub mod token;
/// Delegate bridges for `TKTokenSessionDelegate`, `TKTokenDelegate`, and driver delegates.
pub mod token_delegate;
/// Wrappers for `TKTokenDriver` and `TKSmartCardTokenDriver`.
pub mod token_driver;
/// Snapshot models for `TKTokenKeychainContents`.
pub mod token_keychain_contents;
/// Wrappers for `TKTokenSession` and auth-operation types.
pub mod token_session;
/// Wrappers for `TKTokenWatcher`.
pub mod token_watcher;

pub use error::{CryptoTokenKitError, TKErrorCode, TK_ERROR_DOMAIN};
pub use scard_slot_manager::{
    SlotState, SlotStateCallbacks, SlotStateDelegate, SlotStateObserver, SmartCardSlot,
    SmartCardSlotManager,
};
pub use smart_card::{
    ApduResponse, SmartCard, SmartCardPinCharset, SmartCardPinCompletion, SmartCardPinConfirmation,
    SmartCardPinEncoding, SmartCardPinFormat, SmartCardPinJustification,
};
pub use smart_card_atr::{
    SmartCardAtr, SmartCardAtrInterfaceGroup, SmartCardProtocol, TlvEncoding, TlvRecord,
};
pub use smart_card_interaction::{
    SmartCardUserInteraction, SmartCardUserInteractionDelegate,
    SmartCardUserInteractionDelegateHandle, SmartCardUserInteractionEvent,
    SmartCardUserInteractionForPinOperation, SmartCardUserInteractionForSecurePinChange,
    SmartCardUserInteractionForSecurePinVerification,
};
pub use token::{SmartCardToken, Token, TokenConfigurationSnapshot};
pub use token_delegate::{
    SmartCardTokenDriverDelegate, SmartCardTokenDriverDelegateHandle, TokenAuthOperationHandle,
    TokenDelegate, TokenDelegateHandle, TokenDriverDelegate, TokenDriverDelegateHandle,
    TokenKeyAlgorithm, TokenKeyExchangeParameters, TokenSessionDelegate,
    TokenSessionDelegateHandle,
};
pub use token_driver::{SmartCardTokenDriver, TokenDriver, TokenDriverConfigurationSnapshot};
pub use token_keychain_contents::{
    TokenKeyCapabilities, TokenKeychainCertificate, TokenKeychainEntry, TokenKeychainItem,
    TokenKeychainKey, TokenObjectId, TokenOperation,
};
pub use token_session::{
    SmartCardTokenSession, TokenAuthOperation, TokenPasswordAuthOperation, TokenSession,
    TokenSmartCardPinAuthOperation,
};
pub use token_watcher::{TokenWatcher, TokenWatcherTokenInfo};

/// Common imports.
pub mod prelude {
    pub use crate::error::{CryptoTokenKitError, TKErrorCode, TK_ERROR_DOMAIN};
    pub use crate::scard_slot_manager::{
        SlotState, SlotStateCallbacks, SlotStateDelegate, SlotStateObserver, SmartCardSlot,
        SmartCardSlotManager,
    };
    pub use crate::smart_card::{
        ApduResponse, SmartCard, SmartCardPinCharset, SmartCardPinCompletion,
        SmartCardPinConfirmation, SmartCardPinEncoding, SmartCardPinFormat,
        SmartCardPinJustification,
    };
    pub use crate::smart_card_atr::{
        SmartCardAtr, SmartCardAtrInterfaceGroup, SmartCardProtocol, TlvEncoding, TlvRecord,
    };
    pub use crate::smart_card_interaction::{
        SmartCardUserInteraction, SmartCardUserInteractionDelegate,
        SmartCardUserInteractionDelegateHandle, SmartCardUserInteractionEvent,
        SmartCardUserInteractionForPinOperation, SmartCardUserInteractionForSecurePinChange,
        SmartCardUserInteractionForSecurePinVerification,
    };
    pub use crate::token::{SmartCardToken, Token, TokenConfigurationSnapshot};
    pub use crate::token_delegate::{
        SmartCardTokenDriverDelegate, SmartCardTokenDriverDelegateHandle, TokenAuthOperationHandle,
        TokenDelegate, TokenDelegateHandle, TokenDriverDelegate, TokenDriverDelegateHandle,
        TokenKeyAlgorithm, TokenKeyExchangeParameters, TokenSessionDelegate,
        TokenSessionDelegateHandle,
    };
    pub use crate::token_driver::{
        SmartCardTokenDriver, TokenDriver, TokenDriverConfigurationSnapshot,
    };
    pub use crate::token_keychain_contents::{
        TokenKeyCapabilities, TokenKeychainCertificate, TokenKeychainEntry, TokenKeychainItem,
        TokenKeychainKey, TokenObjectId, TokenOperation,
    };
    pub use crate::token_session::{
        SmartCardTokenSession, TokenAuthOperation, TokenPasswordAuthOperation, TokenSession,
        TokenSmartCardPinAuthOperation,
    };
    pub use crate::token_watcher::{TokenWatcher, TokenWatcherTokenInfo};
}
