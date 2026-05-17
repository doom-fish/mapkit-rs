# MapKit.framework coverage audit (macOS 26.2 SDK)

This crate audits the requested MapKit headers and groups them into the logical areas requested for `mapkit` v0.2.2.

## Implemented

| Area | Header / API surface | Status | Notes |
| --- | --- | --- | --- |
| MapView | `MKMapView` creation, region/center/map-rect conversion, interaction flags, compass/scale/zoom toggles, user-location visibility, generic annotation/overlay insertion/removal, and default reuse identifiers | ✅ | `src/map_view.rs`, `swift-bridge/Sources/MapKitBridge/MapView.swift` |
| Annotation | `MKPointAnnotation`, `MKMapItemAnnotation`, and `MKUserLocation` coordinate/title/subtitle round-trips | ✅ | `src/annotation.rs`, `Annotation.swift` |
| AnnotationView | `MKAnnotationView`, `MKMarkerAnnotationView`, `MKPinAnnotationView`, and `MKUserLocationView` state/apply bridges | ✅ | `src/annotation_view.rs`, `AnnotationView.swift` |
| Overlay | `MKCircle`, `MKPolyline`, `MKMultiPolyline`, `MKPolygon`, `MKMultiPolygon`, overlay level handling, bounding map rect, and coordinate extraction | ✅ | `src/overlay.rs`, `Overlay.swift` |
| OverlayRenderer | `MKOverlayRenderer`, `MKOverlayPathRenderer`, circle/polyline/polygon/tile renderers, gradient polyline, and multi-overlay renderers | ✅ | `src/overlay_renderer.rs`, `OverlayRenderer.swift` |
| LocalSearch | `MKLocalSearch`, `MKLocalSearch.Request`, `MKLocalSearch.Response`, result types, region priority, POI/address filtering hooks | ✅ | `src/local_search.rs`, `LocalSearch.swift` |
| Directions | `MKDirections`, request, ETA, routes, steps, transport types, toll/highway preferences | ✅ | `src/directions.rs`, `Directions.swift` |
| Snapshotter | `MKMapSnapshotter`, options, snapshot size/point lookup | ✅ | `src/snapshotter.rs`, `Snapshotter.swift` |
| Geocoder | `MKGeocodingRequest`, `MKReverseGeocodingRequest`, locale and region configuration, async map-item retrieval | ✅ | `src/geocoder.rs`, `Geocoder.swift` |
| LookAround | `MKLookAroundSceneRequest`, scene retrieval, snapshotter/options/snapshot wrappers | ✅ | `src/look_around.rs`, `LookAround.swift` |
| PointOfInterest | `MKPointOfInterestFilter`, full category convenience constructors, `MKLocalPointsOfInterestRequest` builders | ✅ | `src/point_of_interest.rs`, `PointOfInterest.swift` |
| Address | `MKAddress`, `MKAddressFilter`, `MKAddressRepresentations` extraction from returned map items | ✅ | `src/address.rs`, `Address.swift` |
| MKMapItem | `MKMapItem`/`MKPlacemark` payloads, location/address-based construction, alternate identifiers, address/address representations encoding | ✅ | `src/map_item.rs`, `MapItem.swift` |
| ClusterAnnotation | `MKClusterAnnotation` construction and title/subtitle/member-count access | ✅ | `src/cluster_annotation.rs`, `ClusterAnnotation.swift` |
| UserTrackingButton | macOS-equivalent user-tracking button visibility + tracking mode controls via `MKMapView` | ✅ | `src/user_tracking_button.rs`, `UserTrackingButton.swift` |

## Partial / skipped

| Area | Header / API surface | Status | Reason |
| --- | --- | --- | --- |
| MapView | Delegate callbacks and renderer dequeue/registration hooks | 🟡 | Delegate-driven view customisation is not yet surfaced as Rust traits. |
| Directions | URL parsing helpers on `MKDirectionsRequest` | ⏭️ | Request-URL parsing was not required for the bridge expansion. |
| Snapshotter | `preferredConfiguration`, `camera`, `appearance` | 🟡 | The snapshotter currently exposes region/mapRect/mapType/POI/buildings/size only. |
| Address | `regionCode` | 🟡 | `regionCode` is Swift-refined away in the SDK; the field is emitted as `None` today. |
| MKMapItem | `openInMaps` / launch-option helpers / identifier requests | ⏭️ | Launching the Maps app or brokered request flows is intentionally avoided in tests/examples. |
| LookAround | `MKLookAroundViewController` | ⏭️ | View-controller presentation is not headless-safe. |
| ClusterAnnotation | Detailed member enumeration beyond count | 🟡 | The bridge currently exposes aggregate metadata only. |
| UserTrackingButton | Standalone `MKUserTrackingButton` class | ⏭️ | Not present in the native macOS SDK; `MKMapView.showsUserTrackingButton` is wrapped instead. |
| Other framework headers | GeoJSON helpers, map controls (`MKCompassButton`, `MKZoomControl`, `MKPitchControl`), selection/detail controllers | ⏭️ | These remaining gaps are UI-only, launcher-oriented, or otherwise outside the headless crate scope. |
