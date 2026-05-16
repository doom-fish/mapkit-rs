use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_map_view_new(
        width: f64,
        height: f64,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_map_view_state_json(
        map_view: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_view_apply_options_json(
        map_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_region_that_fits_json(
        map_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_view_map_rect_that_fits_json(
        map_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_view_convert_coordinate_to_point_json(
        map_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_view_convert_point_to_coordinate_json(
        map_view: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_view_add_point_annotation(
        map_view: *mut c_void,
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_remove_point_annotation(
        map_view: *mut c_void,
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_add_cluster_annotation(
        map_view: *mut c_void,
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_remove_cluster_annotation(
        map_view: *mut c_void,
        annotation: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_add_circle(
        map_view: *mut c_void,
        circle: *mut c_void,
        level_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_remove_circle(
        map_view: *mut c_void,
        circle: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_add_polyline(
        map_view: *mut c_void,
        polyline: *mut c_void,
        level_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_remove_polyline(
        map_view: *mut c_void,
        polyline: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_add_polygon(
        map_view: *mut c_void,
        polygon: *mut c_void,
        level_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_remove_polygon(
        map_view: *mut c_void,
        polygon: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_map_view_release(map_view: *mut c_void);
}
