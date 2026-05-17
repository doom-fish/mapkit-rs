# Changelog

All notable changes to this project will be documented in this file.

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
