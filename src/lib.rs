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

pub mod address;
pub mod annotation;
pub mod annotation_view;
pub mod cluster_annotation;
pub mod configuration;
pub mod controls;
pub mod directions;
pub mod distance_formatter;
pub mod error;
mod ffi;
pub mod geocoder;
pub mod geojson;
pub mod geometry;
pub mod local_search;
pub mod local_search_completer;
pub mod look_around;
pub mod look_around_view_controller;
pub mod map_item;
pub mod map_item_detail_view_controller;
pub mod map_view;
pub mod map_view_delegate;
pub mod overlay;
pub mod overlay_renderer;
pub mod point_of_interest;
mod private;
pub mod selection_accessory;
pub mod services;
pub mod snapshotter;
pub mod types;
pub mod user_tracking_button;

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;

pub use address::{
    MKAddress, MKAddressFilter, MKAddressFilterMode, MKAddressFilterOption,
    MKAddressRepresentations, MKAddressRepresentationsContextStyle,
};
pub use annotation::{MKMapItemAnnotation, MKPointAnnotation, MKUserLocation};
pub use controls::{MKCompassButton, MKPitchControl, MKZoomControl};
pub use annotation_view::{
    MKAnnotation, MKAnnotationView, MKAnnotationViewCollisionMode, MKAnnotationViewDragState,
    MKAnnotationViewZPriority, MKFeatureDisplayPriority, MKMarkerAnnotationView,
    MKPinAnnotationColor, MKPinAnnotationView, MKUserLocationView,
};
pub use cluster_annotation::MKClusterAnnotation;
pub use configuration::{
    MKHybridMapConfiguration, MKImageryMapConfiguration, MKMapCamera, MKMapCameraBoundary,
    MKMapCameraZoomRange, MKMapConfiguration, MKMapConfigurationKind, MKMapElevationStyle,
    MKStandardMapConfiguration, MKStandardMapEmphasisStyle,
};
pub use directions::{
    MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKDirectionsRoutePreference,
    MKDirectionsTransportType, MKETAResponse, MKRoute, MKRouteStep,
};
pub use distance_formatter::{
    MKDistanceFormatter, MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits,
};
pub use error::{mk_error_domain, MapKitError, MKErrorCode, NSErrorInfo};
pub use geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
pub use geojson::{
    MKGeoJSONDecoder, MKGeoJSONFeature, MKGeoJSONObject, MKGeoJSONObjectValue,
    MKGeoJSONMultiPolygon, MKGeoJSONMultiPolyline, MKGeoJSONPointAnnotation,
    MKGeoJSONPolygon, MKGeoJSONPolyline,
};
pub use geometry::{
    mk_map_points_per_meter_at_latitude, mk_meters_per_map_point_at_latitude,
    mk_string_from_map_point, mk_string_from_map_rect, mk_string_from_map_size, MKCoordinate,
    MKCoordinateRegion, MKCoordinateSpan, MKMapPoint, MKMapRect, MKMapRectDivision,
    MKMapRectEdge, MKMapSize, MKScreenPoint, MKScreenSize,
};
pub use local_search::{
    MKLocalSearch, MKLocalSearchRegionPriority, MKLocalSearchRequest, MKLocalSearchResponse,
    MKLocalSearchResultType,
};
pub use local_search_completer::{
    MKLocalSearchCompleter, MKLocalSearchCompleterDelegate, MKLocalSearchCompleterResultType,
    MKLocalSearchCompletion, MKTextHighlightRange,
};
pub use look_around::{
    MKLookAroundScene, MKLookAroundSceneRequest, MKLookAroundSnapshot, MKLookAroundSnapshotOptions,
    MKLookAroundSnapshotter,
};
pub use look_around_view_controller::{
    MKLookAroundBadgePosition, MKLookAroundViewController, MKLookAroundViewControllerDelegate,
};
pub use map_item::{MKMapItem, MKMapItemIdentifier, MKMapItemRequest, MKPlacemark};
pub use map_item_detail_view_controller::{
    MKMapItemDetailViewController, MKMapItemDetailViewControllerDelegate,
};
pub use map_view::{MKFeatureVisibility, MKMapType, MKMapView, MKUserTrackingMode};
pub use map_view_delegate::MKMapViewDelegate;
pub use overlay::{
    MKCircle, MKGeodesicPolyline, MKMultiPoint, MKMultiPolygon, MKMultiPolyline, MKOverlay,
    MKOverlayLevel, MKPolygon, MKPolyline, MKShape, MKTileOverlay, MKTileOverlayPath,
};
pub use overlay_renderer::{
    mk_road_width_at_zoom_scale, MKCircleRenderer, MKGradientPolylineRenderer,
    MKMultiPolygonRenderer, MKMultiPolylineRenderer, MKOverlayPathRenderer, MKOverlayRenderer,
    MKPolygonRenderer, MKPolylineRenderer, MKTileOverlayRenderer, MKZoomScale,
};
pub use point_of_interest::{
    MKLocalPointsOfInterestRequest, MKPointOfInterestCategory, MKPointOfInterestFilter,
    MKPointOfInterestFilterMode,
};
pub use selection_accessory::{
    MKMapItemDetailSelectionAccessoryCalloutStyle,
    MKMapItemDetailSelectionAccessoryPresentationKind,
    MKMapItemDetailSelectionAccessoryPresentationStyle, MKSelectionAccessory,
};
pub use snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
pub use user_tracking_button::MKUserTrackingButton;

/// Common imports.
pub mod prelude {
    pub use crate::address::{
        MKAddress, MKAddressFilter, MKAddressFilterMode, MKAddressFilterOption,
        MKAddressRepresentations, MKAddressRepresentationsContextStyle,
    };
    pub use crate::annotation::{MKMapItemAnnotation, MKPointAnnotation, MKUserLocation};
    pub use crate::annotation_view::{
        MKAnnotation, MKAnnotationView, MKAnnotationViewCollisionMode, MKAnnotationViewDragState,
        MKAnnotationViewZPriority, MKFeatureDisplayPriority, MKMarkerAnnotationView,
        MKPinAnnotationColor, MKPinAnnotationView, MKUserLocationView,
    };
    pub use crate::controls::{MKCompassButton, MKPitchControl, MKZoomControl};
    pub use crate::cluster_annotation::MKClusterAnnotation;
    pub use crate::configuration::{
        MKHybridMapConfiguration, MKImageryMapConfiguration, MKMapCamera, MKMapCameraBoundary,
        MKMapCameraZoomRange, MKMapConfiguration, MKMapConfigurationKind, MKMapElevationStyle,
        MKStandardMapConfiguration, MKStandardMapEmphasisStyle,
    };
    pub use crate::directions::{
        MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKDirectionsRoutePreference,
        MKDirectionsTransportType, MKETAResponse, MKRoute, MKRouteStep,
    };
    pub use crate::distance_formatter::{
        MKDistanceFormatter, MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits,
    };
    pub use crate::error::{mk_error_domain, MapKitError, MKErrorCode, NSErrorInfo};
    pub use crate::geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
    pub use crate::geojson::{
        MKGeoJSONDecoder, MKGeoJSONFeature, MKGeoJSONObject, MKGeoJSONObjectValue,
        MKGeoJSONMultiPolygon, MKGeoJSONMultiPolyline, MKGeoJSONPointAnnotation,
        MKGeoJSONPolygon, MKGeoJSONPolyline,
    };
    pub use crate::geometry::{
        mk_map_points_per_meter_at_latitude, mk_meters_per_map_point_at_latitude,
        mk_string_from_map_point, mk_string_from_map_rect, mk_string_from_map_size, MKCoordinate,
        MKCoordinateRegion, MKCoordinateSpan, MKMapPoint, MKMapRect, MKMapRectDivision,
        MKMapRectEdge, MKMapSize, MKScreenPoint, MKScreenSize,
    };
    pub use crate::local_search::{
        MKLocalSearch, MKLocalSearchRegionPriority, MKLocalSearchRequest, MKLocalSearchResponse,
        MKLocalSearchResultType,
    };
    pub use crate::local_search_completer::{
        MKLocalSearchCompleter, MKLocalSearchCompleterDelegate, MKLocalSearchCompleterResultType,
        MKLocalSearchCompletion, MKTextHighlightRange,
    };
    pub use crate::look_around::{
        MKLookAroundScene, MKLookAroundSceneRequest, MKLookAroundSnapshot,
        MKLookAroundSnapshotOptions, MKLookAroundSnapshotter,
    };
    pub use crate::look_around_view_controller::{
        MKLookAroundBadgePosition, MKLookAroundViewController,
        MKLookAroundViewControllerDelegate,
    };
    pub use crate::map_item::{MKMapItem, MKMapItemIdentifier, MKMapItemRequest, MKPlacemark};
    pub use crate::map_item_detail_view_controller::{
        MKMapItemDetailViewController, MKMapItemDetailViewControllerDelegate,
    };
    pub use crate::map_view::{MKFeatureVisibility, MKMapType, MKMapView, MKUserTrackingMode};
    pub use crate::map_view_delegate::MKMapViewDelegate;
    pub use crate::overlay::{
        MKCircle, MKGeodesicPolyline, MKMultiPoint, MKMultiPolygon, MKMultiPolyline, MKOverlay,
        MKOverlayLevel, MKPolygon, MKPolyline, MKShape, MKTileOverlay, MKTileOverlayPath,
    };
    pub use crate::overlay_renderer::{
        mk_road_width_at_zoom_scale, MKCircleRenderer, MKGradientPolylineRenderer,
        MKMultiPolygonRenderer, MKMultiPolylineRenderer, MKOverlayPathRenderer, MKOverlayRenderer,
        MKPolygonRenderer, MKPolylineRenderer, MKTileOverlayRenderer, MKZoomScale,
    };
    pub use crate::point_of_interest::{
        MKLocalPointsOfInterestRequest, MKPointOfInterestCategory, MKPointOfInterestFilter,
        MKPointOfInterestFilterMode,
    };
    pub use crate::selection_accessory::{
        MKMapItemDetailSelectionAccessoryCalloutStyle,
        MKMapItemDetailSelectionAccessoryPresentationKind,
        MKMapItemDetailSelectionAccessoryPresentationStyle, MKSelectionAccessory,
    };
    pub use crate::snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
    pub use crate::user_tracking_button::MKUserTrackingButton;
}
