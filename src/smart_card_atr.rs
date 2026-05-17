use core::ffi::c_void;

use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::private::{decode_optional_json, encode_json_cstring};

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
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_ber_tlv_record_json(tag, value.as_ptr(), value.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn ber_tag_data(tag: u64) -> Option<Vec<u8>> {
        let ptr = unsafe { ffi::smart_card_atr::ctk_ber_tlv_tag_data_json(tag) };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn ber_constructed(tag: u64, records: &[Self]) -> Option<Self> {
        let payload = encode_json_cstring(records).ok()?;
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_ber_tlv_record_with_records_json(tag, payload.as_ptr())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn simple(tag: u8, value: &[u8]) -> Option<Self> {
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_simple_tlv_record_json(tag, value.as_ptr(), value.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn compact(tag: u8, value: &[u8]) -> Option<Self> {
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_compact_tlv_record_json(tag, value.as_ptr(), value.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    pub fn parse(data: &[u8]) -> Option<Self> {
        Self::parse_with_encoding(TlvEncoding::Ber, data)
            .or_else(|| Self::parse_with_encoding(TlvEncoding::Simple, data))
            .or_else(|| Self::parse_with_encoding(TlvEncoding::Compact, data))
    }

    pub fn parse_with_encoding(encoding: TlvEncoding, data: &[u8]) -> Option<Self> {
        let (record, consumed) = parse_tlv_record_prefix(encoding, data)?;
        (consumed == data.len()).then_some(record)
    }

    pub fn parse_sequence(data: &[u8]) -> Option<Vec<Self>> {
        parse_tlv_sequence_with_fallback(data)
    }

    pub fn parse_sequence_with_encoding(encoding: TlvEncoding, data: &[u8]) -> Option<Vec<Self>> {
        let mut remaining = data;
        let mut records = Vec::new();
        while !remaining.is_empty() {
            let (record, consumed) = parse_tlv_record_prefix(encoding, remaining)?;
            records.push(record);
            remaining = &remaining[consumed..];
        }
        Some(records)
    }
}

fn parse_tlv_sequence_with_fallback(data: &[u8]) -> Option<Vec<TlvRecord>> {
    let mut remaining = data;
    let mut records = Vec::new();
    while !remaining.is_empty() {
        let (record, consumed) = parse_tlv_record_prefix(TlvEncoding::Ber, remaining)
            .or_else(|| parse_tlv_record_prefix(TlvEncoding::Simple, remaining))
            .or_else(|| parse_tlv_record_prefix(TlvEncoding::Compact, remaining))?;
        records.push(record);
        remaining = &remaining[consumed..];
    }
    Some(records)
}

fn parse_tlv_record_prefix(encoding: TlvEncoding, data: &[u8]) -> Option<(TlvRecord, usize)> {
    match encoding {
        TlvEncoding::Ber => parse_ber_record_prefix(data),
        TlvEncoding::Simple => parse_simple_record_prefix(data),
        TlvEncoding::Compact => parse_compact_record_prefix(data),
    }
}

fn parse_compact_record_prefix(data: &[u8]) -> Option<(TlvRecord, usize)> {
    let header = *data.first()?;
    let len = usize::from(header & 0x0F);
    let total = 1 + len;
    if data.len() < total {
        return None;
    }
    let tag = u64::from(header >> 4);
    let value = &data[1..total];
    u8::try_from(tag)
        .ok()
        .and_then(|tag| TlvRecord::compact(tag, value).map(|record| (record, total)))
}

fn parse_simple_record_prefix(data: &[u8]) -> Option<(TlvRecord, usize)> {
    let tag = *data.first()?;
    if tag == 0 || tag == 0xFF {
        return None;
    }
    let length_marker = *data.get(1)?;
    let (header_len, value_len) = if length_marker == 0xFF {
        let high = *data.get(2)?;
        let low = *data.get(3)?;
        (4, usize::from(u16::from_be_bytes([high, low])))
    } else {
        (2, usize::from(length_marker))
    };
    let total = header_len + value_len;
    if data.len() < total {
        return None;
    }
    let value = &data[header_len..total];
    TlvRecord::simple(tag, value).map(|record| (record, total))
}

fn parse_ber_record_prefix(data: &[u8]) -> Option<(TlvRecord, usize)> {
    let first = *data.first()?;
    let mut tag_len = 1usize;
    if first & 0x1F == 0x1F {
        loop {
            let byte = *data.get(tag_len)?;
            tag_len += 1;
            if byte & 0x80 == 0 {
                break;
            }
            if tag_len > 8 {
                return None;
            }
        }
    }
    let tag_bytes = &data[..tag_len];
    let mut tag = 0u64;
    for byte in tag_bytes {
        tag = (tag << 8) | u64::from(*byte);
    }

    let length_byte = *data.get(tag_len)?;
    let (length_len, value_len) = if length_byte & 0x80 == 0 {
        (1, usize::from(length_byte))
    } else {
        let count = usize::from(length_byte & 0x7F);
        if count == 0 || count > 8 {
            return None;
        }
        let length_bytes = data.get(tag_len + 1..tag_len + 1 + count)?;
        let mut value_len = 0usize;
        for byte in length_bytes {
            value_len = (value_len << 8) | usize::from(*byte);
        }
        (1 + count, value_len)
    };

    let total = tag_len + length_len + value_len;
    if data.len() < total {
        return None;
    }
    let value = &data[tag_len + length_len..total];
    TlvRecord::ber(tag, value).map(|record| (record, total))
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
