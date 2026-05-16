use core::ffi::{c_char, c_void};

pub type SmartCardUserInteractionEventCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, i32)>;

unsafe extern "C" {
    pub fn ctk_smart_card_slot(card: *mut c_void) -> *mut c_void;
    pub fn ctk_mock_smart_card_new(slot_name: *const c_char) -> *mut c_void;

    pub fn ctk_smart_card_user_interaction_set_delegate(
        interaction: *mut c_void,
        callback: SmartCardUserInteractionEventCallback,
        user_info: *mut c_void,
        out_delegate: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_user_interaction_clear_delegate(interaction: *mut c_void);
    pub fn ctk_smart_card_user_interaction_has_delegate(interaction: *mut c_void) -> bool;
    pub fn ctk_smart_card_user_interaction_emit_delegate_event(interaction: *mut c_void, event: i32);

    pub fn ctk_smart_card_user_interaction_initial_timeout(interaction: *mut c_void) -> f64;
    pub fn ctk_smart_card_user_interaction_set_initial_timeout(interaction: *mut c_void, timeout: f64);
    pub fn ctk_smart_card_user_interaction_interaction_timeout(interaction: *mut c_void) -> f64;
    pub fn ctk_smart_card_user_interaction_set_interaction_timeout(
        interaction: *mut c_void,
        timeout: f64,
    );
    pub fn ctk_smart_card_user_interaction_run(
        interaction: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_user_interaction_cancel(interaction: *mut c_void) -> bool;

    pub fn ctk_smart_card_pin_interaction_completion(interaction: *mut c_void) -> u32;
    pub fn ctk_smart_card_pin_interaction_set_completion(interaction: *mut c_void, completion: u32);
    pub fn ctk_smart_card_pin_interaction_message_indices_json(interaction: *mut c_void)
        -> *mut c_char;
    pub fn ctk_smart_card_pin_interaction_set_message_indices_json(
        interaction: *mut c_void,
        json: *const c_char,
        has_json: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_pin_interaction_locale_identifier(interaction: *mut c_void) -> *mut c_char;
    pub fn ctk_smart_card_pin_interaction_set_locale_identifier(
        interaction: *mut c_void,
        identifier: *const c_char,
        has_identifier: bool,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_pin_interaction_result_sw(interaction: *mut c_void) -> u16;
    pub fn ctk_smart_card_pin_interaction_result_data_json(interaction: *mut c_void) -> *mut c_char;

    pub fn ctk_smart_card_pin_change_interaction_confirmation(interaction: *mut c_void) -> u32;
    pub fn ctk_smart_card_pin_change_interaction_set_confirmation(
        interaction: *mut c_void,
        confirmation: u32,
    );

    pub fn ctk_smart_card_user_interaction_for_secure_pin_verification(
        card: *mut c_void,
        pin_format_json: *const c_char,
        apdu_ptr: *const u8,
        apdu_len: usize,
        pin_byte_offset: isize,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn ctk_smart_card_user_interaction_for_secure_pin_change(
        card: *mut c_void,
        pin_format_json: *const c_char,
        apdu_ptr: *const u8,
        apdu_len: usize,
        current_pin_byte_offset: isize,
        new_pin_byte_offset: isize,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
}
