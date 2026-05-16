use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_local_search_new(
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_local_search_new_points_of_interest(
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_local_search_start_json(
        search: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_local_search_cancel(search: *mut c_void);
    pub fn mk_local_search_is_searching(search: *mut c_void) -> bool;
    pub fn mk_local_search_release(search: *mut c_void);
}
