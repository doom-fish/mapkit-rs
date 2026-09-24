use core::ffi::{c_char, c_void};

use super::MKContextCallback;

pub type MKLocalSearchCompleterEventCallback =
    unsafe extern "C" fn(*mut c_void, *mut c_void, *mut c_char, *mut c_char);

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
    pub fn mk_local_search_completer_set_delegate(
        completer: *mut c_void,
        context: *mut c_void,
        retain_context: MKContextCallback,
        release_context: MKContextCallback,
        on_event: MKLocalSearchCompleterEventCallback,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_local_search_completer_clear_delegate(completer: *mut c_void, proxy: *mut c_void);
    pub fn mk_local_search_completer_cancel(completer: *mut c_void);
    pub fn mk_local_search_completer_release(completer: *mut c_void);
}
