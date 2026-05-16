# mapkit coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 243
VERIFIED: 66
GAPS: 173
EXEMPT: 4
COVERAGE_PCT: 27.6%

This audit counts named top-level macOS declarations from MapKit.framework headers (interfaces, protocols, typedefs/enums/structs, exported constants, and top-level functions). Objective-C categories were not counted as standalone symbols.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| MKAddress | interface | MKAddress.h | MKAddress (src/address.rs) |
| MKAddressFilter | interface | MKAddressFilter.h | MKAddressFilter (src/address.rs) |
| MKAddressFilterOption | enum | MKAddressFilter.h | MKAddressFilterOption (src/address.rs) |
| MKAddressRepresentations | interface | MKAddressRepresentations.h | MKAddressRepresentations (src/address.rs) |
| MKAddressRepresentationsContextStyle | enum | MKAddressRepresentations.h | MKAddressRepresentationsContextStyle (src/address.rs) |
| MKCircle | interface | MKCircle.h | MKCircle (src/overlay.rs) |
| MKClusterAnnotation | interface | MKClusterAnnotation.h | MKClusterAnnotation (src/cluster_annotation.rs) |
| MKDirections | interface | MKDirections.h | MKDirections (src/directions.rs) |
| MKDirectionsRequest | interface | MKDirectionsRequest.h | MKDirectionsRequest (src/directions.rs) |
| MKDirectionsRoutePreference | enum | MKDirectionsRequest.h | MKDirectionsRoutePreference (src/directions.rs) |
| MKDirectionsResponse | interface | MKDirectionsResponse.h | MKDirectionsResponse (src/directions.rs) |
| MKETAResponse | interface | MKDirectionsResponse.h | MKETAResponse (src/directions.rs) |
| MKRoute | interface | MKDirectionsResponse.h | MKRoute (src/directions.rs) |
| MKRouteStep | interface | MKDirectionsResponse.h | MKRouteStep (src/directions.rs) |
| MKDirectionsTransportType | enum | MKDirectionsTypes.h | MKDirectionsTransportType (src/directions.rs) |
| MKDistanceFormatter | interface | MKDistanceFormatter.h | MKDistanceFormatter (src/distance_formatter.rs) |
| MKGeocodingRequest | interface | MKGeocodingRequest.h | MKGeocodingRequest (src/geocoder.rs) |
| MKCoordinateForMapPoint | function | MKGeometry.h | MKMapPoint::coordinate (src/geometry.rs) |
| MKCoordinateRegion | struct | MKGeometry.h | MKCoordinateRegion (src/geometry.rs) |
| MKCoordinateRegionMake | function | MKGeometry.h | MKCoordinateRegion::new (src/geometry.rs) |
| MKCoordinateRegionMakeWithDistance | function | MKGeometry.h | MKCoordinateRegion::with_distance (src/geometry.rs) |
| MKCoordinateSpan | struct | MKGeometry.h | MKCoordinateSpan (src/geometry.rs) |
| MKCoordinateSpanMake | function | MKGeometry.h | MKCoordinateSpan::new (src/geometry.rs) |
| MKMapPoint | struct | MKGeometry.h | MKMapPoint (src/geometry.rs) |
| MKMapPointForCoordinate | function | MKGeometry.h | MKMapPoint::from_coordinate (src/geometry.rs) |
| MKMapPointMake | function | MKGeometry.h | MKMapPoint::new (src/geometry.rs) |
| MKMapRect | struct | MKGeometry.h | MKMapRect (src/geometry.rs) |
| MKMapRectMake | function | MKGeometry.h | MKMapRect::new (src/geometry.rs) |
| MKMapSize | struct | MKGeometry.h | MKMapSize (src/geometry.rs) |
| MKMapSizeMake | function | MKGeometry.h | MKMapSize::new (src/geometry.rs) |
| MKMetersBetweenMapPoints | function | MKGeometry.h | MKMapPoint::distance_to (src/geometry.rs) |
| MKLocalPointsOfInterestRequest | interface | MKLocalPointsOfInterestRequest.h | MKLocalPointsOfInterestRequest (src/point_of_interest.rs) |
| MKLocalSearch | interface | MKLocalSearch.h | MKLocalSearch (src/local_search.rs) |
| MKLocalSearchRequest | interface | MKLocalSearchRequest.h | MKLocalSearchRequest (src/local_search.rs) |
| MKLocalSearchResultType | enum | MKLocalSearchRequest.h | MKLocalSearchResultType (src/local_search.rs) |
| MKLocalSearchResponse | interface | MKLocalSearchResponse.h | MKLocalSearchResponse (src/local_search.rs) |
| MKLookAroundScene | interface | MKLookAroundScene.h | MKLookAroundScene (src/look_around.rs) |
| MKLookAroundSceneRequest | interface | MKLookAroundSceneRequest.h | MKLookAroundSceneRequest (src/look_around.rs) |
| MKLookAroundSnapshot | interface | MKLookAroundSnapshot.h | MKLookAroundSnapshot (src/look_around.rs) |
| MKLookAroundSnapshotOptions | interface | MKLookAroundSnapshotOptions.h | MKLookAroundSnapshotOptions (src/look_around.rs) |
| MKLookAroundSnapshotter | interface | MKLookAroundSnapshotter.h | MKLookAroundSnapshotter (src/look_around.rs) |
| MKMapItem | interface | MKMapItem.h | MKMapItem (src/map_item.rs) |
| MKMapSnapshot | interface | MKMapSnapshot.h | MKMapSnapshot (src/snapshotter.rs) |
| MKMapSnapshotOptions | interface | MKMapSnapshotOptions.h | MKMapSnapshotOptions (src/snapshotter.rs) |
| MKMapSnapshotter | interface | MKMapSnapshotter.h | MKMapSnapshotter (src/snapshotter.rs) |
| MKMapView | interface | MKMapView.h | MKMapView (src/map_view.rs) |
| MKOverlayLevel | enum | MKMapView.h | MKOverlayLevel (src/overlay.rs) |
| MKPointAnnotation | interface | MKPointAnnotation.h | MKPointAnnotation (src/annotation.rs) |
| MKPointOfInterestCategory | typedef | MKPointOfInterestCategory.h | MKPointOfInterestCategory (src/point_of_interest.rs) |
| MKPointOfInterestCategoryAirport | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::airport (src/point_of_interest.rs) |
| MKPointOfInterestCategoryCafe | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::cafe (src/point_of_interest.rs) |
| MKPointOfInterestCategoryHotel | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::hotel (src/point_of_interest.rs) |
| MKPointOfInterestCategoryLibrary | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::library (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMuseum | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::museum (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::park (src/point_of_interest.rs) |
| MKPointOfInterestCategoryRestaurant | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::restaurant (src/point_of_interest.rs) |
| MKPointOfInterestCategorySchool | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::school (src/point_of_interest.rs) |
| MKPointOfInterestCategoryStore | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::store (src/point_of_interest.rs) |
| MKPointOfInterestCategoryUniversity | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::university (src/point_of_interest.rs) |
| MKPointOfInterestFilter | interface | MKPointOfInterestFilter.h | MKPointOfInterestFilter (src/point_of_interest.rs) |
| MKPolygon | interface | MKPolygon.h | MKPolygon (src/overlay.rs) |
| MKPolyline | interface | MKPolyline.h | MKPolyline (src/overlay.rs) |
| MKReverseGeocodingRequest | interface | MKReverseGeocodingRequest.h | MKReverseGeocodingRequest (src/geocoder.rs) |
| MKFeatureVisibility | enum | MKTypes.h | MKFeatureVisibility (src/map_view.rs) |
| MKLocalSearchRegionPriority | enum | MKTypes.h | MKLocalSearchRegionPriority (src/local_search.rs) |
| MKMapType | enum | MKTypes.h | MKMapType (src/map_view.rs) |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| MKAnnotation | protocol | MKAnnotation.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKAnnotationCalloutInfoDidChangeNotification | constant | MKAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKAnnotationView | interface | MKAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKAnnotationViewCollisionMode | enum | MKAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKAnnotationViewDragState | enum | MKAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKAnnotationViewZPriority | typedef | MKAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKFeatureDisplayPriority | typedef | MKAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKCircleRenderer | interface | MKCircleRenderer.h | Renderer and drawing surface is not wrapped. |
| MKCompassButton | interface | MKCompassButton.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKGeoJSONDecoder | interface | MKGeoJSONSerialization.h | GeoJSON decoding APIs are not wrapped. |
| MKGeoJSONFeature | interface | MKGeoJSONSerialization.h | GeoJSON decoding APIs are not wrapped. |
| MKGeoJSONObject | protocol | MKGeoJSONSerialization.h | GeoJSON decoding APIs are not wrapped. |
| MKGeodesicPolyline | interface | MKGeodesicPolyline.h | Additional overlay/shape surface is not wrapped. |
| MKCoordinateRegionForMapRect | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapPointEqualToPoint | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapPointsPerMeterAtLatitude | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectContainsPoint | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectContainsRect | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectDivide | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectEqualToRect | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetHeight | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetMaxX | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetMaxY | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetMidX | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetMidY | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetMinX | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetMinY | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectGetWidth | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectInset | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectIntersection | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectIntersectsRect | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectIsEmpty | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectIsNull | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectNull | constant | MKGeometry.h | Global geometry constants are not surfaced by the crate. |
| MKMapRectOffset | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectRemainder | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectSpans180thMeridian | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectUnion | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapRectWorld | constant | MKGeometry.h | Global geometry constants are not surfaced by the crate. |
| MKMapSizeEqualToSize | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKMapSizeWorld | constant | MKGeometry.h | Global geometry constants are not surfaced by the crate. |
| MKMetersPerMapPointAtLatitude | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKStringFromMapPoint | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKStringFromMapRect | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKStringFromMapSize | function | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKZoomScale | typedef | MKGeometry.h | No equivalent Rust geometry helper is currently exposed. |
| MKGradientPolylineRenderer | interface | MKGradientPolylineRenderer.h | Renderer and drawing surface is not wrapped. |
| MKHybridMapConfiguration | interface | MKHybridMapConfiguration.h | Map configuration/camera APIs are not yet surfaced. |
| MKImageryMapConfiguration | interface | MKImageryMapConfiguration.h | Map configuration/camera APIs are not yet surfaced. |
| MKPointsOfInterestRequestMaxRadius | constant | MKLocalPointsOfInterestRequest.h | The request type is wrapped, but the max-radius constant is not exposed. |
| MKLocalSearchCompleter | interface | MKLocalSearchCompleter.h | Autocomplete/completer APIs are not wrapped. |
| MKLocalSearchCompleterDelegate | protocol | MKLocalSearchCompleter.h | Autocomplete/completer APIs are not wrapped. |
| MKLocalSearchCompleterResultType | enum | MKLocalSearchCompleter.h | Autocomplete/completer APIs are not wrapped. |
| MKLocalSearchCompletion | interface | MKLocalSearchCompleter.h | Autocomplete/completer APIs are not wrapped. |
| MKLookAroundBadgePosition | enum | MKLookAroundViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKLookAroundViewController | interface | MKLookAroundViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKLookAroundViewControllerDelegate | protocol | MKLookAroundViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMapCamera | interface | MKMapCamera.h | Map configuration/camera APIs are not yet surfaced. |
| MKMapCameraBoundary | interface | MKMapCameraBoundary.h | Map configuration/camera APIs are not yet surfaced. |
| MKMapCameraZoomDefault | constant | MKMapCameraZoomRange.h | Map configuration/camera APIs are not yet surfaced. |
| MKMapCameraZoomRange | interface | MKMapCameraZoomRange.h | Map configuration/camera APIs are not yet surfaced. |
| MKMapConfiguration | interface | MKMapConfiguration.h | Map configuration/camera APIs are not yet surfaced. |
| MKMapElevationStyle | enum | MKMapConfiguration.h | Map configuration/camera APIs are not yet surfaced. |
| MKLaunchOptionsCameraKey | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsDirectionsModeCycling | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsDirectionsModeDefault | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsDirectionsModeDriving | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsDirectionsModeKey | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsDirectionsModeTransit | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsDirectionsModeWalking | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsMapCenterKey | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsMapSpanKey | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsMapTypeKey | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKLaunchOptionsShowsTrafficKey | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKMapItemTypeIdentifier | constant | MKMapItem.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKMapItemAnnotation | interface | MKMapItemAnnotation.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKMapItemDetailViewController | interface | MKMapItemDetailViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMapItemDetailViewControllerDelegate | protocol | MKMapItemDetailViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMapItemIdentifier | interface | MKMapItemIdentifier.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKMapItemRequest | interface | MKMapItemRequest.h | Map-item launch, identifier, or request APIs are not exposed. |
| MKMapViewDefaultAnnotationViewReuseIdentifier | constant | MKMapView.h | Additional map-view constants or delegate helpers are not wrapped. |
| MKMapViewDefaultClusterAnnotationViewReuseIdentifier | constant | MKMapView.h | Additional map-view constants or delegate helpers are not wrapped. |
| MKMapViewDelegate | protocol | MKMapView.h | Delegate callbacks are not surfaced as Rust traits. |
| MKMarkerAnnotationView | interface | MKMarkerAnnotationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMultiPoint | interface | MKMultiPoint.h | Additional overlay/shape surface is not wrapped. |
| MKMultiPolygon | interface | MKMultiPolygon.h | Additional overlay/shape surface is not wrapped. |
| MKMultiPolygonRenderer | interface | MKMultiPolygonRenderer.h | Renderer and drawing surface is not wrapped. |
| MKMultiPolyline | interface | MKMultiPolyline.h | Additional overlay/shape surface is not wrapped. |
| MKMultiPolylineRenderer | interface | MKMultiPolylineRenderer.h | Renderer and drawing surface is not wrapped. |
| MKOverlay | protocol | MKOverlay.h | Additional overlay/shape surface is not wrapped. |
| MKOverlayPathRenderer | interface | MKOverlayPathRenderer.h | Renderer and drawing surface is not wrapped. |
| MKOverlayRenderer | interface | MKOverlayRenderer.h | Renderer and drawing surface is not wrapped. |
| MKRoadWidthAtZoomScale | function | MKOverlayRenderer.h | Renderer and drawing surface is not wrapped. |
| MKPitchControl | interface | MKPitchControl.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKPointOfInterestCategoryATM | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryAmusementPark | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryAnimalService | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryAquarium | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryAutomotiveRepair | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBakery | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBank | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBaseball | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBasketball | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBeach | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBeauty | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBowling | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryBrewery | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryCampground | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryCarRental | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryCastle | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryConventionCenter | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryDistillery | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryEVCharger | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryFairground | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryFireStation | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryFishing | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryFitnessCenter | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryFoodMarket | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryFortress | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryGasStation | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryGoKart | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryGolf | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryHiking | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryHospital | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryKayaking | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryLandmark | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryLaundry | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryMailbox | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryMarina | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryMiniGolf | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryMovieTheater | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryMusicVenue | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryNationalMonument | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryNationalPark | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryNightlife | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryParking | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryPharmacy | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryPlanetarium | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryPolice | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryPostOffice | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryPublicTransport | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryRVPark | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryRestroom | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryRockClimbing | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySkatePark | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySkating | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySkiing | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySoccer | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySpa | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryStadium | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySurfing | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategorySwimming | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryTennis | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryTheater | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryVolleyball | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryWinery | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPointOfInterestCategoryZoo | constant | MKPointOfInterestCategory.h | Only 10 convenience category constructors are exposed; this specific category constant is missing. |
| MKPolygonRenderer | interface | MKPolygonRenderer.h | Renderer and drawing surface is not wrapped. |
| MKPolylineRenderer | interface | MKPolylineRenderer.h | Renderer and drawing surface is not wrapped. |
| MKMapItemDetailSelectionAccessoryCalloutStyle | enum | MKSelectionAccessory.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMapItemDetailSelectionAccessoryPresentationStyle | interface | MKSelectionAccessory.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKSelectionAccessory | interface | MKSelectionAccessory.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKShape | interface | MKShape.h | Additional overlay/shape surface is not wrapped. |
| MKStandardMapConfiguration | interface | MKStandardMapConfiguration.h | Map configuration/camera APIs are not yet surfaced. |
| MKStandardMapEmphasisStyle | enum | MKStandardMapConfiguration.h | Map configuration/camera APIs are not yet surfaced. |
| MKTileOverlay | interface | MKTileOverlay.h | Additional overlay/shape surface is not wrapped. |
| MKTileOverlayPath | struct | MKTileOverlay.h | Additional overlay/shape surface is not wrapped. |
| MKTileOverlayRenderer | interface | MKTileOverlayRenderer.h | Renderer and drawing surface is not wrapped. |
| MKErrorCode | enum | MKTypes.h | Errors are surfaced as generic NSErrorInfo rather than the MKErrorCode enum. |
| MKErrorDomain | constant | MKTypes.h | Errors are surfaced as generic NSErrorInfo rather than the MKErrorDomain export. |
| MKUserLocation | interface | MKUserLocation.h | No public Rust wrapper currently exposes this MapKit symbol. |
| MKUserLocationView | interface | MKUserLocationView.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKZoomControl | interface | MKZoomControl.h | UI/delegate/AppKit surface is not wrapped by the crate. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| MKSearchCompletionFilterType | enum | MKLocalSearchCompleter.h | Deprecated on macOS 10.15; superseded by MKLocalSearchCompleterResultType. | __attribute__((availability(ios,introduced=9.3,deprecated=13.0,message="Use MKLocalSearchCompleterResultType"))), __attribute__((availability(macos,introduced=10.11.4,deprecated=10.15,message="Use MKLocalSearchCompleterResultType"))), __attribute__((availability(tvos,introduced=9.2,deprecated=13.0,message="Use MKLocalSearchCompleterResultType"))), __attribute__((availability(watchos,unavailable))) |
| MKPinAnnotationColor | enum | MKPinAnnotationView.h | Deprecated on macOS 10.13; superseded by MKMarkerAnnotationView. | __attribute__((availability(macosx,introduced=10_9,deprecated=10_11,message="" "Use MKPinAnnotationView's pinTintColor instead"))), __attribute__((availability(tvos,unavailable))), __attribute__((availability(watchos,unavailable))) |
| MKPinAnnotationView | interface | MKPinAnnotationView.h | Deprecated on macOS 10.13; superseded by MKMarkerAnnotationView. | __attribute__((availability(macos,introduced=10.9,deprecated=13.0,replacement="MKMarkerAnnotationView"))), __attribute__((availability(ios,introduced=3.0,deprecated=16.0,replacement="MKMarkerAnnotationView"))), __attribute__((availability(tvos,introduced=9.2,deprecated=16.0,replacement="MKMarkerAnnotationView"))), __attribute__((availability(watchos,unavailable))) |
| MKPlacemark | interface | MKPlacemark.h | Deprecated on macOS 26.0; the crate still keeps a compatibility wrapper. | __attribute__((availability(ios,introduced=3.0,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(visionos,introduced=1.0,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(tvos,introduced=9.2,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(macos,introduced=10.9,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(watchos,introduced=2.0,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))) |

