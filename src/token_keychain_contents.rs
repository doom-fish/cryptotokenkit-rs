use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Wraps the object identifier used by `TKToken.ObjectID`.
pub struct TokenObjectId(
    /// Serialized field bridged from `TKToken.ObjectID`.
    pub String,
);

impl TokenObjectId {
    #[must_use]
    /// Creates a new wrapper around `TKToken.ObjectID`.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl From<&str> for TokenObjectId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for TokenObjectId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Mirrors the `TKTokenOperation` values used by `CryptoTokenKit`.
pub enum TokenOperation {
    /// Variant bridged from `TKTokenOperation`.
    None,
    /// Variant bridged from `TKTokenOperation`.
    ReadData,
    /// Variant bridged from `TKTokenOperation`.
    SignData,
    /// Variant bridged from `TKTokenOperation`.
    DecryptData,
    /// Variant bridged from `TKTokenOperation`.
    PerformKeyExchange,
}

impl TokenOperation {
    #[must_use]
    /// Wraps the corresponding `TKTokenOperation` operation.
    pub const fn raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::ReadData => 1,
            Self::SignData => 2,
            Self::DecryptData => 3,
            Self::PerformKeyExchange => 4,
        }
    }

    #[must_use]
    /// Wraps the corresponding `TKTokenOperation` operation.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::ReadData,
            2 => Self::SignData,
            3 => Self::DecryptData,
            4 => Self::PerformKeyExchange,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of `TKTokenKeychainItem` metadata.
pub struct TokenKeychainItem {
    /// Serialized field bridged from `TKTokenKeychainItem`.
    pub object_id: TokenObjectId,
    /// Serialized field bridged from `TKTokenKeychainItem`.
    pub label: Option<String>,
    #[serde(default)]
    /// Serialized field bridged from `TKTokenKeychainItem`.
    pub constraints: BTreeMap<TokenOperation, Value>,
}

impl TokenKeychainItem {
    #[must_use]
    /// Creates a new wrapper around `TKTokenKeychainItem`.
    pub fn new(object_id: impl Into<TokenObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
            label: None,
            constraints: BTreeMap::new(),
        }
    }

    #[must_use]
    /// Runs the corresponding `TKTokenKeychainItem` workflow around the provided callback.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    /// Runs the corresponding `TKTokenKeychainItem` workflow around the provided callback.
    pub fn with_constraint(mut self, operation: TokenOperation, value: Value) -> Self {
        self.constraints.insert(operation, value);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of `TKTokenKeychainCertificate`.
pub struct TokenKeychainCertificate {
    /// Serialized field bridged from `TKTokenKeychainCertificate`.
    pub item: TokenKeychainItem,
    #[serde(with = "serde_bytes")]
    /// Serialized field bridged from `TKTokenKeychainCertificate`.
    pub data: Vec<u8>,
}

impl TokenKeychainCertificate {
    #[must_use]
    /// Creates a new wrapper around `TKTokenKeychainCertificate`.
    pub fn new(object_id: impl Into<TokenObjectId>, data: Vec<u8>) -> Self {
        Self {
            item: TokenKeychainItem::new(object_id),
            data,
        }
    }

    #[must_use]
    /// Runs the corresponding `TKTokenKeychainCertificate` workflow around the provided callback.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.item.label = Some(label.into());
        self
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Snapshot of the capability flags carried by `TKTokenKeychainKey`.
pub struct TokenKeyCapabilities {
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub can_decrypt: bool,
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub can_sign: bool,
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub can_perform_key_exchange: bool,
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub suitable_for_login: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of `TKTokenKeychainKey`.
pub struct TokenKeychainKey {
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub item: TokenKeychainItem,
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub key_type: String,
    #[serde(default, with = "serde_bytes")]
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub application_tag: Option<Vec<u8>>,
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub key_size_in_bits: i64,
    #[serde(default, with = "serde_bytes")]
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub public_key_data: Option<Vec<u8>>,
    #[serde(default, with = "serde_bytes")]
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub public_key_hash: Option<Vec<u8>>,
    /// Serialized field bridged from `TKTokenKeychainKey`.
    pub capabilities: TokenKeyCapabilities,
}

impl TokenKeychainKey {
    #[must_use]
    /// Creates a new wrapper around `TKTokenKeychainKey`.
    pub fn new(object_id: impl Into<TokenObjectId>, key_type: impl Into<String>) -> Self {
        Self {
            item: TokenKeychainItem::new(object_id),
            key_type: key_type.into(),
            application_tag: None,
            key_size_in_bits: 0,
            public_key_data: None,
            public_key_hash: None,
            capabilities: TokenKeyCapabilities::default(),
        }
    }

    #[must_use]
    /// Runs the corresponding `TKTokenKeychainKey` workflow around the provided callback.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.item.label = Some(label.into());
        self
    }

    #[must_use]
    /// Runs the corresponding `TKTokenKeychainKey` workflow around the provided callback.
    pub fn with_key_size_in_bits(mut self, bits: i64) -> Self {
        self.key_size_in_bits = bits;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
/// Enum wrapper over the entry types stored in `TKTokenKeychainContents`.
pub enum TokenKeychainEntry {
    /// Variant bridged from `TKTokenKeychainContents`.
    Item(TokenKeychainItem),
    /// Variant bridged from `TKTokenKeychainContents`.
    Certificate(TokenKeychainCertificate),
    /// Variant bridged from `TKTokenKeychainContents`.
    Key(TokenKeychainKey),
}

impl TokenKeychainEntry {
    #[must_use]
    /// Wraps the corresponding `TKTokenKeychainContents` operation.
    pub fn object_id(&self) -> &TokenObjectId {
        match self {
            Self::Item(item) => &item.object_id,
            Self::Certificate(certificate) => &certificate.item.object_id,
            Self::Key(key) => &key.item.object_id,
        }
    }
}

impl From<TokenKeychainItem> for TokenKeychainEntry {
    fn from(value: TokenKeychainItem) -> Self {
        Self::Item(value)
    }
}

impl From<TokenKeychainCertificate> for TokenKeychainEntry {
    fn from(value: TokenKeychainCertificate) -> Self {
        Self::Certificate(value)
    }
}

impl From<TokenKeychainKey> for TokenKeychainEntry {
    fn from(value: TokenKeychainKey) -> Self {
        Self::Key(value)
    }
}
