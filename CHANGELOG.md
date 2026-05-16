# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-05-16

- Split the Swift bridge and Rust surface into logical modules for MapView, Annotation, Overlay, LocalSearch, Directions, Snapshotter, Geocoder, LookAround, PointOfInterest, Address, MKMapItem, ClusterAnnotation, and UserTrackingButton.
- Added headless-safe examples for every logical area plus smoke coverage.
- Added per-area test files covering constructors, round-trips, and service setup on macOS.
- Extended `MKMapItem`, local search, snapshotting, geocoding, and Look Around support to newer MapKit APIs.
- Added `COVERAGE.md` documenting implemented, partial, and skipped APIs from the audited headers.

## [0.1.0] - 2026-05-16

- Initial release with local search, directions, ETA, geometry helpers, and distance formatting.
