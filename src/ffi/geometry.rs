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
    pub fn mk_geometry_constant_json(
        kind: i32,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_coordinate_region_for_map_rect_json(
        map_rect_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_points_per_meter_at_latitude(
        latitude: f64,
        out_error: *mut *mut c_char,
    ) -> f64;
    pub fn mk_meters_per_map_point_at_latitude(
        latitude: f64,
        out_error: *mut *mut c_char,
    ) -> f64;
    pub fn mk_map_rect_predicate_json(
        rect_json: *const c_char,
        auxiliary_json: *const c_char,
        kind: i32,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn mk_map_rect_transform_json(
        rect_json: *const c_char,
        other_rect_json: *const c_char,
        dx: f64,
        dy: f64,
        amount: f64,
        edge: i32,
        kind: i32,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
