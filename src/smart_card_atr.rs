use core::ffi::c_void;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::private::decode_optional_json;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SmartCardProtocol(pub u32);

impl SmartCardProtocol {
    pub const NONE: Self = Self(0);
    pub const T0: Self = Self(1 << 0);
    pub const T1: Self = Self(1 << 1);
    pub const T15: Self = Self(1 << 15);
    pub const ANY: Self = Self((1 << 16) - 1);

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TlvEncoding {
    Ber,
    Simple,
    Compact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlvRecord {
    pub encoding: TlvEncoding,
    pub tag: u64,
    #[serde(with = "serde_bytes")]
    pub value: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl TlvRecord {
    pub fn ber(tag: u64, value: &[u8]) -> Option<Self> {
        let ptr = unsafe { ffi::smart_card_atr::ctk_ber_tlv_record_json(tag, value.as_ptr(), value.len()) };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn simple(tag: u8, value: &[u8]) -> Option<Self> {
        let ptr = unsafe { ffi::smart_card_atr::ctk_simple_tlv_record_json(tag, value.as_ptr(), value.len()) };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn compact(tag: u8, value: &[u8]) -> Option<Self> {
        let ptr = unsafe { ffi::smart_card_atr::ctk_compact_tlv_record_json(tag, value.as_ptr(), value.len()) };
        decode_optional_json(ptr).ok().flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartCardAtrInterfaceGroup {
    pub index: i64,
    pub ta: Option<u8>,
    pub tb: Option<u8>,
    pub tc: Option<u8>,
    pub protocol: Option<SmartCardProtocol>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartCardAtr {
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
    pub protocols: Vec<SmartCardProtocol>,
    pub interface_groups: Vec<SmartCardAtrInterfaceGroup>,
    #[serde(with = "serde_bytes")]
    pub historical_bytes: Vec<u8>,
    pub historical_records: Option<Vec<TlvRecord>>,
}

struct AtrSourceState<F>
where
    F: FnMut() -> Option<u8>,
{
    callback: F,
}

unsafe extern "C" fn atr_source_trampoline<F>(user_info: *mut c_void) -> i32
where
    F: FnMut() -> Option<u8>,
{
    if user_info.is_null() {
        return -1;
    }

    let state = unsafe { &mut *user_info.cast::<AtrSourceState<F>>() };
    (state.callback)().map_or(-1, i32::from)
}

impl SmartCardAtr {
    #[must_use]
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_smart_card_atr_parse_bytes_json(bytes.as_ptr(), bytes.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn parse_from_source<F>(callback: F) -> Option<Self>
    where
        F: FnMut() -> Option<u8>,
    {
        let mut state = Box::new(AtrSourceState { callback });
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_smart_card_atr_parse_source_json(
                Some(atr_source_trampoline::<F>),
                std::ptr::from_mut(state.as_mut()).cast::<c_void>(),
            )
        };
        decode_optional_json(ptr).ok().flatten()
    }
}
