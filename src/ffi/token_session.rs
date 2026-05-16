use core::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn ctk_token_session_new(token: *mut c_void) -> *mut c_void;
    pub fn ctk_smart_card_token_session_new(token: *mut c_void) -> *mut c_void;
    pub fn ctk_token_session_token_instance_id(session: *mut c_void) -> *mut c_char;
    pub fn ctk_smart_card_token_session_smart_card(session: *mut c_void) -> *mut c_void;
    pub fn ctk_smart_card_token_session_get_smart_card(
        session: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn ctk_token_auth_operation_new() -> *mut c_void;
    pub fn ctk_token_auth_operation_finish(
        operation: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn ctk_token_password_auth_operation_new() -> *mut c_void;
    pub fn ctk_token_password_auth_operation_password(operation: *mut c_void) -> *mut c_char;
    pub fn ctk_token_password_auth_operation_set_password(
        operation: *mut c_void,
        password: *const c_char,
        has_password: bool,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn ctk_token_smart_card_pin_auth_operation_new() -> *mut c_void;
    pub fn ctk_token_smart_card_pin_auth_operation_json(operation: *mut c_void) -> *mut c_char;
    pub fn ctk_token_smart_card_pin_auth_operation_update_json(
        operation: *mut c_void,
        json: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_smart_card_pin_auth_operation_set_smart_card(
        operation: *mut c_void,
        smart_card: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
}
