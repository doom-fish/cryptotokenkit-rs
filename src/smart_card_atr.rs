use core::ffi::c_void;

use doom_fish_utils::panic_safe::catch_user_panic_result;
use serde::{Deserialize, Serialize};

use crate::ffi;
use crate::private::{decode_optional_json, encode_json_cstring};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// Wraps the `TKSmartCardProtocol` bitset.
pub struct SmartCardProtocol(
    /// Serialized field bridged from `TKSmartCardProtocol`.
    pub u32,
);

impl SmartCardProtocol {
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const NONE: Self = Self(0);
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const T0: Self = Self(1 << 0);
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const T1: Self = Self(1 << 1);
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const T15: Self = Self(1 << 15);
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const ANY: Self = Self((1 << 16) - 1);

    #[must_use]
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const fn bits(self) -> u32 {
        self.0
    }

    #[must_use]
    /// Wraps the corresponding `TKSmartCardProtocol` operation.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Selects which `TKTLVRecord` encoding rules to use.
pub enum TlvEncoding {
    /// Variant bridged from `TKTLVRecord`.
    Ber,
    /// Variant bridged from `TKTLVRecord`.
    Simple,
    /// Variant bridged from `TKTLVRecord`.
    Compact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of a `TKTLVRecord` bridged through the Swift helper layer.
pub struct TlvRecord {
    /// Serialized field bridged from `TKTLVRecord`.
    pub encoding: TlvEncoding,
    /// Serialized field bridged from `TKTLVRecord`.
    pub tag: u64,
    #[serde(with = "serde_bytes")]
    /// Serialized field bridged from `TKTLVRecord`.
    pub value: Vec<u8>,
    #[serde(with = "serde_bytes")]
    /// Serialized field bridged from `TKTLVRecord`.
    pub data: Vec<u8>,
}

impl TlvRecord {
    /// Wraps the corresponding `TKTLVRecord` operation.
    pub fn ber(tag: u64, value: &[u8]) -> Option<Self> {
        if tag == 0 {
            return None;
        }
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_ber_tlv_record_json(tag, value.as_ptr(), value.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    /// Wraps the corresponding `TKTLVRecord` operation.
    pub fn ber_tag_data(tag: u64) -> Option<Vec<u8>> {
        if tag == 0 {
            return None;
        }
        let ptr = unsafe { ffi::smart_card_atr::ctk_ber_tlv_tag_data_json(tag) };
        decode_optional_json(ptr).ok().flatten()
    }

    /// Wraps the corresponding `TKTLVRecord` operation.
    pub fn ber_constructed(tag: u64, records: &[Self]) -> Option<Self> {
        if tag == 0 || !records.iter().all(Self::is_constructible) {
            return None;
        }
        let payload = encode_json_cstring(records).ok()?;
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_ber_tlv_record_with_records_json(tag, payload.as_ptr())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    /// Wraps the corresponding `TKTLVRecord` operation.
    pub fn simple(tag: u8, value: &[u8]) -> Option<Self> {
        if value.len() > MAX_SIMPLE_TLV_VALUE_LEN {
            return None;
        }
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_simple_tlv_record_json(tag, value.as_ptr(), value.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    /// Wraps the corresponding `TKTLVRecord` operation.
    pub fn compact(tag: u8, value: &[u8]) -> Option<Self> {
        if tag > MAX_COMPACT_TLV_TAG || value.len() > MAX_COMPACT_TLV_VALUE_LEN {
            return None;
        }
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_compact_tlv_record_json(tag, value.as_ptr(), value.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    fn is_constructible(&self) -> bool {
        match self.encoding {
            TlvEncoding::Ber => self.tag != 0,
            TlvEncoding::Simple => {
                u8::try_from(self.tag).is_ok() && self.value.len() <= MAX_SIMPLE_TLV_VALUE_LEN
            }
            TlvEncoding::Compact => {
                self.tag <= u64::from(MAX_COMPACT_TLV_TAG)
                    && self.value.len() <= MAX_COMPACT_TLV_VALUE_LEN
            }
        }
    }

    /// Parses data with the corresponding `TKTLVRecord` helper.
    pub fn parse(data: &[u8]) -> Option<Self> {
        Self::parse_with_encoding(TlvEncoding::Ber, data)
            .or_else(|| Self::parse_with_encoding(TlvEncoding::Simple, data))
            .or_else(|| Self::parse_with_encoding(TlvEncoding::Compact, data))
    }

    /// Parses data with the corresponding `TKTLVRecord` helper.
    pub fn parse_with_encoding(encoding: TlvEncoding, data: &[u8]) -> Option<Self> {
        let (record, consumed) = parse_tlv_record_prefix(encoding, data)?;
        (consumed == data.len()).then_some(record)
    }

    /// Parses data with the corresponding `TKTLVRecord` helper.
    pub fn parse_sequence(data: &[u8]) -> Option<Vec<Self>> {
        parse_tlv_sequence_with_fallback(data)
    }

    /// Parses data with the corresponding `TKTLVRecord` helper.
    pub fn parse_sequence_with_encoding(encoding: TlvEncoding, data: &[u8]) -> Option<Vec<Self>> {
        let mut remaining = data;
        let mut records = Vec::new();
        while !remaining.is_empty() {
            let (record, consumed) = parse_tlv_record_prefix(encoding, remaining)?;
            records.push(record);
            remaining = remaining.get(consumed..)?;
        }
        Some(records)
    }
}

const MAX_BER_TAG_BYTES: usize = 8;
const MAX_BER_LENGTH_BYTES: usize = 8;
const MAX_SIMPLE_TLV_VALUE_LEN: usize = 0xFFFF;
const MAX_COMPACT_TLV_TAG: u8 = 0x0F;
const MAX_COMPACT_TLV_VALUE_LEN: usize = 0x0F;

fn parse_tlv_sequence_with_fallback(data: &[u8]) -> Option<Vec<TlvRecord>> {
    let mut remaining = data;
    let mut records = Vec::new();
    while !remaining.is_empty() {
        let (record, consumed) = parse_tlv_record_prefix(TlvEncoding::Ber, remaining)
            .or_else(|| parse_tlv_record_prefix(TlvEncoding::Simple, remaining))
            .or_else(|| parse_tlv_record_prefix(TlvEncoding::Compact, remaining))?;
        records.push(record);
        remaining = remaining.get(consumed..)?;
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
    let total = 1 + usize::from(header & 0x0F);
    let value = data.get(1..total)?;
    TlvRecord::compact(header >> 4, value).map(|record| (record, total))
}

fn parse_simple_record_prefix(data: &[u8]) -> Option<(TlvRecord, usize)> {
    let tag = *data.first()?;
    if tag == 0 || tag == 0xFF {
        return None;
    }
    let length_marker = *data.get(1)?;
    let (header_len, value_len): (usize, usize) = if length_marker == 0xFF {
        let high = *data.get(2)?;
        let low = *data.get(3)?;
        (4, usize::from(u16::from_be_bytes([high, low])))
    } else {
        (2, usize::from(length_marker))
    };
    let total = header_len.checked_add(value_len)?;
    let value = data.get(header_len..total)?;
    TlvRecord::simple(tag, value).map(|record| (record, total))
}

fn parse_ber_record_prefix(data: &[u8]) -> Option<(TlvRecord, usize)> {
    let first = *data.first()?;
    let mut tag_len = 1usize;
    if first & 0x1F == 0x1F {
        loop {
            let byte = *data.get(tag_len)?;
            tag_len += 1;
            if tag_len > MAX_BER_TAG_BYTES {
                return None;
            }
            if byte & 0x80 == 0 {
                break;
            }
        }
    }
    let tag = data
        .get(..tag_len)?
        .iter()
        .fold(0u64, |tag, byte| (tag << 8) | u64::from(*byte));

    let length_byte = *data.get(tag_len)?;
    let (length_len, value_len) = if length_byte & 0x80 == 0 {
        (1, usize::from(length_byte))
    } else {
        let count = usize::from(length_byte & 0x7F);
        if count == 0 || count > MAX_BER_LENGTH_BYTES {
            return None;
        }
        let start = tag_len.checked_add(1)?;
        let length_bytes = data.get(start..start.checked_add(count)?)?;
        let value_len = length_bytes.iter().try_fold(0usize, |len, byte| {
            len.checked_mul(256)?.checked_add(usize::from(*byte))
        })?;
        (1 + count, value_len)
    };

    let header_len = tag_len.checked_add(length_len)?;
    let total = header_len.checked_add(value_len)?;
    let value = data.get(header_len..total)?;
    TlvRecord::ber(tag, value).map(|record| (record, total))
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of one `TKSmartCardATR` interface group.
pub struct SmartCardAtrInterfaceGroup {
    /// Serialized field bridged from `TKSmartCardATR`.
    pub index: i64,
    /// Serialized field bridged from `TKSmartCardATR`.
    pub ta: Option<u8>,
    /// Serialized field bridged from `TKSmartCardATR`.
    pub tb: Option<u8>,
    /// Serialized field bridged from `TKSmartCardATR`.
    pub tc: Option<u8>,
    /// Serialized field bridged from `TKSmartCardATR`.
    pub protocol: Option<SmartCardProtocol>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Snapshot of `TKSmartCardATR`.
pub struct SmartCardAtr {
    #[serde(with = "serde_bytes")]
    /// Serialized field bridged from `TKSmartCardATR`.
    pub bytes: Vec<u8>,
    /// Serialized field bridged from `TKSmartCardATR`.
    pub protocols: Vec<SmartCardProtocol>,
    /// Serialized field bridged from `TKSmartCardATR`.
    pub interface_groups: Vec<SmartCardAtrInterfaceGroup>,
    #[serde(with = "serde_bytes")]
    /// Serialized field bridged from `TKSmartCardATR`.
    pub historical_bytes: Vec<u8>,
    /// Serialized field bridged from `TKSmartCardATR`.
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

    // SAFETY: user_info is null-checked above and points to a valid AtrSourceState<F>
    // kept alive by the caller for the duration of the synchronous parse call.
    // The user-supplied closure may panic, so it runs inside catch_user_panic_result to
    // prevent unwinding across the FFI boundary (which would be undefined behavior).
    catch_user_panic_result("SmartCardAtr::parse_from_source", || {
        let state = unsafe { &mut *user_info.cast::<AtrSourceState<F>>() };
        (state.callback)().map_or(-1, i32::from)
    })
    .unwrap_or(-1)
}

impl SmartCardAtr {
    #[must_use]
    /// Parses data with the corresponding `TKSmartCardATR` helper.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let ptr = unsafe {
            ffi::smart_card_atr::ctk_smart_card_atr_parse_bytes_json(bytes.as_ptr(), bytes.len())
        };
        decode_optional_json(ptr).ok().flatten()
    }

    /// Parses data with the corresponding `TKSmartCardATR` helper.
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
