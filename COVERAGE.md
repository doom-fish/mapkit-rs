# MapKit.framework coverage audit (macOS 26.2 SDK)

This crate audits the requested MapKit headers and groups them into the logical areas shipped in `mapkit` v0.2.3.

The audited public macOS 26.2 surface is covered for non-exempt declarations: 241 verified symbols, 0 gaps, and 2 exempt SDK items tracked in `COVERAGE_AUDIT.md`. The count is of top-level declarations, and nine of the verified UI types are Rust-side models rather than AppKit objects (see below). The macOS 26.5 MapKit headers are identical to 26.2. The macOS 27.0 SDK adds eleven `MKPointOfInterestCategory` constants (airport terminal, automotive, commercial-vehicle and motorbike dealerships, information booth, picnic area, ranger station, rest area, scenic view, ticket office and visitor center) that are not wrapped yet.

## Implemented

| Area | Header / API surface | Status | Notes |
| --- | --- | --- | --- |
| Geometry | `MKCoordinateRegion` / `MKMapPoint` / `MKMapSize` / `MKMapRect` helpers, constants, predicates, transforms, and formatting helpers from `MKGeometry.h` | ✅ | `src/geometry.rs`, `swift-bridge/Sources/MapKitBridge/Geometry.swift` |
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
| GeoJSON | `MKGeoJSONDecoder`, `MKGeoJSONFeature`, `MKGeoJSONObject`, and decoded geometry payloads | ✅ | `src/geojson.rs`, `swift-bridge/Sources/MapKitBridge/GeoJSON.swift` |
| PointOfInterest | `MKPointOfInterestFilter`, full category convenience constructors, `MKLocalPointsOfInterestRequest` builders | ✅ | `src/point_of_interest.rs`, `PointOfInterest.swift` |
| Address | `MKAddress`, `MKAddressFilter`, `MKAddressRepresentations` extraction from returned map items | ✅ | `src/address.rs`, `Address.swift` |
| MapItem | `MKMapItem` / `MKPlacemark` payloads, location/address-based construction, alternate identifiers, launch-option constants, and brokered request helpers | ✅ | `src/map_item.rs`, `swift-bridge/Sources/MapKitBridge/MapItem.swift` |
| Errors | `MKErrorCode`, `MKErrorDomain`, and typed `NSErrorInfo` helpers for MapKit failures | ✅ | `src/error.rs` |
| ClusterAnnotation | `MKClusterAnnotation` construction and title/subtitle/member-count access | ✅ | `src/cluster_annotation.rs`, `ClusterAnnotation.swift` |
| MapViewDelegate | `MKMapViewDelegate` region, loading, rendering, selection, drag, user-location and tracking-mode callbacks, and `mapView(_:rendererFor:)` | ✅ | `MKMapView::set_delegate` installs a Swift proxy (`src/map_view_delegate.rs`, `MapViewDelegate.swift`). `mapView(_:viewFor:)`, the `didAdd` callbacks, cluster annotations and selection accessories are not bridged. |
| Controls + controllers | `MKCompassButton`, `MKPitchControl`, `MKZoomControl`, `MKSelectionAccessory`, `MKMapItemDetailSelectionAccessoryPresentationStyle`, `MKMapItemDetailViewController`, `MKLookAroundViewController`, and their delegates | 🟡 model only | Rust-side models in `src/controls.rs`, `src/selection_accessory.rs`, `src/map_item_detail_view_controller.rs`, and `src/look_around_view_controller.rs`. No AppKit object is created, and their delegates are only called by the models' own methods. |
| UserTrackingButton | macOS-equivalent user-tracking button visibility + tracking mode controls via `MKMapView` | ✅ | `src/user_tracking_button.rs`, `UserTrackingButton.swift` |

## Exempt

| Symbol | Reason |
| --- | --- |
| `MKSearchCompletionFilterType` | Deprecated on macOS 10.15 and superseded by `MKLocalSearchCompleterResultType`. |
| `MKPlacemark` | Deprecated on macOS 26.0, but retained as a compatibility wrapper because MapKit still hands placemark payloads back to callers. |

## Notes

- `MKMapView`, the annotation views and `MKUserTrackingButton` are real AppKit objects and must be used on the main thread. The controls, selection accessories and view controllers listed above are Rust-side models, not `NSView` / `NSViewController` bindings.
- `COVERAGE_AUDIT.md` is the source-of-truth symbol ledger used to track verified vs exempt declarations.
