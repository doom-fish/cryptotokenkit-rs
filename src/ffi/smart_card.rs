use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ctk_smart_card_slot_name(card: *mut c_void) -> *mut c_char;
    pub fn ctk_smart_card_valid(card: *mut c_void) -> bool;
    pub fn ctk_smart_card_allowed_protocols(card: *mut c_void) -> u32;
    pub fn ctk_smart_card_set_allowed_protocols(card: *mut c_void, protocols: u32);
    pub fn ctk_smart_card_current_protocol(card: *mut c_void) -> u32;
    pub fn ctk_smart_card_sensitive(card: *mut c_void) -> bool;
    pub fn ctk_smart_card_set_sensitive(card: *mut c_void, sensitive: bool);
    pub fn ctk_smart_card_cla(card: *mut c_void) -> u8;
    pub fn ctk_smart_card_set_cla(card: *mut c_void, cla: u8);
    pub fn ctk_smart_card_use_extended_length(card: *mut c_void) -> bool;
    pub fn ctk_smart_card_set_use_extended_length(card: *mut c_void, enabled: bool);
    pub fn ctk_smart_card_use_command_chaining(card: *mut c_void) -> bool;
    pub fn ctk_smart_card_set_use_command_chaining(card: *mut c_void, enabled: bool);
    pub fn ctk_smart_card_context_json(card: *mut c_void) -> *mut c_char;
    pub fn ctk_smart_card_set_context_json(
        card: *mut c_void,
        json: *const c_char,
        has_json: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_begin_session(card: *mut c_void, error_out: *mut *mut c_char) -> i32;
    pub fn ctk_smart_card_transmit_request_json(
        card: *mut c_void,
        request_ptr: *const u8,
        request_len: usize,
        out_reply_json: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_end_session(card: *mut c_void);
    pub fn ctk_smart_card_send_ins(
        card: *mut c_void,
        ins: u8,
        p1: u8,
        p2: u8,
        data_ptr: *const u8,
        data_len: usize,
        has_le: bool,
        le: usize,
        out_reply_json: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
}
