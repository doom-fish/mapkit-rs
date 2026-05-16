use core::ffi::{c_char, c_void};

extern "C" {
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
