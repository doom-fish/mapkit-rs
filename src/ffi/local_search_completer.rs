use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_local_search_completer_new(out_error: *mut *mut c_char) -> *mut c_void;
    pub fn mk_local_search_completer_state_json(
        completer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_local_search_completer_apply_options_json(
        completer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_local_search_completer_refresh_json(
        completer: *mut c_void,
        timeout_millis: u64,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_local_search_completer_cancel(completer: *mut c_void);
    pub fn mk_local_search_completer_release(completer: *mut c_void);
}
