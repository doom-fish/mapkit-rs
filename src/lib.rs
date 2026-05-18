#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::cargo_common_metadata,
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::return_self_not_must_use,
    clippy::single_option_map,
    clippy::struct_excessive_bools
)]

/// MapKit wrappers for `MKAddressFilterOption`, `MKAddressFilterMode`, and `MKAddressFilter`.
pub mod address;
/// MapKit wrappers for `MKAnnotation`, `MKCoordinate`, and `MKMapItem`.
pub mod annotation;
/// MapKit wrappers for `MKPointAnnotation`, `MKClusterAnnotation`, and `MKCoordinate`.
pub mod annotation_view;
/// MapKit wrappers for `MKPointAnnotation`, `MKCoordinate`, and `MKClusterAnnotationState`.
pub mod cluster_annotation;
/// MapKit wrappers for `MKCoordinate`, `MKCoordinateRegion`, and `MKMapRect`.
pub mod configuration;
/// MapKit wrappers for `MKFeatureVisibility`, `MKMapView`, and `MKCompassButton`.
pub mod controls;
/// MapKit wrappers for `MKMapItem`, `MKDirectionsTransportType`, and `MKDirectionsRoutePreference`.
pub mod directions;
/// MapKit wrappers for `MKDistanceFormatterUnits`, `MKDistanceFormatterUnitStyle`, and `MKDistanceFormatter`.
pub mod distance_formatter;
/// MapKit wrappers for `MKErrorCode` and `MKErrorDomain`.
pub mod error;
mod ffi;
/// MapKit wrappers for `MKCoordinate`, `MKCoordinateRegion`, and `MKMapItem`.
pub mod geocoder;
/// MapKit wrappers for `MKCoordinate`, `MKGeoJSONObject`, and `MKGeoJSONPointAnnotation`.
pub mod geojson;
/// MapKit wrappers for `MKCoordinate`, `MKCoordinateSpan`, and `MKCoordinateRegion`.
pub mod geometry;
/// MapKit wrappers for `MKAddressFilter`, `MKCoordinateRegion`, and `MKMapItem`.
pub mod local_search;
/// MapKit wrappers for `MKAddressFilter`, `MKCoordinateRegion`, and `MKLocalSearchRegionPriority`.
pub mod local_search_completer;
/// MapKit wrappers for `MKCoordinate`, `MKScreenSize`, and `MKMapItem`.
pub mod look_around;
/// MapKit wrappers for `MKLookAroundScene`, `MKPointOfInterestFilter`, and `MKLookAroundBadgePosition`.
pub mod look_around_view_controller;
/// MapKit wrappers for `MKAddress`, `MKAddressRepresentations`, and `MKCoordinate`.
pub mod map_item;
/// MapKit wrappers for `MKMapItem`, `MKMapItemDetailViewControllerDelegate`, and `MKMapItemDetailViewController`.
pub mod map_item_detail_view_controller;
/// MapKit wrappers for `MKPointAnnotation`, `MKUserLocation`, and `MKAnnotation`.
pub mod map_view;
/// MapKit wrappers for `MKUserLocation`, `MKAnnotation`, and `MKAnnotationView`.
pub mod map_view_delegate;
/// MapKit wrappers for `MKAnnotation`, `MKCoordinate`, and `MKMapRect`.
pub mod overlay;
/// MapKit wrappers for `MKMapRect`, `MKCircle`, and `MKMultiPolygon`.
pub mod overlay_renderer;
/// MapKit wrappers for `MKCoordinate`, `MKCoordinateRegion`, and `MKPointOfInterestCategory`.
pub mod point_of_interest;
mod private;
/// MapKit wrappers for `MKMapItemDetailSelectionAccessoryCalloutStyle`, `MKMapItemDetailSelectionAccessoryPresentationKind`, and `MKMapItemDetailSelectionAccessoryPresentationStyle`.
pub mod selection_accessory;
/// MapKit wrappers for `MKDirections`, `MKDistanceFormatter`, and `MKGeocodingRequest`.
pub mod services;
/// MapKit wrappers for `MKCoordinate`, `MKCoordinateRegion`, and `MKMapRect`.
pub mod snapshotter;
/// MapKit wrappers for `MKAddress`, `MKAddressFilter`, and `MKAddressFilterMode`.
pub mod types;
/// MapKit wrappers for `MKMapView`, `MKUserTrackingMode`, and `MKUserTrackingButtonState`.
pub mod user_tracking_button;

/// Async wrappers around MapKit completion-handler APIs.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;

/// Re-exports MapKit wrappers such as `MKAddress`, `MKAddressFilter`, and `MKAddressFilterMode`.
pub use address::{
    MKAddress, MKAddressFilter, MKAddressFilterMode, MKAddressFilterOption,
    MKAddressRepresentations, MKAddressRepresentationsContextStyle,
};
/// Re-exports MapKit wrappers such as `MKMapItemAnnotation`, `MKPointAnnotation`, and `MKUserLocation`.
pub use annotation::{MKMapItemAnnotation, MKPointAnnotation, MKUserLocation};
/// Re-exports MapKit wrappers such as `MKAnnotation`, `MKAnnotationView`, and `MKAnnotationViewCollisionMode`.
pub use annotation_view::{
    MKAnnotation, MKAnnotationView, MKAnnotationViewCollisionMode, MKAnnotationViewDragState,
    MKAnnotationViewZPriority, MKFeatureDisplayPriority, MKMarkerAnnotationView,
    MKPinAnnotationColor, MKPinAnnotationView, MKUserLocationView,
};
/// Re-exports MapKit wrappers such as `MKClusterAnnotation`.
pub use cluster_annotation::MKClusterAnnotation;
/// Re-exports MapKit wrappers such as `MKHybridMapConfiguration`, `MKImageryMapConfiguration`, and `MKMapCamera`.
pub use configuration::{
    MKHybridMapConfiguration, MKImageryMapConfiguration, MKMapCamera, MKMapCameraBoundary,
    MKMapCameraZoomRange, MKMapConfiguration, MKMapConfigurationKind, MKMapElevationStyle,
    MKStandardMapConfiguration, MKStandardMapEmphasisStyle,
};
/// Re-exports MapKit wrappers such as `MKCompassButton`, `MKPitchControl`, and `MKZoomControl`.
pub use controls::{MKCompassButton, MKPitchControl, MKZoomControl};
/// Re-exports MapKit wrappers such as `MKDirections`, `MKDirectionsRequest`, and `MKDirectionsResponse`.
pub use directions::{
    MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKDirectionsRoutePreference,
    MKDirectionsTransportType, MKETAResponse, MKRoute, MKRouteStep,
};
/// Re-exports MapKit wrappers such as `MKDistanceFormatter`, `MKDistanceFormatterUnitStyle`, and `MKDistanceFormatterUnits`.
pub use distance_formatter::{
    MKDistanceFormatter, MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits,
};
/// Re-exports MapKit wrappers such as `mk_error_domain`, `MapKitError`, and `MKErrorCode`.
pub use error::{mk_error_domain, MKErrorCode, MapKitError, NSErrorInfo};
/// Re-exports MapKit wrappers such as `MKGeocodingRequest` and `MKReverseGeocodingRequest`.
pub use geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
/// Re-exports MapKit wrappers such as `MKGeoJSONDecoder`, `MKGeoJSONFeature`, and `MKGeoJSONObject`.
pub use geojson::{
    MKGeoJSONDecoder, MKGeoJSONFeature, MKGeoJSONMultiPolygon, MKGeoJSONMultiPolyline,
    MKGeoJSONObject, MKGeoJSONObjectValue, MKGeoJSONPointAnnotation, MKGeoJSONPolygon,
    MKGeoJSONPolyline,
};
/// Re-exports MapKit wrappers such as `mk_map_points_per_meter_at_latitude`, `mk_meters_per_map_point_at_latitude`, and `mk_string_from_map_point`.
pub use geometry::{
    mk_map_points_per_meter_at_latitude, mk_meters_per_map_point_at_latitude,
    mk_string_from_map_point, mk_string_from_map_rect, mk_string_from_map_size, MKCoordinate,
    MKCoordinateRegion, MKCoordinateSpan, MKMapPoint, MKMapRect, MKMapRectDivision, MKMapRectEdge,
    MKMapSize, MKScreenPoint, MKScreenSize,
};
/// Re-exports MapKit wrappers such as `MKLocalSearch`, `MKLocalSearchRegionPriority`, and `MKLocalSearchRequest`.
pub use local_search::{
    MKLocalSearch, MKLocalSearchRegionPriority, MKLocalSearchRequest, MKLocalSearchResponse,
    MKLocalSearchResultType,
};
/// Re-exports MapKit wrappers such as `MKLocalSearchCompleter`, `MKLocalSearchCompleterDelegate`, and `MKLocalSearchCompleterResultType`.
pub use local_search_completer::{
    MKLocalSearchCompleter, MKLocalSearchCompleterDelegate, MKLocalSearchCompleterResultType,
    MKLocalSearchCompletion, MKTextHighlightRange,
};
/// Re-exports MapKit wrappers such as `MKLookAroundScene`, `MKLookAroundSceneRequest`, and `MKLookAroundSnapshot`.
pub use look_around::{
    MKLookAroundScene, MKLookAroundSceneRequest, MKLookAroundSnapshot, MKLookAroundSnapshotOptions,
    MKLookAroundSnapshotter,
};
/// Re-exports MapKit wrappers such as `MKLookAroundBadgePosition`, `MKLookAroundViewController`, and `MKLookAroundViewControllerDelegate`.
pub use look_around_view_controller::{
    MKLookAroundBadgePosition, MKLookAroundViewController, MKLookAroundViewControllerDelegate,
};
/// Re-exports MapKit wrappers such as `MKMapItem`, `MKMapItemIdentifier`, and `MKMapItemRequest`.
pub use map_item::{MKMapItem, MKMapItemIdentifier, MKMapItemRequest, MKPlacemark};
/// Re-exports MapKit wrappers such as `MKMapItemDetailViewController` and `MKMapItemDetailViewControllerDelegate`.
pub use map_item_detail_view_controller::{
    MKMapItemDetailViewController, MKMapItemDetailViewControllerDelegate,
};
/// Re-exports MapKit wrappers such as `MKFeatureVisibility`, `MKMapType`, and `MKMapView`.
pub use map_view::{MKFeatureVisibility, MKMapType, MKMapView, MKUserTrackingMode};
/// Re-exports MapKit wrappers such as `MKMapViewDelegate`.
pub use map_view_delegate::MKMapViewDelegate;
/// Re-exports MapKit wrappers such as `MKCircle`, `MKGeodesicPolyline`, and `MKMultiPoint`.
pub use overlay::{
    MKCircle, MKGeodesicPolyline, MKMultiPoint, MKMultiPolygon, MKMultiPolyline, MKOverlay,
    MKOverlayLevel, MKPolygon, MKPolyline, MKShape, MKTileOverlay, MKTileOverlayPath,
};
/// Re-exports MapKit wrappers such as `mk_road_width_at_zoom_scale`, `MKCircleRenderer`, and `MKGradientPolylineRenderer`.
pub use overlay_renderer::{
    mk_road_width_at_zoom_scale, MKCircleRenderer, MKGradientPolylineRenderer,
    MKMultiPolygonRenderer, MKMultiPolylineRenderer, MKOverlayPathRenderer, MKOverlayRenderer,
    MKPolygonRenderer, MKPolylineRenderer, MKTileOverlayRenderer, MKZoomScale,
};
/// Re-exports MapKit wrappers such as `MKLocalPointsOfInterestRequest`, `MKPointOfInterestCategory`, and `MKPointOfInterestFilter`.
pub use point_of_interest::{
    MKLocalPointsOfInterestRequest, MKPointOfInterestCategory, MKPointOfInterestFilter,
    MKPointOfInterestFilterMode,
};
/// Re-exports MapKit wrappers such as `MKMapItemDetailSelectionAccessoryCalloutStyle`, `MKMapItemDetailSelectionAccessoryPresentationKind`, and `MKMapItemDetailSelectionAccessoryPresentationStyle`.
pub use selection_accessory::{
    MKMapItemDetailSelectionAccessoryCalloutStyle,
    MKMapItemDetailSelectionAccessoryPresentationKind,
    MKMapItemDetailSelectionAccessoryPresentationStyle, MKSelectionAccessory,
};
/// Re-exports MapKit wrappers such as `MKMapSnapshot`, `MKMapSnapshotOptions`, and `MKMapSnapshotter`.
pub use snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
/// Re-exports MapKit wrappers such as `MKUserTrackingButton`.
pub use user_tracking_button::MKUserTrackingButton;

/// Common imports.
pub mod prelude {
    /// Re-exports MapKit wrappers such as `MKAddress`, `MKAddressFilter`, and `MKAddressFilterMode`.
    pub use crate::address::{
        MKAddress, MKAddressFilter, MKAddressFilterMode, MKAddressFilterOption,
        MKAddressRepresentations, MKAddressRepresentationsContextStyle,
    };
    /// Re-exports MapKit wrappers such as `MKMapItemAnnotation`, `MKPointAnnotation`, and `MKUserLocation`.
    pub use crate::annotation::{MKMapItemAnnotation, MKPointAnnotation, MKUserLocation};
    /// Re-exports MapKit wrappers such as `MKAnnotation`, `MKAnnotationView`, and `MKAnnotationViewCollisionMode`.
    pub use crate::annotation_view::{
        MKAnnotation, MKAnnotationView, MKAnnotationViewCollisionMode, MKAnnotationViewDragState,
        MKAnnotationViewZPriority, MKFeatureDisplayPriority, MKMarkerAnnotationView,
        MKPinAnnotationColor, MKPinAnnotationView, MKUserLocationView,
    };
    /// Re-exports MapKit wrappers such as `MKClusterAnnotation`.
    pub use crate::cluster_annotation::MKClusterAnnotation;
    /// Re-exports MapKit wrappers such as `MKHybridMapConfiguration`, `MKImageryMapConfiguration`, and `MKMapCamera`.
    pub use crate::configuration::{
        MKHybridMapConfiguration, MKImageryMapConfiguration, MKMapCamera, MKMapCameraBoundary,
        MKMapCameraZoomRange, MKMapConfiguration, MKMapConfigurationKind, MKMapElevationStyle,
        MKStandardMapConfiguration, MKStandardMapEmphasisStyle,
    };
    /// Re-exports MapKit wrappers such as `MKCompassButton`, `MKPitchControl`, and `MKZoomControl`.
    pub use crate::controls::{MKCompassButton, MKPitchControl, MKZoomControl};
    /// Re-exports MapKit wrappers such as `MKDirections`, `MKDirectionsRequest`, and `MKDirectionsResponse`.
    pub use crate::directions::{
        MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKDirectionsRoutePreference,
        MKDirectionsTransportType, MKETAResponse, MKRoute, MKRouteStep,
    };
    /// Re-exports MapKit wrappers such as `MKDistanceFormatter`, `MKDistanceFormatterUnitStyle`, and `MKDistanceFormatterUnits`.
    pub use crate::distance_formatter::{
        MKDistanceFormatter, MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits,
    };
    /// Re-exports MapKit wrappers such as `mk_error_domain`, `MapKitError`, and `MKErrorCode`.
    pub use crate::error::{mk_error_domain, MKErrorCode, MapKitError, NSErrorInfo};
    /// Re-exports MapKit wrappers such as `MKGeocodingRequest` and `MKReverseGeocodingRequest`.
    pub use crate::geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
    /// Re-exports MapKit wrappers such as `MKGeoJSONDecoder`, `MKGeoJSONFeature`, and `MKGeoJSONObject`.
    pub use crate::geojson::{
        MKGeoJSONDecoder, MKGeoJSONFeature, MKGeoJSONMultiPolygon, MKGeoJSONMultiPolyline,
        MKGeoJSONObject, MKGeoJSONObjectValue, MKGeoJSONPointAnnotation, MKGeoJSONPolygon,
        MKGeoJSONPolyline,
    };
    /// Re-exports MapKit wrappers such as `mk_map_points_per_meter_at_latitude`, `mk_meters_per_map_point_at_latitude`, and `mk_string_from_map_point`.
    pub use crate::geometry::{
        mk_map_points_per_meter_at_latitude, mk_meters_per_map_point_at_latitude,
        mk_string_from_map_point, mk_string_from_map_rect, mk_string_from_map_size, MKCoordinate,
        MKCoordinateRegion, MKCoordinateSpan, MKMapPoint, MKMapRect, MKMapRectDivision,
        MKMapRectEdge, MKMapSize, MKScreenPoint, MKScreenSize,
    };
    /// Re-exports MapKit wrappers such as `MKLocalSearch`, `MKLocalSearchRegionPriority`, and `MKLocalSearchRequest`.
    pub use crate::local_search::{
        MKLocalSearch, MKLocalSearchRegionPriority, MKLocalSearchRequest, MKLocalSearchResponse,
        MKLocalSearchResultType,
    };
    /// Re-exports MapKit wrappers such as `MKLocalSearchCompleter`, `MKLocalSearchCompleterDelegate`, and `MKLocalSearchCompleterResultType`.
    pub use crate::local_search_completer::{
        MKLocalSearchCompleter, MKLocalSearchCompleterDelegate, MKLocalSearchCompleterResultType,
        MKLocalSearchCompletion, MKTextHighlightRange,
    };
    /// Re-exports MapKit wrappers such as `MKLookAroundScene`, `MKLookAroundSceneRequest`, and `MKLookAroundSnapshot`.
    pub use crate::look_around::{
        MKLookAroundScene, MKLookAroundSceneRequest, MKLookAroundSnapshot,
        MKLookAroundSnapshotOptions, MKLookAroundSnapshotter,
    };
    /// Re-exports MapKit wrappers such as `MKLookAroundBadgePosition`, `MKLookAroundViewController`, and `MKLookAroundViewControllerDelegate`.
    pub use crate::look_around_view_controller::{
        MKLookAroundBadgePosition, MKLookAroundViewController, MKLookAroundViewControllerDelegate,
    };
    /// Re-exports MapKit wrappers such as `MKMapItem`, `MKMapItemIdentifier`, and `MKMapItemRequest`.
    pub use crate::map_item::{MKMapItem, MKMapItemIdentifier, MKMapItemRequest, MKPlacemark};
    /// Re-exports MapKit wrappers such as `MKMapItemDetailViewController` and `MKMapItemDetailViewControllerDelegate`.
    pub use crate::map_item_detail_view_controller::{
        MKMapItemDetailViewController, MKMapItemDetailViewControllerDelegate,
    };
    /// Re-exports MapKit wrappers such as `MKFeatureVisibility`, `MKMapType`, and `MKMapView`.
    pub use crate::map_view::{MKFeatureVisibility, MKMapType, MKMapView, MKUserTrackingMode};
    /// Re-exports MapKit wrappers such as `MKMapViewDelegate`.
    pub use crate::map_view_delegate::MKMapViewDelegate;
    /// Re-exports MapKit wrappers such as `MKCircle`, `MKGeodesicPolyline`, and `MKMultiPoint`.
    pub use crate::overlay::{
        MKCircle, MKGeodesicPolyline, MKMultiPoint, MKMultiPolygon, MKMultiPolyline, MKOverlay,
        MKOverlayLevel, MKPolygon, MKPolyline, MKShape, MKTileOverlay, MKTileOverlayPath,
    };
    /// Re-exports MapKit wrappers such as `mk_road_width_at_zoom_scale`, `MKCircleRenderer`, and `MKGradientPolylineRenderer`.
    pub use crate::overlay_renderer::{
        mk_road_width_at_zoom_scale, MKCircleRenderer, MKGradientPolylineRenderer,
        MKMultiPolygonRenderer, MKMultiPolylineRenderer, MKOverlayPathRenderer, MKOverlayRenderer,
        MKPolygonRenderer, MKPolylineRenderer, MKTileOverlayRenderer, MKZoomScale,
    };
    /// Re-exports MapKit wrappers such as `MKLocalPointsOfInterestRequest`, `MKPointOfInterestCategory`, and `MKPointOfInterestFilter`.
    pub use crate::point_of_interest::{
        MKLocalPointsOfInterestRequest, MKPointOfInterestCategory, MKPointOfInterestFilter,
        MKPointOfInterestFilterMode,
    };
    /// Re-exports MapKit wrappers such as `MKMapItemDetailSelectionAccessoryCalloutStyle`, `MKMapItemDetailSelectionAccessoryPresentationKind`, and `MKMapItemDetailSelectionAccessoryPresentationStyle`.
    pub use crate::selection_accessory::{
        MKMapItemDetailSelectionAccessoryCalloutStyle,
        MKMapItemDetailSelectionAccessoryPresentationKind,
        MKMapItemDetailSelectionAccessoryPresentationStyle, MKSelectionAccessory,
    };
    /// Re-exports MapKit wrappers such as `MKMapSnapshot`, `MKMapSnapshotOptions`, and `MKMapSnapshotter`.
    pub use crate::snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
    /// Re-exports MapKit wrappers such as `MKUserTrackingButton`.
    pub use crate::user_tracking_button::MKUserTrackingButton;
}
