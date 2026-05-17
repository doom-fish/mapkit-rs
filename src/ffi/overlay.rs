use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_circle_new_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_circle_state_json(circle: *mut c_void, out_error: *mut *mut c_char) -> *mut c_char;
    pub fn mk_circle_release(circle: *mut c_void);

    pub fn mk_polyline_new_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_polyline_state_json(
        polyline: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_polyline_release(polyline: *mut c_void);

    pub fn mk_geodesic_polyline_new_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_geodesic_polyline_state_json(
        polyline: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_geodesic_polyline_release(polyline: *mut c_void);

    pub fn mk_polygon_new_json(
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_polygon_state_json(polygon: *mut c_void, out_error: *mut *mut c_char) -> *mut c_char;
    pub fn mk_polygon_release(polygon: *mut c_void);

    pub fn mk_multi_polyline_new(
        polylines: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_multi_polyline_state_json(
        overlay: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_multi_polyline_release(overlay: *mut c_void);

    pub fn mk_multi_polygon_new(
        polygons: *const *mut c_void,
        count: usize,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_multi_polygon_state_json(
        overlay: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_multi_polygon_release(overlay: *mut c_void);

    pub fn mk_tile_overlay_new(
        url_template: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_tile_overlay_state_json(
        overlay: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_tile_overlay_apply_options_json(
        overlay: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_tile_overlay_url_for_tile_path_json(
        overlay: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_tile_overlay_release(overlay: *mut c_void);
}
