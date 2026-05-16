use core::ffi::c_void;

pub mod status {
    pub const OK: i32 = 0;
    pub const INVALID_ARGUMENT: i32 = -1;
    pub const FRAMEWORK_ERROR: i32 = -2;
    pub const TIMED_OUT: i32 = -3;
}

pub mod scard_slot_manager;
pub mod smart_card;
pub mod smart_card_atr;
pub mod smart_card_interaction;
pub mod token;
pub mod token_delegate;
pub mod token_driver;
pub mod token_session;
pub mod token_watcher;

unsafe extern "C" {
    pub fn ctk_object_release(ptr: *mut c_void);
}
