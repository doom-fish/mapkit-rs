# mapkit coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 243
VERIFIED: 241
GAPS: 0
EXEMPT: 2
COVERAGE_PCT: 100.0%

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
| MKMapItemAnnotation | interface | MKMapItemAnnotation.h | MKMapItemAnnotation (src/annotation.rs) |
| MKMapView | interface | MKMapView.h | MKMapView (src/map_view.rs) |
| MKMapViewDefaultAnnotationViewReuseIdentifier | constant | MKMapView.h | MKMapView::default_annotation_view_reuse_identifier (src/map_view.rs) |
| MKMapViewDefaultClusterAnnotationViewReuseIdentifier | constant | MKMapView.h | MKMapView::default_cluster_annotation_view_reuse_identifier (src/map_view.rs) |
| MKMarkerAnnotationView | interface | MKMarkerAnnotationView.h | MKMarkerAnnotationView (src/annotation_view.rs) |
| MKMetersBetweenMapPoints | function | MKGeometry.h | MKMapPoint::distance_to (src/geometry.rs) |
| MKMultiPoint | interface | MKMultiPoint.h | MKMultiPoint (src/overlay.rs) |
| MKMultiPolygon | interface | MKMultiPolygon.h | MKMultiPolygon (src/overlay.rs) |
| MKMultiPolygonRenderer | interface | MKMultiPolygonRenderer.h | MKMultiPolygonRenderer (src/overlay_renderer.rs) |
| MKMultiPolyline | interface | MKMultiPolyline.h | MKMultiPolyline (src/overlay.rs) |
| MKMultiPolylineRenderer | interface | MKMultiPolylineRenderer.h | MKMultiPolylineRenderer (src/overlay_renderer.rs) |
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
| MKPinAnnotationColor | enum | MKPinAnnotationView.h | MKPinAnnotationColor (src/annotation_view.rs) |
| MKPinAnnotationView | interface | MKPinAnnotationView.h | MKPinAnnotationView (src/annotation_view.rs) |
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
| MKUserLocation | interface | MKUserLocation.h | MKUserLocation (src/annotation.rs) |
| MKUserLocationView | interface | MKUserLocationView.h | MKUserLocationView (src/annotation_view.rs) |
| MKZoomScale | typedef | MKGeometry.h | MKZoomScale (src/overlay_renderer.rs) |

| MKCompassButton | interface | MKCompassButton.h | MKCompassButton (src/controls.rs) |
| MKGeoJSONDecoder | interface | MKGeoJSONSerialization.h | MKGeoJSONDecoder (src/geojson.rs) |
| MKGeoJSONFeature | interface | MKGeoJSONSerialization.h | MKGeoJSONFeature (src/geojson.rs) |
| MKGeoJSONObject | protocol | MKGeoJSONSerialization.h | MKGeoJSONObject + MKGeoJSONObjectValue (src/geojson.rs) |
| MKCoordinateRegionForMapRect | function | MKGeometry.h | MKCoordinateRegion::from_map_rect (src/geometry.rs) |
| MKMapPointEqualToPoint | function | MKGeometry.h | MKMapPoint::equal_to (src/geometry.rs) |
| MKMapPointsPerMeterAtLatitude | function | MKGeometry.h | mk_map_points_per_meter_at_latitude (src/geometry.rs) |
| MKMapRectContainsPoint | function | MKGeometry.h | MKMapRect::contains_point (src/geometry.rs) |
| MKMapRectContainsRect | function | MKGeometry.h | MKMapRect::contains_rect (src/geometry.rs) |
| MKMapRectDivide | function | MKGeometry.h | MKMapRect::divide + MKMapRectDivision (src/geometry.rs) |
| MKMapRectEqualToRect | function | MKGeometry.h | MKMapRect::equal_to (src/geometry.rs) |
| MKMapRectGetHeight | function | MKGeometry.h | MKMapRect::height (src/geometry.rs) |
| MKMapRectGetMaxX | function | MKGeometry.h | MKMapRect::max_x (src/geometry.rs) |
| MKMapRectGetMaxY | function | MKGeometry.h | MKMapRect::max_y (src/geometry.rs) |
| MKMapRectGetMidX | function | MKGeometry.h | MKMapRect::mid_x (src/geometry.rs) |
| MKMapRectGetMidY | function | MKGeometry.h | MKMapRect::mid_y (src/geometry.rs) |
| MKMapRectGetMinX | function | MKGeometry.h | MKMapRect::min_x (src/geometry.rs) |
| MKMapRectGetMinY | function | MKGeometry.h | MKMapRect::min_y (src/geometry.rs) |
| MKMapRectGetWidth | function | MKGeometry.h | MKMapRect::width (src/geometry.rs) |
| MKMapRectInset | function | MKGeometry.h | MKMapRect::inset (src/geometry.rs) |
| MKMapRectIntersection | function | MKGeometry.h | MKMapRect::intersection (src/geometry.rs) |
| MKMapRectIntersectsRect | function | MKGeometry.h | MKMapRect::intersects_rect (src/geometry.rs) |
| MKMapRectIsEmpty | function | MKGeometry.h | MKMapRect::is_empty (src/geometry.rs) |
| MKMapRectIsNull | function | MKGeometry.h | MKMapRect::is_null (src/geometry.rs) |
| MKMapRectNull | constant | MKGeometry.h | MKMapRect::null (src/geometry.rs) |
| MKMapRectOffset | function | MKGeometry.h | MKMapRect::offset (src/geometry.rs) |
| MKMapRectRemainder | function | MKGeometry.h | MKMapRect::remainder (src/geometry.rs) |
| MKMapRectSpans180thMeridian | function | MKGeometry.h | MKMapRect::spans_180th_meridian (src/geometry.rs) |
| MKMapRectUnion | function | MKGeometry.h | MKMapRect::union (src/geometry.rs) |
| MKMapRectWorld | constant | MKGeometry.h | MKMapRect::world (src/geometry.rs) |
| MKMapSizeEqualToSize | function | MKGeometry.h | MKMapSize::equal_to (src/geometry.rs) |
| MKMapSizeWorld | constant | MKGeometry.h | MKMapSize::world (src/geometry.rs) |
| MKMetersPerMapPointAtLatitude | function | MKGeometry.h | mk_meters_per_map_point_at_latitude (src/geometry.rs) |
| MKStringFromMapPoint | function | MKGeometry.h | mk_string_from_map_point / MKMapPoint::string_representation (src/geometry.rs) |
| MKStringFromMapRect | function | MKGeometry.h | mk_string_from_map_rect / MKMapRect::string_representation (src/geometry.rs) |
| MKStringFromMapSize | function | MKGeometry.h | mk_string_from_map_size / MKMapSize::string_representation (src/geometry.rs) |
| MKLookAroundBadgePosition | enum | MKLookAroundViewController.h | MKLookAroundBadgePosition (src/look_around_view_controller.rs) |
| MKLookAroundViewController | interface | MKLookAroundViewController.h | MKLookAroundViewController (src/look_around_view_controller.rs) |
| MKLookAroundViewControllerDelegate | protocol | MKLookAroundViewController.h | MKLookAroundViewControllerDelegate (src/look_around_view_controller.rs) |
| MKLaunchOptionsCameraKey | constant | MKMapItem.h | MKMapItem::launch_options_camera_key (src/map_item.rs) |
| MKLaunchOptionsDirectionsModeCycling | constant | MKMapItem.h | MKMapItem::launch_options_directions_mode_cycling (src/map_item.rs) |
| MKLaunchOptionsDirectionsModeDefault | constant | MKMapItem.h | MKMapItem::launch_options_directions_mode_default (src/map_item.rs) |
| MKLaunchOptionsDirectionsModeDriving | constant | MKMapItem.h | MKMapItem::launch_options_directions_mode_driving (src/map_item.rs) |
| MKLaunchOptionsDirectionsModeKey | constant | MKMapItem.h | MKMapItem::launch_options_directions_mode_key (src/map_item.rs) |
| MKLaunchOptionsDirectionsModeTransit | constant | MKMapItem.h | MKMapItem::launch_options_directions_mode_transit (src/map_item.rs) |
| MKLaunchOptionsDirectionsModeWalking | constant | MKMapItem.h | MKMapItem::launch_options_directions_mode_walking (src/map_item.rs) |
| MKLaunchOptionsMapCenterKey | constant | MKMapItem.h | MKMapItem::launch_options_map_center_key (src/map_item.rs) |
| MKLaunchOptionsMapSpanKey | constant | MKMapItem.h | MKMapItem::launch_options_map_span_key (src/map_item.rs) |
| MKLaunchOptionsMapTypeKey | constant | MKMapItem.h | MKMapItem::launch_options_map_type_key (src/map_item.rs) |
| MKLaunchOptionsShowsTrafficKey | constant | MKMapItem.h | MKMapItem::launch_options_shows_traffic_key (src/map_item.rs) |
| MKMapItemTypeIdentifier | constant | MKMapItem.h | MKMapItem::type_identifier (src/map_item.rs) |
| MKMapItemDetailViewController | interface | MKMapItemDetailViewController.h | MKMapItemDetailViewController (src/map_item_detail_view_controller.rs) |
| MKMapItemDetailViewControllerDelegate | protocol | MKMapItemDetailViewController.h | MKMapItemDetailViewControllerDelegate (src/map_item_detail_view_controller.rs) |
| MKMapItemIdentifier | interface | MKMapItemIdentifier.h | MKMapItemIdentifier (src/map_item.rs) |
| MKMapItemRequest | interface | MKMapItemRequest.h | MKMapItemRequest (src/map_item.rs) |
| MKMapViewDelegate | protocol | MKMapView.h | MKMapViewDelegate (src/map_view_delegate.rs) |
| MKPitchControl | interface | MKPitchControl.h | MKPitchControl (src/controls.rs) |
| MKMapItemDetailSelectionAccessoryCalloutStyle | enum | MKSelectionAccessory.h | MKMapItemDetailSelectionAccessoryCalloutStyle (src/selection_accessory.rs) |
| MKMapItemDetailSelectionAccessoryPresentationStyle | interface | MKSelectionAccessory.h | MKMapItemDetailSelectionAccessoryPresentationStyle (src/selection_accessory.rs) |
| MKSelectionAccessory | interface | MKSelectionAccessory.h | MKSelectionAccessory (src/selection_accessory.rs) |
| MKErrorCode | enum | MKTypes.h | MKErrorCode (src/error.rs) |
| MKErrorDomain | constant | MKTypes.h | mk_error_domain() (src/error.rs) |
| MKZoomControl | interface | MKZoomControl.h | MKZoomControl (src/controls.rs) |

## 🔴 GAPS
_None._

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| MKSearchCompletionFilterType | enum | MKLocalSearchCompleter.h | Deprecated on macOS 10.15; superseded by MKLocalSearchCompleterResultType. | __attribute__((availability(ios,introduced=9.3,deprecated=13.0,message="Use MKLocalSearchCompleterResultType"))), __attribute__((availability(macos,introduced=10.11.4,deprecated=10.15,message="Use MKLocalSearchCompleterResultType"))), __attribute__((availability(tvos,introduced=9.2,deprecated=13.0,message="Use MKLocalSearchCompleterResultType"))), __attribute__((availability(watchos,unavailable))) |
| MKPlacemark | interface | MKPlacemark.h | Deprecated on macOS 26.0; the crate still keeps a compatibility wrapper. | __attribute__((availability(ios,introduced=3.0,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(visionos,introduced=1.0,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(tvos,introduced=9.2,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(macos,introduced=10.9,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))), __attribute__((availability(watchos,introduced=2.0,deprecated=26.0,message="Use MKMapItem's location, address and addressRepresentations properties instead. Use MKAddressRepresentations for formatted address strings for MapKit provided MKMapItems"))) |

