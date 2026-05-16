use core::ffi::{c_char, c_double};

extern "C" {
    pub fn mk_map_camera_boundary_from_map_rect_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_camera_boundary_from_region_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_camera_zoom_default() -> c_double;
}
