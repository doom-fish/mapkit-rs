use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_geocoding_request_new(
        address_string: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_geocoding_request_state_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_geocoding_request_set_region_json(
        request: *mut c_void,
        region_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_geocoding_request_set_preferred_locale(
        request: *mut c_void,
        locale_identifier: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_geocoding_request_get_map_items_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_geocoding_request_cancel(request: *mut c_void);
    pub fn mk_geocoding_request_release(request: *mut c_void);

    pub fn mk_reverse_geocoding_request_new_json(
        location_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_reverse_geocoding_request_state_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_reverse_geocoding_request_set_preferred_locale(
        request: *mut c_void,
        locale_identifier: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_reverse_geocoding_request_get_map_items_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_reverse_geocoding_request_cancel(request: *mut c_void);
    pub fn mk_reverse_geocoding_request_release(request: *mut c_void);
}
