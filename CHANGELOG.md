# Changelog

All notable changes to this project will be documented in this file.

## [0.3.0] - 2026-05-17

### Added — Tier-1 Async API (`async` feature)

New cargo feature `async` exposing executor-agnostic `Future` wrappers around
MapKit's completion-handler APIs.  Works with any async runtime (Tokio, async-std,
smol, `pollster`, …).

#### `MKLocalSearch` async
- `AsyncMKLocalSearch::search(&MKLocalSearchRequest) -> Result<LocalSearchStartFuture>` —
  wraps `MKLocalSearch.start(completionHandler:)`.
- `AsyncMKLocalSearch::search_points_of_interest(&MKLocalPointsOfInterestRequest) -> Result<LocalSearchStartFuture>` —
  companion API for `MKLocalPointsOfInterestRequest`.
- `AsyncMKLocalSearch::start(MKLocalSearch) -> LocalSearchStartFuture` — lower-level variant
  accepting a pre-built handle.

#### `MKDirections` async
- `AsyncMKDirections::calculate(MKDirections) -> DirectionsCalculateFuture` —
  wraps `MKDirections.calculate(completionHandler:)`.
- `AsyncMKDirections::calculate_eta(MKDirections) -> DirectionsEtaFuture` —
  wraps `MKDirections.calculateETA(completionHandler:)`.
- `AsyncMKDirections::calculate_from_request` / `calculate_eta_from_request` convenience variants.

#### `MKMapSnapshotter` async
- `AsyncMKMapSnapshotter::start(MKMapSnapshotter) -> SnapshotterStartFuture` —
  wraps `MKMapSnapshotter.start(completionHandler:)` on a background queue (no main run loop required).
- `AsyncMKMapSnapshotter::snapshot(&MKMapSnapshotOptions) -> Result<SnapshotterStartFuture>` —
  convenience variant.

#### `MKGeocodingRequest` async (macOS 26.0+)
- `AsyncMKGeocodingRequest::get_map_items(MKGeocodingRequest) -> GeocoderFuture` —
  wraps `MKGeocodingRequest.getMapItems(completionHandler:)`.
- `AsyncMKGeocodingRequest::geocode(&str) -> Result<GeocoderFuture>` — convenience.
- Note: `CLGeocoder` (deprecated predecessor) is not wrapped.

#### `MKReverseGeocodingRequest` async (macOS 26.0+)
- `AsyncMKReverseGeocodingRequest::get_map_items(MKReverseGeocodingRequest) -> ReverseGeocoderFuture`
- `AsyncMKReverseGeocodingRequest::reverse_geocode(MKCoordinate) -> Result<ReverseGeocoderFuture>`

#### Multi-fire delegate surfaces (Tier 2, deferred)
`MKLocalSearchCompleter` and `MKMapViewDelegate` use multi-fire delegate patterns
and are deferred to a Tier-2 Stream rollout.

#### Infrastructure
- New Swift bridge file `Async.swift` with `@_cdecl` thunks for each API.
- New `src/async_api.rs` module (behind `#[cfg(feature = "async")]`).
- `doom-fish-utils` added as a dependency (`AsyncCompletion` / `AsyncCompletionFuture`).
- `pollster` added as a dev-dependency.
- 3 new examples: `20_async_local_search`, `21_async_directions`, `22_async_snapshot`.
- 13 new tests in `tests/async_api_tests.rs` (all non-ignored tests pass).

## [0.2.3] - 2026-05-17

### Added

- GeoJSON decoding support via `MKGeoJSONDecoder`, `MKGeoJSONFeature`, `MKGeoJSONObject`, and typed geometry payloads.
- The remaining `MKGeometry.h` helpers/constants, including map-rect predicates/transforms, world/null constants, and string/meters conversions.
- `MKMapItemIdentifier`, `MKMapItemRequest`, MapKit launch-option constants, and typed `MKErrorCode` / `MKErrorDomain` helpers.
- Headless-safe Rust models/traits for `MKCompassButton`, `MKPitchControl`, `MKZoomControl`, `MKSelectionAccessory`, `MKMapItemDetailViewController`, `MKLookAroundViewController`, and `MKMapViewDelegate`.
- Smoke tests covering the new geometry, GeoJSON, map-item/error, and UI-surface APIs.

### Changed

- `COVERAGE_AUDIT.md` now verifies all 241 non-exempt public macOS 26.2 MapKit symbols (100% audited coverage, 0 remaining gaps).
- README and coverage documentation now describe the expanded headless-safe control/controller/delegate modeling.

## [0.2.2] - 2026-05-17

- Added `MKMapItemAnnotation`, `MKUserLocation`, `MKPinAnnotationView`, and `MKUserLocationView` wrappers for the remaining annotation-model/view gaps in the requested scope.
- Added `MKMultiPolyline`, `MKMultiPolygon`, `MKMultiPolylineRenderer`, and `MKMultiPolygonRenderer` wrappers plus generic `MKMapView` annotation/overlay insertion helpers and default annotation-view reuse identifiers.
- Expanded tests/examples for the new annotation and multi-overlay surfaces, and bumped the crate version to `0.2.2`.

## [0.2.1] - 2026-05-16

- Added `MKMapCamera`, `MKMapCameraBoundary`, `MKMapCameraZoomRange`, `MKMapConfiguration`, and the standard/hybrid/imagery configuration wrappers.
- Added `MKAnnotationView`, `MKMarkerAnnotationView`, overlay renderer/tile overlay wrappers, and `MKRoadWidthAtZoomScale`.
- Added `MKLocalSearchCompleter` bindings plus the remaining `MKPointOfInterestCategory` convenience constructors and `MKPointsOfInterestRequestMaxRadius` access.
- Added examples and smoke coverage for configuration/camera, local search completer, annotation views, overlay renderers, and point-of-interest requests.
- Updated `COVERAGE_AUDIT.md` to 167 verified macOS MapKit symbols (68.7% coverage).

## [0.2.0] - 2026-05-16

- Split the Swift bridge and Rust surface into logical modules for MapView, Annotation, Overlay, LocalSearch, Directions, Snapshotter, Geocoder, LookAround, PointOfInterest, Address, MKMapItem, ClusterAnnotation, and UserTrackingButton.
- Added headless-safe examples for every logical area plus smoke coverage.
- Added per-area test files covering constructors, round-trips, and service setup on macOS.
- Extended `MKMapItem`, local search, snapshotting, geocoding, and Look Around support to newer MapKit APIs.
- Added `COVERAGE.md` documenting implemented, partial, and skipped APIs from the audited headers.

## [0.1.0] - 2026-05-16

- Initial release with local search, directions, ETA, geometry helpers, and distance formatting.
