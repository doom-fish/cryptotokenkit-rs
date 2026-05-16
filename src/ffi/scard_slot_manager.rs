use core::ffi::{c_char, c_void};

pub type SlotStateCallback = Option<unsafe extern "C" fn(*mut c_void, i32)>;

unsafe extern "C" {
    pub fn ctk_slot_manager_default() -> *mut c_void;
    pub fn ctk_slot_manager_slot_names_json(
        manager: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ctk_slot_manager_slot_named(
        manager: *mut c_void,
        name: *const c_char,
        out_slot: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_slot_manager_get_slot_with_name(
        manager: *mut c_void,
        name: *const c_char,
        out_slot: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn ctk_slot_name(slot: *mut c_void) -> *mut c_char;
    pub fn ctk_slot_max_input_length(slot: *mut c_void) -> isize;
    pub fn ctk_slot_max_output_length(slot: *mut c_void) -> isize;
    pub fn ctk_slot_state(slot: *mut c_void) -> i32;
    pub fn ctk_slot_atr_json(slot: *mut c_void) -> *mut c_char;
    pub fn ctk_slot_make_smart_card(slot: *mut c_void) -> *mut c_void;
    pub fn ctk_slot_observe_state(
        slot: *mut c_void,
        callback: SlotStateCallback,
        user_info: *mut c_void,
        out_observer: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
}
