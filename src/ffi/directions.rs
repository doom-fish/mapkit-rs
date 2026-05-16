use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_directions_new(
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_directions_calculate_json(
        directions: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_directions_calculate_eta_json(
        directions: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_directions_cancel(directions: *mut c_void);
    pub fn mk_directions_is_calculating(directions: *mut c_void) -> bool;
    pub fn mk_directions_release(directions: *mut c_void);
}
