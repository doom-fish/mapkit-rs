//! Async API for MapKit (requires the `async` cargo feature)
//!
//! Provides executor-agnostic [`Future`] wrappers around MapKit's
//! completion-handler APIs.  Works with any async runtime (Tokio, async-std,
//! smol, pollster, …).
//!
//! ## Available async types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AsyncMKLocalSearch`] | Async local search |
//! | [`AsyncMKDirections`] | Async route calculation and ETA |
//! | [`AsyncMKMapSnapshotter`] | Async map snapshot capture |
//! | [`AsyncMKGeocodingRequest`] | Async forward geocoding (macOS 26.0+) |
//! | [`AsyncMKReverseGeocodingRequest`] | Async reverse geocoding (macOS 26.0+) |
//!
//! ## Multi-fire delegates (Tier 2)
//!
//! The following MapKit surfaces use *multi-fire* delegate patterns and are
//! **not** wrapped here — they require a Stream-based (Tier-2) approach:
//!
//! - `MKLocalSearchCompleter` (fires many completion suggestions)
//! - `MKMapViewDelegate` (fires for every region/overlay/annotation change)
//! - `MKLookAroundViewControllerDelegate`
//!
//! ## Notes on the main run loop
//!
//! `MKLocalSearch` and `MKDirections` dispatch their completion handlers on
//! the main queue.  In headless programs without a main run loop (e.g. plain
//! `cargo run`) these futures will not resolve.  Either spin the main run loop
//! (`NSRunLoop.main.run(until:)`) or run inside an AppKit / SwiftUI app.
//!
//! [`MKMapSnapshotter`] accepts a background `DispatchQueue` and does not
//! require the main run loop.
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { pollster::block_on(async {
//! use mapkit::async_api::AsyncMKLocalSearch;
//! use mapkit::MKLocalSearchRequest;
//!
//! let request = MKLocalSearchRequest::new("coffee");
//! let response = AsyncMKLocalSearch::search(&request)?.await?;
//! println!("Found {} items", response.map_items.len());
//! # Ok(()) }) }
//! ```

use std::ffi::{c_char, c_void, CStr};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};

use crate::directions::{MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKETAResponse};
use crate::error::MapKitError;
use crate::geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
use crate::local_search::{
    MKLocalSearch, MKLocalSearchRequest, MKLocalSearchResponse,
};
use crate::map_item::MKMapItem;
use crate::point_of_interest::MKLocalPointsOfInterestRequest;
use crate::snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
use crate::{ffi, MKCoordinate};

// ============================================================================
// Internal Send-safe owned handle wrappers
//
// These newtypes hold a raw *mut c_void that represents a retained
// Objective-C object, and release it on drop.  They implement Send because
// MapKit's service objects are safe to transfer across thread boundaries.
// ============================================================================

struct OwnedLocalSearch(*mut c_void);
unsafe impl Send for OwnedLocalSearch {}
impl Drop for OwnedLocalSearch {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::mk_local_search_release(self.0) }
        }
    }
}

struct OwnedDirections(*mut c_void);
unsafe impl Send for OwnedDirections {}
impl Drop for OwnedDirections {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::mk_directions_release(self.0) }
        }
    }
}

struct OwnedSnapshotter(*mut c_void);
unsafe impl Send for OwnedSnapshotter {}
impl Drop for OwnedSnapshotter {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::mk_map_snapshotter_release(self.0) }
        }
    }
}

struct OwnedGeocodingRequest(*mut c_void);
unsafe impl Send for OwnedGeocodingRequest {}
impl Drop for OwnedGeocodingRequest {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::mk_geocoding_request_release(self.0) }
        }
    }
}

struct OwnedReverseGeocodingRequest(*mut c_void);
unsafe impl Send for OwnedReverseGeocodingRequest {}
impl Drop for OwnedReverseGeocodingRequest {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { ffi::mk_reverse_geocoding_request_release(self.0) }
        }
    }
}

/// Retained raw pointer returned by the snapshot callback.
struct RawSendPtr(*mut c_void);
unsafe impl Send for RawSendPtr {}

// ============================================================================
// Shared C callbacks
// ============================================================================

/// C callback for all JSON-result async functions (T = String).
extern "C" fn json_completion_cb(
    json: *const c_char,
    error: *const c_char,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<String>::complete_err(ctx, msg) };
    } else if !json.is_null() {
        let s = unsafe { CStr::from_ptr(json).to_string_lossy().into_owned() };
        unsafe { AsyncCompletion::complete_ok(ctx, s) };
    } else {
        unsafe { AsyncCompletion::<String>::complete_err(ctx, "empty async result".to_string()) };
    }
}

/// C callback for the snapshot async function (T = RawSendPtr).
extern "C" fn snapshot_handle_cb(
    handle: *mut c_void,
    error: *const c_char,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let msg = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<RawSendPtr>::complete_err(ctx, msg) };
    } else if !handle.is_null() {
        unsafe { AsyncCompletion::complete_ok(ctx, RawSendPtr(handle)) };
    } else {
        unsafe {
            AsyncCompletion::<RawSendPtr>::complete_err(
                ctx,
                "null snapshot handle from Swift".to_string(),
            );
        }
    }
}

// ============================================================================
// Shared JSON parse helper
// ============================================================================

fn parse_async_json<T: serde::de::DeserializeOwned>(
    json: &str,
    context: &str,
) -> Result<T, MapKitError> {
    serde_json::from_str(json).map_err(|e| {
        MapKitError::OperationFailed(format!(
            "failed to parse {context} JSON: {e}; payload={json}"
        ))
    })
}

// ============================================================================
// MKLocalSearch — async start
// ============================================================================

/// Future returned by [`AsyncMKLocalSearch::start`].
pub struct LocalSearchStartFuture {
    _owned: OwnedLocalSearch,
    inner: AsyncCompletionFuture<String>,
}

impl Future for LocalSearchStartFuture {
    type Output = Result<MKLocalSearchResponse, MapKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            r.map_err(MapKitError::OperationFailed)
                .and_then(|json| parse_async_json(&json, "MKLocalSearchResponse"))
        })
    }
}

/// Async wrapper around `MKLocalSearch.start(completionHandler:)`.
///
/// # Note
///
/// MapKit dispatches the completion handler on the **main queue**.  In
/// headless programs the main run loop must be spinning for the future
/// to resolve.
pub struct AsyncMKLocalSearch;

impl AsyncMKLocalSearch {
    /// Starts an existing `MKLocalSearch` and returns a future that resolves
    /// to [`MKLocalSearchResponse`].
    pub fn start(search: MKLocalSearch) -> LocalSearchStartFuture {
        let raw = search.into_raw();
        let (future, ctx) = AsyncCompletion::<String>::create();
        unsafe { ffi::mk_local_search_start_async(raw, json_completion_cb, ctx) }
        LocalSearchStartFuture {
            _owned: OwnedLocalSearch(raw),
            inner: future,
        }
    }

    /// Convenience: create an `MKLocalSearch` from `request` and start it.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing the `MKLocalSearch` fails.
    pub fn search(
        request: &MKLocalSearchRequest,
    ) -> Result<LocalSearchStartFuture, MapKitError> {
        Ok(Self::start(MKLocalSearch::new(request)?))
    }

    /// Convenience: create an `MKLocalSearch` from a
    /// [`MKLocalPointsOfInterestRequest`] and start it.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing the `MKLocalSearch` fails.
    pub fn search_points_of_interest(
        request: &MKLocalPointsOfInterestRequest,
    ) -> Result<LocalSearchStartFuture, MapKitError> {
        Ok(Self::start(MKLocalSearch::from_points_of_interest_request(
            request,
        )?))
    }
}

// ============================================================================
// MKDirections — async calculate / calculateETA
// ============================================================================

/// Future returned by [`AsyncMKDirections::calculate`].
pub struct DirectionsCalculateFuture {
    _owned: OwnedDirections,
    inner: AsyncCompletionFuture<String>,
}

impl Future for DirectionsCalculateFuture {
    type Output = Result<MKDirectionsResponse, MapKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            r.map_err(MapKitError::OperationFailed)
                .and_then(|json| parse_async_json(&json, "MKDirectionsResponse"))
        })
    }
}

/// Future returned by [`AsyncMKDirections::calculate_eta`].
pub struct DirectionsEtaFuture {
    _owned: OwnedDirections,
    inner: AsyncCompletionFuture<String>,
}

impl Future for DirectionsEtaFuture {
    type Output = Result<MKETAResponse, MapKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            r.map_err(MapKitError::OperationFailed)
                .and_then(|json| parse_async_json(&json, "MKETAResponse"))
        })
    }
}

/// Async wrapper around `MKDirections.calculate(completionHandler:)` and
/// `MKDirections.calculateETA(completionHandler:)`.
///
/// # Note
///
/// MapKit dispatches the completion handler on the **main queue**.  In
/// headless programs the main run loop must be spinning for the futures
/// to resolve.
pub struct AsyncMKDirections;

impl AsyncMKDirections {
    /// Start an async route calculation on an existing `MKDirections`.
    pub fn calculate(directions: MKDirections) -> DirectionsCalculateFuture {
        let raw = directions.into_raw();
        let (future, ctx) = AsyncCompletion::<String>::create();
        unsafe { ffi::mk_directions_calculate_async(raw, json_completion_cb, ctx) }
        DirectionsCalculateFuture {
            _owned: OwnedDirections(raw),
            inner: future,
        }
    }

    /// Convenience: create `MKDirections` from `request` and calculate.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing `MKDirections` fails.
    pub fn calculate_from_request(
        request: &MKDirectionsRequest,
    ) -> Result<DirectionsCalculateFuture, MapKitError> {
        Ok(Self::calculate(MKDirections::new(request)?))
    }

    /// Start an async ETA calculation on an existing `MKDirections`.
    pub fn calculate_eta(directions: MKDirections) -> DirectionsEtaFuture {
        let raw = directions.into_raw();
        let (future, ctx) = AsyncCompletion::<String>::create();
        unsafe { ffi::mk_directions_calculate_eta_async(raw, json_completion_cb, ctx) }
        DirectionsEtaFuture {
            _owned: OwnedDirections(raw),
            inner: future,
        }
    }

    /// Convenience: create `MKDirections` from `request` and calculate ETA.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing `MKDirections` fails.
    pub fn calculate_eta_from_request(
        request: &MKDirectionsRequest,
    ) -> Result<DirectionsEtaFuture, MapKitError> {
        Ok(Self::calculate_eta(MKDirections::new(request)?))
    }
}

// ============================================================================
// MKMapSnapshotter — async start
// ============================================================================

/// Future returned by [`AsyncMKMapSnapshotter::start`].
pub struct SnapshotterStartFuture {
    _owned: OwnedSnapshotter,
    inner: AsyncCompletionFuture<RawSendPtr>,
}

impl Future for SnapshotterStartFuture {
    type Output = Result<MKMapSnapshot, MapKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            r.map_err(MapKitError::OperationFailed)
                .and_then(|p| {
                    unsafe { MKMapSnapshot::from_raw_ptr(p.0) }
                        .ok_or_else(|| {
                            MapKitError::OperationFailed("null snapshot handle".to_string())
                        })
                })
        })
    }
}

/// Async wrapper around `MKMapSnapshotter.start(completionHandler:)`.
///
/// Unlike `MKLocalSearch` and `MKDirections`, the snapshotter dispatches
/// its callback on a background queue, so the main run loop is **not**
/// required.
pub struct AsyncMKMapSnapshotter;

impl AsyncMKMapSnapshotter {
    /// Start an existing `MKMapSnapshotter` asynchronously.
    pub fn start(snapshotter: MKMapSnapshotter) -> SnapshotterStartFuture {
        let raw = snapshotter.into_raw();
        let (future, ctx) = AsyncCompletion::<RawSendPtr>::create();
        unsafe { ffi::mk_map_snapshotter_start_async(raw, snapshot_handle_cb, ctx) }
        SnapshotterStartFuture {
            _owned: OwnedSnapshotter(raw),
            inner: future,
        }
    }

    /// Convenience: create an `MKMapSnapshotter` from `options` and start it.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing the snapshotter fails.
    pub fn snapshot(
        options: &MKMapSnapshotOptions,
    ) -> Result<SnapshotterStartFuture, MapKitError> {
        Ok(Self::start(MKMapSnapshotter::new(options)?))
    }
}

// ============================================================================
// MKGeocodingRequest — async getMapItems (macOS 26.0+)
// ============================================================================

/// Future returned by [`AsyncMKGeocodingRequest::get_map_items`].
///
/// Note: `MKGeocodingRequest` is a macOS 26.0+ API.  On older systems the
/// future resolves with an `OperationFailed` error.  `CLGeocoder` (which
/// `MKGeocodingRequest` replaces) is deprecated and not wrapped here.
pub struct GeocoderFuture {
    _owned: OwnedGeocodingRequest,
    inner: AsyncCompletionFuture<String>,
}

impl Future for GeocoderFuture {
    type Output = Result<Vec<MKMapItem>, MapKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            r.map_err(MapKitError::OperationFailed)
                .and_then(|json| parse_async_json(&json, "Vec<MKMapItem>"))
        })
    }
}

/// Async wrapper around `MKGeocodingRequest.getMapItems(completionHandler:)`
/// (macOS 26.0+).
///
/// `CLGeocoder` — the older CoreLocation geocoder — is deprecated in favour
/// of this API and is not wrapped by mapkit-rs.
pub struct AsyncMKGeocodingRequest;

impl AsyncMKGeocodingRequest {
    /// Run geocoding on an existing request and return a future that resolves
    /// to a list of [`MKMapItem`]s.
    pub fn get_map_items(request: MKGeocodingRequest) -> GeocoderFuture {
        let raw = request.into_raw();
        let (future, ctx) = AsyncCompletion::<String>::create();
        unsafe { ffi::mk_geocoding_request_map_items_async(raw, json_completion_cb, ctx) }
        GeocoderFuture {
            _owned: OwnedGeocodingRequest(raw),
            inner: future,
        }
    }

    /// Convenience: create a new `MKGeocodingRequest` from `address_string`
    /// and geocode it.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing the request fails.
    pub fn geocode(address_string: &str) -> Result<GeocoderFuture, MapKitError> {
        Ok(Self::get_map_items(MKGeocodingRequest::new(address_string)?))
    }
}

// ============================================================================
// MKReverseGeocodingRequest — async getMapItems (macOS 26.0+)
// ============================================================================

/// Future returned by [`AsyncMKReverseGeocodingRequest::get_map_items`].
pub struct ReverseGeocoderFuture {
    _owned: OwnedReverseGeocodingRequest,
    inner: AsyncCompletionFuture<String>,
}

impl Future for ReverseGeocoderFuture {
    type Output = Result<Vec<MKMapItem>, MapKitError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx).map(|r| {
            r.map_err(MapKitError::OperationFailed)
                .and_then(|json| parse_async_json(&json, "Vec<MKMapItem>"))
        })
    }
}

/// Async wrapper around `MKReverseGeocodingRequest.getMapItems(completionHandler:)`
/// (macOS 26.0+).
pub struct AsyncMKReverseGeocodingRequest;

impl AsyncMKReverseGeocodingRequest {
    /// Run reverse geocoding on an existing request and return a future that
    /// resolves to a list of [`MKMapItem`]s.
    pub fn get_map_items(request: MKReverseGeocodingRequest) -> ReverseGeocoderFuture {
        let raw = request.into_raw();
        let (future, ctx) = AsyncCompletion::<String>::create();
        unsafe { ffi::mk_reverse_geocoding_request_map_items_async(raw, json_completion_cb, ctx) }
        ReverseGeocoderFuture {
            _owned: OwnedReverseGeocodingRequest(raw),
            inner: future,
        }
    }

    /// Convenience: create a new `MKReverseGeocodingRequest` from `location`
    /// and reverse-geocode it.
    ///
    /// # Errors
    ///
    /// Returns an error if constructing the request fails.
    pub fn reverse_geocode(
        location: MKCoordinate,
    ) -> Result<ReverseGeocoderFuture, MapKitError> {
        Ok(Self::get_map_items(MKReverseGeocodingRequest::new(location)?))
    }
}
