use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_cluster_annotation_new(
        annotations: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_cluster_annotation_state_json(
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_cluster_annotation_apply_json(
        annotation: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_cluster_annotation_release(annotation: *mut c_void);
}
