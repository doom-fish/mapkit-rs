use core::ffi::{c_char, c_void};

/// Callback type for async APIs that return a JSON string result.
/// `json` is valid only for the duration of the callback (do not store the pointer).
/// `error` is valid only for the duration of the callback.
/// `ctx` is the opaque context from `AsyncCompletion::create()`.
pub type MKAsyncJsonCb =
    extern "C" fn(json: *const c_char, error: *const c_char, ctx: *mut c_void);

/// Callback type for async APIs that return a retained opaque object handle.
pub type MKAsyncHandleCb =
    extern "C" fn(handle: *mut c_void, error: *const c_char, ctx: *mut c_void);

extern "C" {
    /// Starts `MKLocalSearch` asynchronously; fires `cb` with JSON result or error.
    /// The search handle must remain alive until `cb` fires.
    pub fn mk_local_search_start_async(
        search: *mut c_void,
        cb: MKAsyncJsonCb,
        ctx: *mut c_void,
    );

    /// Calculates directions asynchronously; fires `cb` with JSON result or error.
    pub fn mk_directions_calculate_async(
        directions: *mut c_void,
        cb: MKAsyncJsonCb,
        ctx: *mut c_void,
    );

    /// Calculates ETA asynchronously; fires `cb` with JSON result or error.
    pub fn mk_directions_calculate_eta_async(
        directions: *mut c_void,
        cb: MKAsyncJsonCb,
        ctx: *mut c_void,
    );

    /// Starts `MKMapSnapshotter` asynchronously; fires `cb` with a retained snapshot handle or error.
    pub fn mk_map_snapshotter_start_async(
        snapshotter: *mut c_void,
        cb: MKAsyncHandleCb,
        ctx: *mut c_void,
    );

    /// Runs `MKGeocodingRequest.getMapItems` asynchronously (macOS 26.0+);
    /// fires `cb` with JSON result or error.
    pub fn mk_geocoding_request_map_items_async(
        request: *mut c_void,
        cb: MKAsyncJsonCb,
        ctx: *mut c_void,
    );

    /// Runs `MKReverseGeocodingRequest.getMapItems` asynchronously (macOS 26.0+);
    /// fires `cb` with JSON result or error.
    pub fn mk_reverse_geocoding_request_map_items_async(
        request: *mut c_void,
        cb: MKAsyncJsonCb,
        ctx: *mut c_void,
    );
}
