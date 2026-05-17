use core::ffi::{c_char, c_void};

pub type AtrSourceCallback = Option<unsafe extern "C" fn(*mut c_void) -> i32>;

unsafe extern "C" {
    pub fn ctk_smart_card_atr_parse_bytes_json(data_ptr: *const u8, data_len: usize)
        -> *mut c_char;
    pub fn ctk_smart_card_atr_parse_source_json(
        callback: AtrSourceCallback,
        user_info: *mut c_void,
    ) -> *mut c_char;
    pub fn ctk_ber_tlv_record_json(tag: u64, value_ptr: *const u8, value_len: usize)
        -> *mut c_char;
    pub fn ctk_simple_tlv_record_json(
        tag: u8,
        value_ptr: *const u8,
        value_len: usize,
    ) -> *mut c_char;
    pub fn ctk_compact_tlv_record_json(
        tag: u8,
        value_ptr: *const u8,
        value_len: usize,
    ) -> *mut c_char;
    pub fn ctk_ber_tlv_tag_data_json(tag: u64) -> *mut c_char;
    pub fn ctk_ber_tlv_record_with_records_json(
        tag: u64,
        records_json: *const c_char,
    ) -> *mut c_char;
}
