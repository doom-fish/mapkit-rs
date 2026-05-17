use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_point_annotation_new_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_point_annotation_state_json(
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_point_annotation_apply_json(
        annotation: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_point_annotation_release(annotation: *mut c_void);

    pub fn mk_map_item_annotation_new_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_map_item_annotation_state_json(
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_item_annotation_release(annotation: *mut c_void);

    pub fn mk_user_location_state_json(
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_user_location_release(annotation: *mut c_void);
}
