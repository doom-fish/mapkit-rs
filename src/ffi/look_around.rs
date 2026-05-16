use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_look_around_scene_request_new_coordinate_json(
        coordinate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_look_around_scene_request_new_map_item_json(
        map_item_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_look_around_scene_request_state_json(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_look_around_scene_request_get_scene(
        request: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_look_around_scene_request_cancel(request: *mut c_void);
    pub fn mk_look_around_scene_request_release(request: *mut c_void);

    pub fn mk_look_around_scene_release(scene: *mut c_void);

    pub fn mk_look_around_snapshotter_new(
        scene: *mut c_void,
        options_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_look_around_snapshotter_get_snapshot(
        snapshotter: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_look_around_snapshotter_cancel(snapshotter: *mut c_void);
    pub fn mk_look_around_snapshotter_is_loading(snapshotter: *mut c_void) -> bool;
    pub fn mk_look_around_snapshotter_release(snapshotter: *mut c_void);

    pub fn mk_look_around_snapshot_state_json(
        snapshot: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_look_around_snapshot_release(snapshot: *mut c_void);
}
