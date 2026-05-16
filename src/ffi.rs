use core::ffi::{c_char, c_void};

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
}

pub type SlotStateCallback = Option<unsafe extern "C" fn(*mut c_void, i32)>;

unsafe extern "C" {
    pub fn ctk_object_release(ptr: *mut c_void);

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
    pub fn ctk_slot_make_smart_card(slot: *mut c_void) -> *mut c_void;
    pub fn ctk_slot_observe_state(
        slot: *mut c_void,
        callback: SlotStateCallback,
        user_info: *mut c_void,
        out_observer: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;

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
