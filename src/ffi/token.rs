use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ctk_token_new(
        driver: *mut c_void,
        instance_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn ctk_smart_card_token_new(
        smart_card: *mut c_void,
        aid_ptr: *const u8,
        aid_len: usize,
        has_aid: bool,
        instance_id: *const c_char,
        driver: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn ctk_smart_card_token_aid_json(token: *mut c_void) -> *mut c_char;
    pub fn ctk_token_configuration_json(
        token: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ctk_token_set_configuration_data(
        token: *mut c_void,
        data_ptr: *const u8,
        data_len: usize,
        has_data: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_set_keychain_items_json(
        token: *mut c_void,
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_key_for_object_id_json(
        token: *mut c_void,
        object_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ctk_token_certificate_for_object_id_json(
        token: *mut c_void,
        object_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ctk_token_keychain_contents_items_json(
        token: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
