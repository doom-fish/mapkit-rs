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
pub mod cluster_annotation;
pub mod directions;
pub mod distance_formatter;
pub mod error;
mod ffi;
pub mod geocoder;
pub mod geometry;
pub mod local_search;
pub mod look_around;
pub mod map_item;
pub mod map_view;
pub mod overlay;
mod private;
pub mod point_of_interest;
pub mod services;
pub mod snapshotter;
pub mod types;
pub mod user_tracking_button;

pub use address::{
    MKAddress, MKAddressFilter, MKAddressFilterMode, MKAddressFilterOption,
    MKAddressRepresentations, MKAddressRepresentationsContextStyle,
};
pub use annotation::MKPointAnnotation;
pub use cluster_annotation::MKClusterAnnotation;
pub use directions::{
    MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKDirectionsRoutePreference,
    MKDirectionsTransportType, MKETAResponse, MKRoute, MKRouteStep,
};
pub use distance_formatter::{
    MKDistanceFormatter, MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits,
};
pub use error::{MapKitError, NSErrorInfo};
pub use geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
pub use geometry::{
    MKCoordinate, MKCoordinateRegion, MKCoordinateSpan, MKMapPoint, MKMapRect, MKMapSize,
    MKScreenPoint, MKScreenSize,
};
pub use local_search::{
    MKLocalSearch, MKLocalSearchRegionPriority, MKLocalSearchRequest, MKLocalSearchResponse,
    MKLocalSearchResultType,
};
pub use look_around::{
    MKLookAroundScene, MKLookAroundSceneRequest, MKLookAroundSnapshot,
    MKLookAroundSnapshotOptions, MKLookAroundSnapshotter,
};
pub use map_item::{MKMapItem, MKPlacemark};
pub use map_view::{
    MKFeatureVisibility, MKMapType, MKMapView, MKUserTrackingMode,
};
pub use overlay::{MKCircle, MKOverlayLevel, MKPolygon, MKPolyline};
pub use point_of_interest::{
    MKLocalPointsOfInterestRequest, MKPointOfInterestCategory, MKPointOfInterestFilter,
    MKPointOfInterestFilterMode,
};
pub use snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
pub use user_tracking_button::MKUserTrackingButton;

/// Common imports.
pub mod prelude {
    pub use crate::address::{
        MKAddress, MKAddressFilter, MKAddressFilterMode, MKAddressFilterOption,
        MKAddressRepresentations, MKAddressRepresentationsContextStyle,
    };
    pub use crate::annotation::MKPointAnnotation;
    pub use crate::cluster_annotation::MKClusterAnnotation;
    pub use crate::directions::{
        MKDirections, MKDirectionsRequest, MKDirectionsResponse, MKDirectionsRoutePreference,
        MKDirectionsTransportType, MKETAResponse, MKRoute, MKRouteStep,
    };
    pub use crate::distance_formatter::{
        MKDistanceFormatter, MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits,
    };
    pub use crate::error::{MapKitError, NSErrorInfo};
    pub use crate::geocoder::{MKGeocodingRequest, MKReverseGeocodingRequest};
    pub use crate::geometry::{
        MKCoordinate, MKCoordinateRegion, MKCoordinateSpan, MKMapPoint, MKMapRect, MKMapSize,
        MKScreenPoint, MKScreenSize,
    };
    pub use crate::local_search::{
        MKLocalSearch, MKLocalSearchRegionPriority, MKLocalSearchRequest,
        MKLocalSearchResponse, MKLocalSearchResultType,
    };
    pub use crate::look_around::{
        MKLookAroundScene, MKLookAroundSceneRequest, MKLookAroundSnapshot,
        MKLookAroundSnapshotOptions, MKLookAroundSnapshotter,
    };
    pub use crate::map_item::{MKMapItem, MKPlacemark};
    pub use crate::map_view::{
        MKFeatureVisibility, MKMapType, MKMapView, MKUserTrackingMode,
    };
    pub use crate::overlay::{MKCircle, MKOverlayLevel, MKPolygon, MKPolyline};
    pub use crate::point_of_interest::{
        MKLocalPointsOfInterestRequest, MKPointOfInterestCategory,
        MKPointOfInterestFilter, MKPointOfInterestFilterMode,
    };
    pub use crate::snapshotter::{MKMapSnapshot, MKMapSnapshotOptions, MKMapSnapshotter};
    pub use crate::user_tracking_button::MKUserTrackingButton;
}
