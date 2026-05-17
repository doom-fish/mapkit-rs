use core::ffi::c_char;

extern "C" {
    pub fn mk_geojson_decode_json(
        data: *const u8,
        len: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
}
