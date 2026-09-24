# mapkit

Safe Rust bindings for Apple's [MapKit](https://developer.apple.com/documentation/mapkit) framework on macOS 13+. APIs that MapKit introduced later, such as `MKMapItemRequest` (macOS 15) and the geocoding requests (macOS 26), return an error on older systems.

> **Status:** v0.4.0 connects `MKMapViewDelegate` to `MKMapView`, so overlays get renderers and map events arrive, requires the main thread for AppKit views, and makes the synchronous waits race-free and cancellable. `COVERAGE.md` explains what the audited coverage numbers include.

```toml
[dependencies]
mapkit = "0.4"
```

## Quick start

```rust,no_run
use mapkit::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 240.0))?;
    let center = MKCoordinate::new(37.3349, -122.0090);
    let region = MKCoordinateRegion::with_distance(center, 2_000.0, 2_000.0)?;
    map_view.set_region(region, false)?;

    let annotation = MKPointAnnotation::new(center)?;
    annotation.set_title(Some("Apple Park"))?;
    map_view.add_point_annotation(&annotation)?;

    println!("annotations={}", map_view.annotation_count()?);
    Ok(())
}
```

## Async API

Enable the `async` Cargo feature for executor-agnostic [`Future`][std::future::Future] wrappers
around MapKit's completion-handler APIs (works with Tokio, async-std, smol,
pollster, …):

```toml
[dependencies]
mapkit = { version = "0.4", features = ["async"] }
```

```rust,no_run
# #[cfg(feature = "async")]
# fn main() -> Result<(), Box<dyn std::error::Error>> { pollster::block_on(async {
use mapkit::async_api::{AsyncMKLocalSearch, AsyncMKDirections};
use mapkit::local_search::MKLocalSearchRequest;

// Async local search
let response = AsyncMKLocalSearch::search(
    &MKLocalSearchRequest::new("coffee")
)?.await?;
println!("Found {} results", response.map_items.len());
# Ok(()) }) }
# #[cfg(not(feature = "async"))]
# fn main() {}
```

### Async surface (Tier 1)

| Type | Maps to |
|------|---------|
| `AsyncMKLocalSearch` | `MKLocalSearch.start(completionHandler:)` |
| `AsyncMKDirections` | `MKDirections.calculate` / `calculateETA` |
| `AsyncMKMapSnapshotter` | `MKMapSnapshotter.start(completionHandler:)` |
| `AsyncMKGeocodingRequest` | `MKGeocodingRequest.getMapItems` (macOS 26.0+) |
| `AsyncMKReverseGeocodingRequest` | `MKReverseGeocodingRequest.getMapItems` (macOS 26.0+) |

> **Note:** `MKLocalSearch` and `MKDirections` dispatch their callbacks on the
> main queue; the main run loop must be active for those futures to resolve.
> `MKMapSnapshotter` uses a background queue and has no such restriction.
>
> `MKLocalSearchCompleter` updates are not exposed as a stream yet.
> `MKMapViewDelegate` is a callback delegate; see below.

## Map view delegate

`MKMapView::set_delegate` connects an `MKMapViewDelegate` and returns a registration; the delegate stays connected until the registration is dropped. MapKit asks the delegate for overlay renderers, so implement `map_view_renderer_for_overlay` to draw overlays:

```rust,no_run
use mapkit::prelude::*;

struct Routes;

impl MKMapViewDelegate for Routes {
    fn map_view_renderer_for_overlay(
        &mut self,
        _map_view: &MKMapView,
        overlay: &MKAnyOverlay,
    ) -> Option<Box<dyn MKOverlayRendering>> {
        match overlay {
            MKAnyOverlay::Polyline(polyline) => {
                Some(Box::new(MKPolylineRenderer::new(polyline).ok()?))
            }
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map_view = MKMapView::new(MKScreenSize::new(320.0, 240.0))?;
    let _delegate = map_view.set_delegate(Routes)?;
    let route = MKPolyline::new(&[
        MKCoordinate::new(37.3349, -122.0090),
        MKCoordinate::new(37.3318, -122.0312),
    ])?;
    map_view.add_overlay(&route, MKOverlayLevel::AboveRoads)?;
    Ok(())
}
```

The delegate also receives region, loading, rendering, selection, drag, user-location and tracking-mode callbacks. `mapView(_:viewFor:)`, `didAdd` views and renderers, cluster annotations and selection accessories are not bridged.

## Threads

- `MKMapView`, the annotation views and `MKUserTrackingButton` are AppKit views and are `!Send`. Create and use them on the main thread; on any other thread their methods return an error instead of hopping to the main queue.
- MapKit calls the map view delegate on the main thread. A callback that MapKit raises while another callback of the same delegate is still running is skipped rather than re-entering the delegate.
- MapKit delivers `MKLocalSearch`, `MKDirections`, geocoding, Look Around, `MKMapItemRequest` and `MKLocalSearchCompleter` results on the main queue. Their synchronous methods wait for that result, so they return an error on the main thread instead of spinning its run loop. Call them from another thread while the main thread runs its run loop, or use the `async` API. A wait that times out cancels the request and returns an error.
- `MKMapSnapshotter` delivers on a background queue and works from any thread.
- Overlay renderers aren't bound to a thread: MapKit draws them on background threads, and they can be created and styled on any thread.

## Sealed traits

`MKAnnotation`, `MKOverlay` and `MKOverlayRendering` hand the native MapKit object behind a value to MapKit, so only this crate's types implement them:

```rust,compile_fail
use core::ffi::c_void;
use mapkit::prelude::*;

struct Fake;

impl MKOverlayRendering for Fake {
    fn as_raw_renderer(&self) -> *mut c_void {
        core::ptr::null_mut()
    }
}
```

## Highlights

- `MKMapView` creation, region/map-rect conversions, interaction toggles, camera boundaries/zoom ranges, preferred configurations, generic annotations/overlays, default reuse identifiers, and user tracking mode
- `MKPointAnnotation`, `MKMapItemAnnotation`, `MKUserLocation`, `MKAnnotationView`, `MKMarkerAnnotationView`, `MKPinAnnotationView`, `MKUserLocationView`, `MKClusterAnnotation`, `MKCircle`, `MKPolyline`, `MKMultiPolyline`, `MKPolygon`, `MKMultiPolygon`, `MKTileOverlay`, and overlay renderers
- `MKLocalSearch`, `MKLocalSearchCompleter`, and `MKDirections` request/response bridges for search and routing services
- `MKMapSnapshotter` and `MKLookAroundSnapshotter` wrappers for headless image generation
- `MKGeocodingRequest` and `MKReverseGeocodingRequest` on macOS 26+
- `MKPointOfInterestFilter`, `MKLocalPointsOfInterestRequest`, the full `MKPointOfInterestCategory` convenience set, `MKAddress`, and `MKAddressFilter`
- `MKMapItem` / `MKPlacemark` data models, launch-option constants, `MKMapItemIdentifier`, `MKMapItemRequest`, and `MKErrorCode` / `MKErrorDomain` helpers
- `MKGeoJSONDecoder` plus the full audited `MKGeometry.h` helper surface, `MKMapViewDelegate` connected through `MKMapView::set_delegate`, and Rust-side models (no AppKit object behind them) for map controls, selection accessories, and the map-item detail / Look Around controllers

## Examples

This crate ships numbered, headless-safe examples for every logical area:

```bash
cargo run --example 01_mapkit_smoke
cargo run --example 15_configuration_camera
cargo run --example 17_annotation_view_basic
cargo run --example 18_overlay_renderer_basic
cargo run --example 19_points_of_interest_request
```

Expected success footer from the smoke example:

```text
✅ mapkit OK
```

## Notes

- The examples are designed to exit successfully on a headless macOS host.
- `tests/main_thread.rs` is a harness-free test binary that runs on the process main thread, so `cargo test` exercises the views and the map view delegate headlessly.
- `MKUserTrackingButton` is not exposed as a standalone native macOS class in the SDK, so this crate wraps the equivalent `MKMapView.showsUserTrackingButton` / `userTrackingMode` functionality instead.
- `COVERAGE.md` and `COVERAGE_AUDIT.md` document the audited surface, which UI/controller types are Rust-side models rather than AppKit objects, and the two exempt SDK declarations.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
