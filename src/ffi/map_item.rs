use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_map_item_string_constant(
        kind: i32,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_item_request_new(
        map_item_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_map_item_request_state_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_item_request_get_map_item_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_item_request_cancel(request: *mut c_void);
    pub fn mk_map_item_request_release(request: *mut c_void);
}
