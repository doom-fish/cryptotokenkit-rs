use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ctk_token_driver_new() -> *mut c_void;
    pub fn ctk_smart_card_token_driver_new() -> *mut c_void;
    pub fn ctk_driver_configurations_json() -> *mut c_char;
}
