use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_user_tracking_button_state_json(
        map_view: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_user_tracking_button_set_visible(
        map_view: *mut c_void,
        visible: bool,
        out_error: *mut *mut c_char,
    );
    pub fn mk_user_tracking_button_set_tracking_mode(
        map_view: *mut c_void,
        mode_json: *const c_char,
        animated: bool,
        out_error: *mut *mut c_char,
    );
}
