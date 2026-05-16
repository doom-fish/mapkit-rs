use core::ffi::c_char;

extern "C" {
    pub fn mk_coordinate_region_make_with_distance_json(
        center_json: *const c_char,
        latitudinal_meters: f64,
        longitudinal_meters: f64,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_point_for_coordinate_json(
        coordinate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_coordinate_for_map_point_json(
        map_point_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_meters_between_map_points(
        first_map_point_json: *const c_char,
        second_map_point_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> f64;
}
