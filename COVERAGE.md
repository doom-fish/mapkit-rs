# MapKit.framework coverage audit (macOS 26.2 SDK)

This crate audits the requested MapKit headers and groups them into the logical areas requested for `mapkit` v0.2.0.

## Implemented

| Area | Header / API surface | Status | Notes |
| --- | --- | --- | --- |
| MapView | `MKMapView` creation, region/center/map-rect conversion, interaction flags, compass/scale/zoom toggles, user-location visibility, annotation/overlay insertion/removal | ✅ | `src/map_view.rs`, `swift-bridge/Sources/MapKitBridge/MapView.swift` |
| Annotation | `MKPointAnnotation` coordinate/title/subtitle round-trips | ✅ | `src/annotation.rs`, `Annotation.swift` |
| Overlay | `MKCircle`, `MKPolyline`, `MKPolygon`, overlay level handling, bounding map rect and coordinate extraction | ✅ | `src/overlay.rs`, `Overlay.swift` |
| LocalSearch | `MKLocalSearch`, `MKLocalSearch.Request`, `MKLocalSearch.Response`, result types, region priority, POI/address filtering hooks | ✅ | `src/local_search.rs`, `LocalSearch.swift` |
| Directions | `MKDirections`, request, ETA, routes, steps, transport types, toll/highway preferences | ✅ | `src/directions.rs`, `Directions.swift` |
| Snapshotter | `MKMapSnapshotter`, options, snapshot size/point lookup | ✅ | `src/snapshotter.rs`, `Snapshotter.swift` |
| Geocoder | `MKGeocodingRequest`, `MKReverseGeocodingRequest`, locale and region configuration, async map-item retrieval | ✅ | `src/geocoder.rs`, `Geocoder.swift` |
| LookAround | `MKLookAroundSceneRequest`, scene retrieval, snapshotter/options/snapshot wrappers | ✅ | `src/look_around.rs`, `LookAround.swift` |
| PointOfInterest | `MKPointOfInterestFilter`, common categories, `MKLocalPointsOfInterestRequest` builders | ✅ | `src/point_of_interest.rs`, `PointOfInterest.swift` |
| Address | `MKAddress`, `MKAddressFilter`, `MKAddressRepresentations` extraction from returned map items | ✅ | `src/address.rs`, `Address.swift` |
| MKMapItem | `MKMapItem`/`MKPlacemark` payloads, location/address-based construction, alternate identifiers, address/address representations encoding | ✅ | `src/map_item.rs`, `MapItem.swift` |
| ClusterAnnotation | `MKClusterAnnotation` construction and title/subtitle/member-count access | ✅ | `src/cluster_annotation.rs`, `ClusterAnnotation.swift` |
| UserTrackingButton | macOS-equivalent user-tracking button visibility + tracking mode controls via `MKMapView` | ✅ | `src/user_tracking_button.rs`, `UserTrackingButton.swift` |

## Partial / skipped

| Area | Header / API surface | Status | Reason |
| --- | --- | --- | --- |
| MapView | Delegate callbacks, renderer registration/dequeue, camera/camera boundary/zoom-range APIs | 🟡 | Delegate-driven view customisation is not yet surfaced as Rust traits. |
| Annotation | `MKAnnotationView`, `MKMarkerAnnotationView`, `MKPinAnnotationView` | ⏭️ | Rendering/view customisation APIs are NSView-heavy and require delegate plumbing. |
| Overlay | Renderer families (`MKOverlayRenderer`, `MKCircleRenderer`, `MKPolygonRenderer`, `MKPolylineRenderer`, tile overlays) | ⏭️ | Drawing and renderer subclasses require delegate-managed AppKit integration. |
| LocalSearch | `MKLocalSearchCompleter` | ⏭️ | Separate completer service; outside the requested logical areas. |
| Directions | URL parsing helpers on `MKDirectionsRequest` | ⏭️ | Request-URL parsing was not required for the bridge expansion. |
| Snapshotter | `preferredConfiguration`, `camera`, `appearance` | 🟡 | The snapshotter currently exposes region/mapRect/mapType/POI/buildings/size only. |
| PointOfInterest | One Rust convenience constant for every category symbol in `MKPointOfInterestCategory.h` | 🟡 | The filter accepts raw category strings plus common helpers, but not every constant has a dedicated Rust constructor. |
| Address | `regionCode` | 🟡 | `regionCode` is Swift-refined away in the SDK; the field is emitted as `None` today. |
| MKMapItem | `openInMaps` / launch-option helpers | ⏭️ | Launching the Maps app is intentionally avoided in tests/examples. |
| LookAround | `MKLookAroundViewController` | ⏭️ | View-controller presentation is not headless-safe. |
| ClusterAnnotation | Detailed member enumeration beyond count | 🟡 | The bridge currently exposes aggregate metadata only. |
| UserTrackingButton | Standalone `MKUserTrackingButton` class | ⏭️ | Not present in the native macOS SDK; `MKMapView.showsUserTrackingButton` is wrapped instead. |
| Other framework headers | GeoJSON, map configuration families, controls (`MKCompassButton`, `MKZoomControl`, `MKPitchControl`), renderer/view-controller helpers | ⏭️ | Not requested for this crate pass and either UI-only, launcher-oriented, or out of scope for headless examples. |
