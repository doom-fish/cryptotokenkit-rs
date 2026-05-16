use core::ffi::{c_char, c_void};

pub type TokenWatcherCallback = Option<unsafe extern "C" fn(*mut c_void, *const c_char)>;

unsafe extern "C" {
    pub fn ctk_token_watcher_new() -> *mut c_void;
    pub fn ctk_token_watcher_token_ids_json(
        watcher: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ctk_token_watcher_set_insertion_handler(
        watcher: *mut c_void,
        callback: TokenWatcherCallback,
        user_info: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_watcher_add_removal_handler(
        watcher: *mut c_void,
        token_id: *const c_char,
        callback: TokenWatcherCallback,
        user_info: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_watcher_token_info_json(
        watcher: *mut c_void,
        token_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}
