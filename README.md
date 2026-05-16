# mapkit

Safe Rust bindings for Apple's [MapKit](https://developer.apple.com/documentation/mapkit) framework on macOS.

> **Status:** v0.2.1 covers the requested MapKit areas for headless macOS workflows: `MKMapView`, annotation views, overlay renderers, camera/configuration, local search + completer, directions, snapshotters, geocoding, Look Around, expanded point-of-interest filters/constants, addresses, map items, cluster annotations, and user-tracking button visibility.

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

## Highlights

- `MKMapView` creation, region/map-rect conversions, interaction toggles, camera boundaries/zoom ranges, preferred configurations, annotations, overlays, and user tracking mode
- `MKPointAnnotation`, `MKAnnotationView`, `MKMarkerAnnotationView`, `MKClusterAnnotation`, `MKCircle`, `MKPolyline`, `MKPolygon`, `MKTileOverlay`, and overlay renderers
- `MKLocalSearch`, `MKLocalSearchCompleter`, and `MKDirections` request/response bridges for search and routing services
- `MKMapSnapshotter` and `MKLookAroundSnapshotter` wrappers for headless image generation
- `MKGeocodingRequest` and `MKReverseGeocodingRequest` on macOS 26+
- `MKPointOfInterestFilter`, `MKLocalPointsOfInterestRequest`, the full `MKPointOfInterestCategory` convenience set, `MKAddress`, and `MKAddressFilter`
- `MKMapItem` / `MKPlacemark` data models that round-trip through the Swift bridge

## Examples

This crate ships numbered, headless-safe examples for every logical area:

```bash
cargo run --example 01_mapkit_smoke
cargo run --example 15_configuration_camera
cargo run --example 17_annotation_view_basic
cargo run --example 19_points_of_interest_request
```

Expected success footer from the smoke example:

```text
✅ mapkit OK
```

## Notes

- The examples are designed to exit successfully on a headless macOS host.
- Some UI-heavy integration tests are `#[ignore]` under `cargo test` because MapKit view/rendering objects require a dedicated main-thread process; the matching numbered examples exercise those APIs directly.
- `MKUserTrackingButton` is not exposed as a standalone native macOS class in the SDK, so this crate wraps the equivalent `MKMapView.showsUserTrackingButton` / `userTrackingMode` functionality instead.
- `COVERAGE.md` contains the SDK audit and the skip/defer rationale for UI- or launcher-oriented APIs that are intentionally not exercised from tests.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
