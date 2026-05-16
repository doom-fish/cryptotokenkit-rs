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

pub mod error;
pub mod ffi;
pub mod keychain;
pub mod scard_slot_manager;
pub mod smart_card;
pub mod smart_card_atr;
pub mod smart_card_interaction;
pub mod smartcard;
mod private;
pub mod token;
pub mod token_delegate;
pub mod token_driver;
pub mod token_keychain_contents;
pub mod token_session;
pub mod token_watcher;

pub use error::{CryptoTokenKitError, TKErrorCode, TK_ERROR_DOMAIN};
pub use scard_slot_manager::{
    SlotState, SlotStateCallbacks, SlotStateDelegate, SlotStateObserver, SmartCardSlot,
    SmartCardSlotManager,
};
pub use smart_card::{
    ApduResponse, SmartCard, SmartCardPinCharset, SmartCardPinCompletion,
    SmartCardPinConfirmation, SmartCardPinEncoding, SmartCardPinFormat,
    SmartCardPinJustification,
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
        SmartCardTokenDriverDelegate, SmartCardTokenDriverDelegateHandle,
        TokenAuthOperationHandle, TokenDelegate, TokenDelegateHandle, TokenDriverDelegate,
        TokenDriverDelegateHandle, TokenKeyAlgorithm, TokenKeyExchangeParameters,
        TokenSessionDelegate, TokenSessionDelegateHandle,
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
