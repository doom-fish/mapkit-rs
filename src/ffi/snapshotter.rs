use core::ffi::{c_char, c_void};

extern "C" {
    pub fn mk_map_snapshotter_new(
        options_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_map_snapshotter_start(
        snapshotter: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn mk_map_snapshotter_cancel(snapshotter: *mut c_void);
    pub fn mk_map_snapshotter_is_loading(snapshotter: *mut c_void) -> bool;
    pub fn mk_map_snapshotter_release(snapshotter: *mut c_void);

    pub fn mk_map_snapshot_state_json(
        snapshot: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_snapshot_point_for_coordinate_json(
        snapshot: *mut c_void,
        coordinate_json: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn mk_map_snapshot_release(snapshot: *mut c_void);
}
