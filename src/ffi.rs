#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_string_free(string: *mut c_char);

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

    pub fn mk_local_search_new(
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_local_search_start_json(
        search: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_local_search_cancel(search: *mut c_void);
    pub fn mk_local_search_is_searching(search: *mut c_void) -> bool;
    pub fn mk_local_search_release(search: *mut c_void);

    pub fn mk_directions_new(
        request_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_directions_calculate_json(
        directions: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_directions_calculate_eta_json(
        directions: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_directions_cancel(directions: *mut c_void);
    pub fn mk_directions_is_calculating(directions: *mut c_void) -> bool;
    pub fn mk_directions_release(directions: *mut c_void);

    pub fn mk_distance_formatter_new() -> *mut c_void;
    pub fn mk_distance_formatter_set_units(formatter: *mut c_void, raw_units: u64);
    pub fn mk_distance_formatter_set_unit_style(
        formatter: *mut c_void,
        raw_unit_style: u64,
    );
    pub fn mk_distance_formatter_string_from_distance(
        formatter: *mut c_void,
        distance: f64,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_distance_formatter_distance_from_string(
        formatter: *mut c_void,
        distance_string: *const c_char,
        out_error: *mut *mut c_char,
    ) -> f64;
    pub fn mk_distance_formatter_release(formatter: *mut c_void);
}
