use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_road_width_at_zoom_scale(zoom_scale: f64) -> f64;
    pub fn mk_overlay_renderer_new(
        overlay: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_overlay_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_overlay_renderer_apply_options_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_overlay_renderer_can_draw_map_rect_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> bool;
    pub fn mk_overlay_renderer_set_needs_display(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    );
    pub fn mk_overlay_renderer_set_needs_display_in_map_rect_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_overlay_renderer_set_needs_display_in_map_rect_zoom_scale_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_overlay_renderer_release(renderer: *mut c_void);
    pub fn mk_overlay_path_renderer_new(
        overlay: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_overlay_path_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_overlay_path_renderer_apply_options_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_overlay_path_renderer_release(renderer: *mut c_void);
    pub fn mk_circle_renderer_new(circle: *mut c_void, out_error: *mut *mut c_char) -> *mut c_void;
    pub fn mk_circle_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_circle_renderer_apply_options_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_circle_renderer_release(renderer: *mut c_void);
    pub fn mk_polyline_renderer_new(
        polyline: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_polyline_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_polyline_renderer_apply_options_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_polyline_renderer_release(renderer: *mut c_void);
    pub fn mk_gradient_polyline_renderer_new(
        polyline: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_gradient_polyline_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_gradient_polyline_renderer_apply_options_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_gradient_polyline_renderer_release(renderer: *mut c_void);
    pub fn mk_polygon_renderer_new(
        polygon: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_polygon_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_polygon_renderer_apply_options_json(
        renderer: *mut c_void,
        payload_json: *const c_char,
        out_error: *mut *mut c_char,
    );
    pub fn mk_polygon_renderer_release(renderer: *mut c_void);
    pub fn mk_tile_overlay_renderer_new(
        overlay: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_tile_overlay_renderer_state_json(
        renderer: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_tile_overlay_renderer_reload_data(renderer: *mut c_void, out_error: *mut *mut c_char);
    pub fn mk_tile_overlay_renderer_release(renderer: *mut c_void);
}
