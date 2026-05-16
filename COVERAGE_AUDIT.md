# mapkit coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 243
VERIFIED: 167
GAPS: 72
EXEMPT: 4
COVERAGE_PCT: 68.7%

This audit counts named top-level macOS declarations from MapKit.framework headers (interfaces, protocols, typedefs/enums/structs, exported constants, and top-level functions). Objective-C categories were not counted as standalone symbols.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| MKAddress | interface | MKAddress.h | MKAddress (src/address.rs) |
| MKAddressFilter | interface | MKAddressFilter.h | MKAddressFilter (src/address.rs) |
| MKAddressFilterOption | enum | MKAddressFilter.h | MKAddressFilterOption (src/address.rs) |
| MKAddressRepresentations | interface | MKAddressRepresentations.h | MKAddressRepresentations (src/address.rs) |
| MKAddressRepresentationsContextStyle | enum | MKAddressRepresentations.h | MKAddressRepresentationsContextStyle (src/address.rs) |
| MKAnnotation | protocol | MKAnnotation.h | MKAnnotation (src/annotation_view.rs) |
| MKAnnotationCalloutInfoDidChangeNotification | constant | MKAnnotationView.h | MKAnnotationView::callout_info_did_change_notification (src/annotation_view.rs) |
| MKAnnotationView | interface | MKAnnotationView.h | MKAnnotationView (src/annotation_view.rs) |
| MKAnnotationViewCollisionMode | enum | MKAnnotationView.h | MKAnnotationViewCollisionMode (src/annotation_view.rs) |
| MKAnnotationViewDragState | enum | MKAnnotationView.h | MKAnnotationViewDragState (src/annotation_view.rs) |
| MKAnnotationViewZPriority | typedef | MKAnnotationView.h | MKAnnotationViewZPriority (src/annotation_view.rs) |
| MKCircle | interface | MKCircle.h | MKCircle (src/overlay.rs) |
| MKCircleRenderer | interface | MKCircleRenderer.h | MKCircleRenderer (src/overlay_renderer.rs) |
| MKClusterAnnotation | interface | MKClusterAnnotation.h | MKClusterAnnotation (src/cluster_annotation.rs) |
| MKCoordinateForMapPoint | function | MKGeometry.h | MKMapPoint::coordinate (src/geometry.rs) |
| MKCoordinateRegion | struct | MKGeometry.h | MKCoordinateRegion (src/geometry.rs) |
| MKCoordinateRegionMake | function | MKGeometry.h | MKCoordinateRegion::new (src/geometry.rs) |
| MKCoordinateRegionMakeWithDistance | function | MKGeometry.h | MKCoordinateRegion::with_distance (src/geometry.rs) |
| MKCoordinateSpan | struct | MKGeometry.h | MKCoordinateSpan (src/geometry.rs) |
| MKCoordinateSpanMake | function | MKGeometry.h | MKCoordinateSpan::new (src/geometry.rs) |
| MKDirections | interface | MKDirections.h | MKDirections (src/directions.rs) |
| MKDirectionsRequest | interface | MKDirectionsRequest.h | MKDirectionsRequest (src/directions.rs) |
| MKDirectionsResponse | interface | MKDirectionsResponse.h | MKDirectionsResponse (src/directions.rs) |
| MKDirectionsRoutePreference | enum | MKDirectionsRequest.h | MKDirectionsRoutePreference (src/directions.rs) |
| MKDirectionsTransportType | enum | MKDirectionsTypes.h | MKDirectionsTransportType (src/directions.rs) |
| MKDistanceFormatter | interface | MKDistanceFormatter.h | MKDistanceFormatter (src/distance_formatter.rs) |
| MKETAResponse | interface | MKDirectionsResponse.h | MKETAResponse (src/directions.rs) |
| MKFeatureDisplayPriority | typedef | MKAnnotationView.h | MKFeatureDisplayPriority (src/annotation_view.rs) |
| MKFeatureVisibility | enum | MKTypes.h | MKFeatureVisibility (src/map_view.rs) |
| MKGeocodingRequest | interface | MKGeocodingRequest.h | MKGeocodingRequest (src/geocoder.rs) |
| MKGeodesicPolyline | interface | MKGeodesicPolyline.h | MKGeodesicPolyline (src/overlay.rs) |
| MKGradientPolylineRenderer | interface | MKGradientPolylineRenderer.h | MKGradientPolylineRenderer (src/overlay_renderer.rs) |
| MKHybridMapConfiguration | interface | MKHybridMapConfiguration.h | MKHybridMapConfiguration (src/configuration.rs) |
| MKImageryMapConfiguration | interface | MKImageryMapConfiguration.h | MKImageryMapConfiguration (src/configuration.rs) |
| MKLocalPointsOfInterestRequest | interface | MKLocalPointsOfInterestRequest.h | MKLocalPointsOfInterestRequest (src/point_of_interest.rs) |
| MKLocalSearch | interface | MKLocalSearch.h | MKLocalSearch (src/local_search.rs) |
| MKLocalSearchCompleter | interface | MKLocalSearchCompleter.h | MKLocalSearchCompleter (src/local_search_completer.rs) |
| MKLocalSearchCompleterDelegate | protocol | MKLocalSearchCompleter.h | MKLocalSearchCompleterDelegate (src/local_search_completer.rs) |
| MKLocalSearchCompleterResultType | enum | MKLocalSearchCompleter.h | MKLocalSearchCompleterResultType (src/local_search_completer.rs) |
| MKLocalSearchCompletion | interface | MKLocalSearchCompleter.h | MKLocalSearchCompletion (src/local_search_completer.rs) |
| MKLocalSearchRegionPriority | enum | MKTypes.h | MKLocalSearchRegionPriority (src/local_search.rs) |
| MKLocalSearchRequest | interface | MKLocalSearchRequest.h | MKLocalSearchRequest (src/local_search.rs) |
| MKLocalSearchResponse | interface | MKLocalSearchResponse.h | MKLocalSearchResponse (src/local_search.rs) |
| MKLocalSearchResultType | enum | MKLocalSearchRequest.h | MKLocalSearchResultType (src/local_search.rs) |
| MKLookAroundScene | interface | MKLookAroundScene.h | MKLookAroundScene (src/look_around.rs) |
| MKLookAroundSceneRequest | interface | MKLookAroundSceneRequest.h | MKLookAroundSceneRequest (src/look_around.rs) |
| MKLookAroundSnapshot | interface | MKLookAroundSnapshot.h | MKLookAroundSnapshot (src/look_around.rs) |
| MKLookAroundSnapshotOptions | interface | MKLookAroundSnapshotOptions.h | MKLookAroundSnapshotOptions (src/look_around.rs) |
| MKLookAroundSnapshotter | interface | MKLookAroundSnapshotter.h | MKLookAroundSnapshotter (src/look_around.rs) |
| MKMapCamera | interface | MKMapCamera.h | MKMapCamera (src/configuration.rs) |
| MKMapCameraBoundary | interface | MKMapCameraBoundary.h | MKMapCameraBoundary (src/configuration.rs) |
| MKMapCameraZoomDefault | constant | MKMapCameraZoomRange.h | MKMapCameraZoomRange::default_distance (src/configuration.rs) |
| MKMapCameraZoomRange | interface | MKMapCameraZoomRange.h | MKMapCameraZoomRange (src/configuration.rs) |
| MKMapConfiguration | interface | MKMapConfiguration.h | MKMapConfiguration (src/configuration.rs) |
| MKMapElevationStyle | enum | MKMapConfiguration.h | MKMapElevationStyle (src/configuration.rs) |
| MKMapItem | interface | MKMapItem.h | MKMapItem (src/map_item.rs) |
| MKMapPoint | struct | MKGeometry.h | MKMapPoint (src/geometry.rs) |
| MKMapPointForCoordinate | function | MKGeometry.h | MKMapPoint::from_coordinate (src/geometry.rs) |
| MKMapPointMake | function | MKGeometry.h | MKMapPoint::new (src/geometry.rs) |
| MKMapRect | struct | MKGeometry.h | MKMapRect (src/geometry.rs) |
| MKMapRectMake | function | MKGeometry.h | MKMapRect::new (src/geometry.rs) |
| MKMapSize | struct | MKGeometry.h | MKMapSize (src/geometry.rs) |
| MKMapSizeMake | function | MKGeometry.h | MKMapSize::new (src/geometry.rs) |
| MKMapSnapshot | interface | MKMapSnapshot.h | MKMapSnapshot (src/snapshotter.rs) |
| MKMapSnapshotOptions | interface | MKMapSnapshotOptions.h | MKMapSnapshotOptions (src/snapshotter.rs) |
| MKMapSnapshotter | interface | MKMapSnapshotter.h | MKMapSnapshotter (src/snapshotter.rs) |
| MKMapType | enum | MKTypes.h | MKMapType (src/map_view.rs) |
| MKMapView | interface | MKMapView.h | MKMapView (src/map_view.rs) |
| MKMarkerAnnotationView | interface | MKMarkerAnnotationView.h | MKMarkerAnnotationView (src/annotation_view.rs) |
| MKMetersBetweenMapPoints | function | MKGeometry.h | MKMapPoint::distance_to (src/geometry.rs) |
| MKMultiPoint | interface | MKMultiPoint.h | MKMultiPoint (src/overlay.rs) |
| MKOverlay | protocol | MKOverlay.h | MKOverlay (src/overlay.rs) |
| MKOverlayLevel | enum | MKMapView.h | MKOverlayLevel (src/overlay.rs) |
| MKOverlayPathRenderer | interface | MKOverlayPathRenderer.h | MKOverlayPathRenderer (src/overlay_renderer.rs) |
| MKOverlayRenderer | interface | MKOverlayRenderer.h | MKOverlayRenderer (src/overlay_renderer.rs) |
| MKPointAnnotation | interface | MKPointAnnotation.h | MKPointAnnotation (src/annotation.rs) |
| MKPointOfInterestCategory | typedef | MKPointOfInterestCategory.h | MKPointOfInterestCategory (src/point_of_interest.rs) |
| MKPointOfInterestCategoryATM | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::atm (src/point_of_interest.rs) |
| MKPointOfInterestCategoryAirport | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::airport (src/point_of_interest.rs) |
| MKPointOfInterestCategoryAmusementPark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::amusement_park (src/point_of_interest.rs) |
| MKPointOfInterestCategoryAnimalService | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::animal_service (src/point_of_interest.rs) |
| MKPointOfInterestCategoryAquarium | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::aquarium (src/point_of_interest.rs) |
| MKPointOfInterestCategoryAutomotiveRepair | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::automotive_repair (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBakery | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::bakery (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBank | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::bank (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBaseball | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::baseball (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBasketball | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::basketball (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBeach | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::beach (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBeauty | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::beauty (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBowling | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::bowling (src/point_of_interest.rs) |
| MKPointOfInterestCategoryBrewery | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::brewery (src/point_of_interest.rs) |
| MKPointOfInterestCategoryCafe | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::cafe (src/point_of_interest.rs) |
| MKPointOfInterestCategoryCampground | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::campground (src/point_of_interest.rs) |
| MKPointOfInterestCategoryCarRental | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::car_rental (src/point_of_interest.rs) |
| MKPointOfInterestCategoryCastle | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::castle (src/point_of_interest.rs) |
| MKPointOfInterestCategoryConventionCenter | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::convention_center (src/point_of_interest.rs) |
| MKPointOfInterestCategoryDistillery | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::distillery (src/point_of_interest.rs) |
| MKPointOfInterestCategoryEVCharger | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::ev_charger (src/point_of_interest.rs) |
| MKPointOfInterestCategoryFairground | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::fairground (src/point_of_interest.rs) |
| MKPointOfInterestCategoryFireStation | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::fire_station (src/point_of_interest.rs) |
| MKPointOfInterestCategoryFishing | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::fishing (src/point_of_interest.rs) |
| MKPointOfInterestCategoryFitnessCenter | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::fitness_center (src/point_of_interest.rs) |
| MKPointOfInterestCategoryFoodMarket | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::food_market (src/point_of_interest.rs) |
| MKPointOfInterestCategoryFortress | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::fortress (src/point_of_interest.rs) |
| MKPointOfInterestCategoryGasStation | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::gas_station (src/point_of_interest.rs) |
| MKPointOfInterestCategoryGoKart | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::go_kart (src/point_of_interest.rs) |
| MKPointOfInterestCategoryGolf | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::golf (src/point_of_interest.rs) |
| MKPointOfInterestCategoryHiking | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::hiking (src/point_of_interest.rs) |
| MKPointOfInterestCategoryHospital | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::hospital (src/point_of_interest.rs) |
| MKPointOfInterestCategoryHotel | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::hotel (src/point_of_interest.rs) |
| MKPointOfInterestCategoryKayaking | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::kayaking (src/point_of_interest.rs) |
| MKPointOfInterestCategoryLandmark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::landmark (src/point_of_interest.rs) |
| MKPointOfInterestCategoryLaundry | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::laundry (src/point_of_interest.rs) |
| MKPointOfInterestCategoryLibrary | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::library (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMailbox | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::mailbox (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMarina | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::marina (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMiniGolf | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::mini_golf (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMovieTheater | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::movie_theater (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMuseum | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::museum (src/point_of_interest.rs) |
| MKPointOfInterestCategoryMusicVenue | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::music_venue (src/point_of_interest.rs) |
| MKPointOfInterestCategoryNationalMonument | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::national_monument (src/point_of_interest.rs) |
| MKPointOfInterestCategoryNationalPark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::national_park (src/point_of_interest.rs) |
| MKPointOfInterestCategoryNightlife | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::nightlife (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::park (src/point_of_interest.rs) |
| MKPointOfInterestCategoryParking | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::parking (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPharmacy | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::pharmacy (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPlanetarium | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::planetarium (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPolice | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::police (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPostOffice | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::post_office (src/point_of_interest.rs) |
| MKPointOfInterestCategoryPublicTransport | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::public_transport (src/point_of_interest.rs) |
| MKPointOfInterestCategoryRVPark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::rv_park (src/point_of_interest.rs) |
| MKPointOfInterestCategoryRestaurant | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::restaurant (src/point_of_interest.rs) |
| MKPointOfInterestCategoryRestroom | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::restroom (src/point_of_interest.rs) |
| MKPointOfInterestCategoryRockClimbing | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::rock_climbing (src/point_of_interest.rs) |
| MKPointOfInterestCategorySchool | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::school (src/point_of_interest.rs) |
| MKPointOfInterestCategorySkatePark | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::skate_park (src/point_of_interest.rs) |
| MKPointOfInterestCategorySkating | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::skating (src/point_of_interest.rs) |
| MKPointOfInterestCategorySkiing | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::skiing (src/point_of_interest.rs) |
| MKPointOfInterestCategorySoccer | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::soccer (src/point_of_interest.rs) |
| MKPointOfInterestCategorySpa | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::spa (src/point_of_interest.rs) |
| MKPointOfInterestCategoryStadium | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::stadium (src/point_of_interest.rs) |
| MKPointOfInterestCategoryStore | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::store (src/point_of_interest.rs) |
| MKPointOfInterestCategorySurfing | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::surfing (src/point_of_interest.rs) |
| MKPointOfInterestCategorySwimming | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::swimming (src/point_of_interest.rs) |
| MKPointOfInterestCategoryTennis | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::tennis (src/point_of_interest.rs) |
| MKPointOfInterestCategoryTheater | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::theater (src/point_of_interest.rs) |
| MKPointOfInterestCategoryUniversity | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::university (src/point_of_interest.rs) |
| MKPointOfInterestCategoryVolleyball | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::volleyball (src/point_of_interest.rs) |
| MKPointOfInterestCategoryWinery | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::winery (src/point_of_interest.rs) |
| MKPointOfInterestCategoryZoo | constant | MKPointOfInterestCategory.h | MKPointOfInterestCategory::zoo (src/point_of_interest.rs) |
| MKPointOfInterestFilter | interface | MKPointOfInterestFilter.h | MKPointOfInterestFilter (src/point_of_interest.rs) |
| MKPointsOfInterestRequestMaxRadius | constant | MKLocalPointsOfInterestRequest.h | MKLocalPointsOfInterestRequest::max_radius (src/point_of_interest.rs) |
| MKPolygon | interface | MKPolygon.h | MKPolygon (src/overlay.rs) |
| MKPolygonRenderer | interface | MKPolygonRenderer.h | MKPolygonRenderer (src/overlay_renderer.rs) |
| MKPolyline | interface | MKPolyline.h | MKPolyline (src/overlay.rs) |
| MKPolylineRenderer | interface | MKPolylineRenderer.h | MKPolylineRenderer (src/overlay_renderer.rs) |
| MKReverseGeocodingRequest | interface | MKReverseGeocodingRequest.h | MKReverseGeocodingRequest (src/geocoder.rs) |
| MKRoadWidthAtZoomScale | function | MKOverlayRenderer.h | mk_road_width_at_zoom_scale (src/overlay_renderer.rs) |
| MKRoute | interface | MKDirectionsResponse.h | MKRoute (src/directions.rs) |
| MKRouteStep | interface | MKDirectionsResponse.h | MKRouteStep (src/directions.rs) |
| MKShape | interface | MKShape.h | MKShape (src/overlay.rs) |
| MKStandardMapConfiguration | interface | MKStandardMapConfiguration.h | MKStandardMapConfiguration (src/configuration.rs) |
| MKStandardMapEmphasisStyle | enum | MKStandardMapConfiguration.h | MKStandardMapEmphasisStyle (src/configuration.rs) |
| MKTileOverlay | interface | MKTileOverlay.h | MKTileOverlay (src/overlay.rs) |
| MKTileOverlayPath | struct | MKTileOverlay.h | MKTileOverlayPath (src/overlay.rs) |
| MKTileOverlayRenderer | interface | MKTileOverlayRenderer.h | MKTileOverlayRenderer (src/overlay_renderer.rs) |
| MKZoomScale | typedef | MKGeometry.h | MKZoomScale (src/overlay_renderer.rs) |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| MKCompassButton | interface | MKCompassButton.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKGeoJSONDecoder | interface | MKGeoJSONSerialization.h | GeoJSON decoding APIs are not wrapped. |
| MKGeoJSONFeature | interface | MKGeoJSONSerialization.h | GeoJSON decoding APIs are not wrapped. |
| MKGeoJSONObject | protocol | MKGeoJSONSerialization.h | GeoJSON decoding APIs are not wrapped. |
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
| MKLookAroundBadgePosition | enum | MKLookAroundViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKLookAroundViewController | interface | MKLookAroundViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKLookAroundViewControllerDelegate | protocol | MKLookAroundViewController.h | UI/delegate/AppKit surface is not wrapped by the crate. |
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
| MKMultiPolygon | interface | MKMultiPolygon.h | Additional overlay/shape surface is not wrapped. |
| MKMultiPolygonRenderer | interface | MKMultiPolygonRenderer.h | Renderer and drawing surface is not wrapped. |
| MKMultiPolyline | interface | MKMultiPolyline.h | Additional overlay/shape surface is not wrapped. |
| MKMultiPolylineRenderer | interface | MKMultiPolylineRenderer.h | Renderer and drawing surface is not wrapped. |
| MKPitchControl | interface | MKPitchControl.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMapItemDetailSelectionAccessoryCalloutStyle | enum | MKSelectionAccessory.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKMapItemDetailSelectionAccessoryPresentationStyle | interface | MKSelectionAccessory.h | UI/delegate/AppKit surface is not wrapped by the crate. |
| MKSelectionAccessory | interface | MKSelectionAccessory.h | UI/delegate/AppKit surface is not wrapped by the crate. |
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

