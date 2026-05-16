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
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::new_without_default
)]

pub mod error;
pub mod ffi;
pub mod keychain;
mod private;
pub mod smartcard;

pub use error::CryptoTokenKitError;
pub use keychain::{
    TokenKeyCapabilities, TokenKeychainCertificate, TokenKeychainItem, TokenKeychainKey,
    TokenObjectId, TokenOperation,
};
pub use smartcard::{
    ApduResponse, SlotState, SlotStateCallbacks, SlotStateDelegate, SlotStateObserver, SmartCard,
    SmartCardSlot, SmartCardSlotManager,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::CryptoTokenKitError;
    pub use crate::keychain::{
        TokenKeyCapabilities, TokenKeychainCertificate, TokenKeychainItem, TokenKeychainKey,
        TokenObjectId, TokenOperation,
    };
    pub use crate::smartcard::{
        ApduResponse, SlotState, SlotStateCallbacks, SlotStateDelegate, SlotStateObserver,
        SmartCard, SmartCardSlot, SmartCardSlotManager,
    };
}
