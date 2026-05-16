use core::ffi::{c_char, c_void};

pub type TokenSessionBeginAuthCallback = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        i32,
        *const c_char,
        *mut *mut c_void,
        *mut *mut c_char,
    ) -> i32,
>;

pub type TokenSessionSupportsCallback = Option<
    unsafe extern "C" fn(*mut c_void, *mut c_void, i32, *const c_char, *mut c_void) -> bool,
>;

pub type TokenSessionDataCallback = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *const u8,
        usize,
        *const c_char,
        *mut c_void,
        *mut *mut c_char,
        *mut *mut c_char,
    ) -> i32,
>;

pub type TokenSessionKeyExchangeCallback = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *const u8,
        usize,
        *const c_char,
        *mut c_void,
        *mut c_void,
        *mut *mut c_char,
        *mut *mut c_char,
    ) -> i32,
>;

pub type TokenCreateSessionCallback = Option<
    unsafe extern "C" fn(*mut c_void, *mut c_void, *mut *mut c_void, *mut *mut c_char) -> i32,
>;

pub type TokenTerminateSessionCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void)>;

pub type TokenDriverCreateTokenCallback = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *const c_char,
        *mut *mut c_void,
        *mut *mut c_char,
    ) -> i32,
>;

pub type TokenDriverTerminateTokenCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void)>;

pub type SmartCardTokenDriverCreateTokenCallback = Option<
    unsafe extern "C" fn(
        *mut c_void,
        *mut c_void,
        *mut c_void,
        *const u8,
        usize,
        bool,
        *mut *mut c_void,
        *mut *mut c_char,
    ) -> i32,
>;

pub type SmartCardTokenDriverTerminateTokenCallback =
    Option<unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_void)>;

unsafe extern "C" {
    pub fn ctk_token_session_set_delegate(
        session: *mut c_void,
        begin_auth_callback: TokenSessionBeginAuthCallback,
        supports_callback: TokenSessionSupportsCallback,
        sign_callback: TokenSessionDataCallback,
        decrypt_callback: TokenSessionDataCallback,
        key_exchange_callback: TokenSessionKeyExchangeCallback,
        user_info: *mut c_void,
        out_delegate: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_session_clear_delegate(session: *mut c_void);
    pub fn ctk_token_session_has_delegate(session: *mut c_void) -> bool;
    pub fn ctk_token_session_token(session: *mut c_void) -> *mut c_void;
    pub fn ctk_token_session_invoke_delegate_begin_auth(
        session: *mut c_void,
        operation: i32,
        constraint_json: *const c_char,
        out_operation: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_session_invoke_delegate_supports(
        session: *mut c_void,
        operation: i32,
        object_id: *const c_char,
        base_algorithm: *const c_char,
        supported_algorithms_json: *const c_char,
    ) -> bool;
    pub fn ctk_token_session_invoke_delegate_sign(
        session: *mut c_void,
        operation: i32,
        request_ptr: *const u8,
        request_len: usize,
        object_id: *const c_char,
        base_algorithm: *const c_char,
        supported_algorithms_json: *const c_char,
        out_reply_json: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_session_invoke_delegate_decrypt(
        session: *mut c_void,
        operation: i32,
        request_ptr: *const u8,
        request_len: usize,
        object_id: *const c_char,
        base_algorithm: *const c_char,
        supported_algorithms_json: *const c_char,
        out_reply_json: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_session_invoke_delegate_key_exchange(
        session: *mut c_void,
        public_key_ptr: *const u8,
        public_key_len: usize,
        object_id: *const c_char,
        base_algorithm: *const c_char,
        supported_algorithms_json: *const c_char,
        requested_size: isize,
        shared_info_ptr: *const u8,
        shared_info_len: usize,
        has_shared_info: bool,
        out_reply_json: *mut *mut c_char,
        error_out: *mut *mut c_char,
    ) -> i32;

    pub fn ctk_token_set_delegate(
        token: *mut c_void,
        create_session_callback: TokenCreateSessionCallback,
        terminate_session_callback: TokenTerminateSessionCallback,
        user_info: *mut c_void,
        out_delegate: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_clear_delegate(token: *mut c_void);
    pub fn ctk_token_has_delegate(token: *mut c_void) -> bool;
    pub fn ctk_token_token_driver(token: *mut c_void) -> *mut c_void;
    pub fn ctk_token_invoke_delegate_create_session(
        token: *mut c_void,
        out_session: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_invoke_delegate_terminate_session(token: *mut c_void, session: *mut c_void);

    pub fn ctk_token_driver_set_delegate(
        driver: *mut c_void,
        create_token_callback: TokenDriverCreateTokenCallback,
        terminate_token_callback: TokenDriverTerminateTokenCallback,
        user_info: *mut c_void,
        out_delegate: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_driver_clear_delegate(driver: *mut c_void);
    pub fn ctk_token_driver_has_delegate(driver: *mut c_void) -> bool;
    pub fn ctk_token_driver_add_token_configuration_json(
        class_id: *const c_char,
        instance_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn ctk_token_driver_remove_token_configuration(
        class_id: *const c_char,
        instance_id: *const c_char,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_driver_invoke_delegate_token_for_configuration_json(
        driver: *mut c_void,
        configuration_json: *const c_char,
        out_token: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_token_driver_invoke_delegate_terminate_token(driver: *mut c_void, token: *mut c_void);

    pub fn ctk_smart_card_token_driver_set_delegate(
        driver: *mut c_void,
        create_token_callback: SmartCardTokenDriverCreateTokenCallback,
        terminate_token_callback: SmartCardTokenDriverTerminateTokenCallback,
        user_info: *mut c_void,
        out_delegate: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_token_driver_invoke_delegate_create_token(
        driver: *mut c_void,
        smart_card: *mut c_void,
        aid_ptr: *const u8,
        aid_len: usize,
        has_aid: bool,
        out_token: *mut *mut c_void,
        error_out: *mut *mut c_char,
    ) -> i32;
    pub fn ctk_smart_card_token_driver_invoke_delegate_terminate_token(
        driver: *mut c_void,
        token: *mut c_void,
    );

    pub fn ctk_token_key_algorithm_is_algorithm(
        algorithm: *mut c_void,
        algorithm_name: *const c_char,
    ) -> bool;
    pub fn ctk_token_key_algorithm_supports_algorithm(
        algorithm: *mut c_void,
        algorithm_name: *const c_char,
    ) -> bool;
    pub fn ctk_token_key_exchange_parameters_requested_size(parameters: *mut c_void) -> isize;
    pub fn ctk_token_key_exchange_parameters_shared_info_json(parameters: *mut c_void) -> *mut c_char;
    pub fn ctk_token_auth_operation_kind(operation: *mut c_void) -> i32;
}
