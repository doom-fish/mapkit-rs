use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_annotation_callout_info_did_change_notification(
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_annotation_view_new(
        annotation: *mut c_void,
        reuse_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_annotation_view_state_json(
        annotation_view: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_annotation_view_apply_options_json(
        annotation_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_annotation_view_prepare_for_reuse(
        annotation_view: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_annotation_view_prepare_for_display(
        annotation_view: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_annotation_view_release(annotation_view: *mut c_void);
    pub fn mk_marker_annotation_view_new(
        annotation: *mut c_void,
        reuse_identifier: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_marker_annotation_view_state_json(
        annotation_view: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_marker_annotation_view_apply_options_json(
        annotation_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_marker_annotation_view_release(annotation_view: *mut c_void);
}
