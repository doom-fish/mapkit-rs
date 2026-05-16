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

pub mod error;
mod ffi;
mod private;
pub mod services;
pub mod types;

pub use error::{MapKitError, NSErrorInfo};
pub use services::{MKDirections, MKDistanceFormatter, MKLocalSearch};
pub use types::{
    MKCoordinate, MKCoordinateRegion, MKCoordinateSpan, MKDirectionsRequest,
    MKDirectionsResponse, MKDirectionsRoutePreference, MKDirectionsTransportType,
    MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits, MKETAResponse,
    MKLocalSearchRequest, MKLocalSearchResponse, MKLocalSearchResultType, MKMapItem,
    MKMapPoint, MKPlacemark, MKRoute, MKRouteStep,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::{MapKitError, NSErrorInfo};
    pub use crate::services::{MKDirections, MKDistanceFormatter, MKLocalSearch};
    pub use crate::types::{
        MKCoordinate, MKCoordinateRegion, MKCoordinateSpan, MKDirectionsRequest,
        MKDirectionsResponse, MKDirectionsRoutePreference, MKDirectionsTransportType,
        MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits, MKETAResponse,
        MKLocalSearchRequest, MKLocalSearchResponse, MKLocalSearchResultType, MKMapItem,
        MKMapPoint, MKPlacemark, MKRoute, MKRouteStep,
    };
}
