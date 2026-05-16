use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TokenObjectId(pub String);

impl TokenObjectId {
    #[must_use]
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
pub enum TokenOperation {
    None,
    ReadData,
    SignData,
    DecryptData,
    PerformKeyExchange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenKeychainItem {
    pub object_id: TokenObjectId,
    pub label: Option<String>,
    #[serde(default)]
    pub constraints: BTreeMap<TokenOperation, Value>,
}

impl TokenKeychainItem {
    #[must_use]
    pub fn new(object_id: impl Into<TokenObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
            label: None,
            constraints: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn with_constraint(mut self, operation: TokenOperation, value: Value) -> Self {
        self.constraints.insert(operation, value);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenKeychainCertificate {
    pub item: TokenKeychainItem,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl TokenKeychainCertificate {
    #[must_use]
    pub fn new(object_id: impl Into<TokenObjectId>, data: Vec<u8>) -> Self {
        Self {
            item: TokenKeychainItem::new(object_id),
            data,
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.item.label = Some(label.into());
        self
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenKeyCapabilities {
    pub can_decrypt: bool,
    pub can_sign: bool,
    pub can_perform_key_exchange: bool,
    pub suitable_for_login: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenKeychainKey {
    pub item: TokenKeychainItem,
    pub key_type: String,
    #[serde(default, with = "serde_bytes")]
    pub application_tag: Option<Vec<u8>>,
    pub key_size_in_bits: i64,
    #[serde(default, with = "serde_bytes")]
    pub public_key_data: Option<Vec<u8>>,
    #[serde(default, with = "serde_bytes")]
    pub public_key_hash: Option<Vec<u8>>,
    pub capabilities: TokenKeyCapabilities,
}

impl TokenKeychainKey {
    #[must_use]
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
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.item.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn with_key_size_in_bits(mut self, bits: i64) -> Self {
        self.key_size_in_bits = bits;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum TokenKeychainEntry {
    Item(TokenKeychainItem),
    Certificate(TokenKeychainCertificate),
    Key(TokenKeychainKey),
}

impl TokenKeychainEntry {
    #[must_use]
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
