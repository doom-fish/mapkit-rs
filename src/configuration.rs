use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKCoordinateRegion, MKMapRect};
use crate::point_of_interest::MKPointOfInterestFilter;
use crate::private::{json_cstring, parse_json_ptr};

/// Wraps `MKMapElevationStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKMapElevationStyle {
    Flat,
    Realistic,
}

/// Wraps `MKStandardMapEmphasisStyle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKStandardMapEmphasisStyle {
    Default,
    Muted,
}

/// Wraps `MKMapConfigurationKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKMapConfigurationKind {
    Standard,
    Hybrid,
    Imagery,
}

/// Wraps `MKMapCamera`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapCamera {
    /// Wraps `MKMapCamera.centerCoordinate`.
    pub center_coordinate: MKCoordinate,
    /// Wraps `MKMapCamera.centerCoordinateDistance`.
    pub center_coordinate_distance: f64,
    /// Wraps `MKMapCamera.heading`.
    pub heading: f64,
    /// Wraps `MKMapCamera.pitch`.
    pub pitch: f64,
    /// Wraps `MKMapCamera.altitude`.
    pub altitude: f64,
}

impl MKMapCamera {
    /// Creates a wrapper for `MKMapCamera`.
    pub fn new(
        center_coordinate: MKCoordinate,
        center_coordinate_distance: f64,
        pitch: f64,
        heading: f64,
    ) -> Self {
        Self {
            center_coordinate,
            center_coordinate_distance,
            heading,
            pitch,
            altitude: center_coordinate_distance,
        }
    }

    /// Wraps `MKMapCamera.lookingAtCenterCoordinate`.
    pub fn looking_at_center_coordinate(
        center_coordinate: MKCoordinate,
        distance: f64,
        pitch: f64,
        heading: f64,
    ) -> Self {
        Self::new(center_coordinate, distance, pitch, heading)
    }
}

/// Wraps `MKMapCameraBoundary`.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapCameraBoundary {
    /// Wraps `MKMapCameraBoundary.mapRect`.
    pub map_rect: MKMapRect,
    /// Wraps `MKMapCameraBoundary.region`.
    pub region: MKCoordinateRegion,
}

impl MKMapCameraBoundary {
    /// Wraps `MKMapCameraBoundary.fromMapRect`.
    pub fn from_map_rect(map_rect: MKMapRect) -> Result<Self, MapKitError> {
        let map_rect = json_cstring(&map_rect, "MKMapRect")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_camera_boundary_from_map_rect_json(map_rect.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to create MKMapCameraBoundary")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapCameraBoundary") }
        }
    }

    /// Wraps `MKMapCameraBoundary.fromRegion`.
    pub fn from_region(region: MKCoordinateRegion) -> Result<Self, MapKitError> {
        let region = json_cstring(&region, "MKCoordinateRegion")?;
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_map_camera_boundary_from_region_json(region.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to create MKMapCameraBoundary")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapCameraBoundary") }
        }
    }
}

/// Wraps `MKMapCameraZoomRange`.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapCameraZoomRange {
    /// Wraps `MKMapCameraZoomRange.minCenterCoordinateDistance`.
    pub min_center_coordinate_distance: Option<f64>,
    /// Wraps `MKMapCameraZoomRange.maxCenterCoordinateDistance`.
    pub max_center_coordinate_distance: Option<f64>,
}

impl MKMapCameraZoomRange {
    /// Creates a wrapper for `MKMapCameraZoomRange`.
    pub fn new(min_center_coordinate_distance: f64, max_center_coordinate_distance: f64) -> Self {
        Self {
            min_center_coordinate_distance: Some(min_center_coordinate_distance),
            max_center_coordinate_distance: Some(max_center_coordinate_distance),
        }
    }

    /// Wraps `MKMapCameraZoomRange.minCenterCoordinateDistance`.
    pub fn with_min_center_coordinate_distance(min_center_coordinate_distance: f64) -> Self {
        Self {
            min_center_coordinate_distance: Some(min_center_coordinate_distance),
            max_center_coordinate_distance: None,
        }
    }

    /// Wraps `MKMapCameraZoomRange.maxCenterCoordinateDistance`.
    pub fn with_max_center_coordinate_distance(max_center_coordinate_distance: f64) -> Self {
        Self {
            min_center_coordinate_distance: None,
            max_center_coordinate_distance: Some(max_center_coordinate_distance),
        }
    }

    /// Wraps `MKMapCameraZoomRange.defaultDistance`.
    pub fn default_distance() -> f64 {
        unsafe { ffi::mk_map_camera_zoom_default() }
    }
}

/// Wraps `MKMapConfiguration`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapConfiguration {
    /// Wraps `MKMapConfiguration.kind`.
    pub kind: MKMapConfigurationKind,
    /// Wraps `MKMapConfiguration.elevationStyle`.
    pub elevation_style: MKMapElevationStyle,
    /// Wraps `MKMapConfiguration.emphasisStyle`.
    pub emphasis_style: Option<MKStandardMapEmphasisStyle>,
    /// Wraps `MKMapConfiguration.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
    /// Wraps `MKMapConfiguration.showsTraffic`.
    pub shows_traffic: Option<bool>,
}

impl MKMapConfiguration {
    /// Wraps `MKMapConfiguration.elevationStyle`.
    pub fn elevation_style(&self) -> MKMapElevationStyle {
        self.elevation_style
    }

    /// Wraps `MKMapConfiguration.asStandard`.
    pub fn as_standard(&self) -> Option<MKStandardMapConfiguration> {
        (self.kind == MKMapConfigurationKind::Standard).then(|| MKStandardMapConfiguration {
            elevation_style: self.elevation_style,
            emphasis_style: self
                .emphasis_style
                .unwrap_or(MKStandardMapEmphasisStyle::Default),
            point_of_interest_filter: self.point_of_interest_filter.clone(),
            shows_traffic: self.shows_traffic.unwrap_or_default(),
        })
    }

    /// Wraps `MKMapConfiguration.asHybrid`.
    pub fn as_hybrid(&self) -> Option<MKHybridMapConfiguration> {
        (self.kind == MKMapConfigurationKind::Hybrid).then(|| MKHybridMapConfiguration {
            elevation_style: self.elevation_style,
            point_of_interest_filter: self.point_of_interest_filter.clone(),
            shows_traffic: self.shows_traffic.unwrap_or_default(),
        })
    }

    /// Wraps `MKMapConfiguration.asImagery`.
    pub fn as_imagery(&self) -> Option<MKImageryMapConfiguration> {
        (self.kind == MKMapConfigurationKind::Imagery).then_some(MKImageryMapConfiguration {
            elevation_style: self.elevation_style,
        })
    }
}

/// Wraps `MKStandardMapConfiguration`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKStandardMapConfiguration {
    /// Wraps `MKStandardMapConfiguration.elevationStyle`.
    pub elevation_style: MKMapElevationStyle,
    /// Wraps `MKStandardMapConfiguration.emphasisStyle`.
    pub emphasis_style: MKStandardMapEmphasisStyle,
    /// Wraps `MKStandardMapConfiguration.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
    /// Wraps `MKStandardMapConfiguration.showsTraffic`.
    pub shows_traffic: bool,
}

impl MKStandardMapConfiguration {
    /// Creates a wrapper for `MKStandardMapConfiguration`.
    pub fn new() -> Self {
        Self {
            elevation_style: MKMapElevationStyle::Flat,
            emphasis_style: MKStandardMapEmphasisStyle::Default,
            point_of_interest_filter: None,
            shows_traffic: false,
        }
    }

    /// Wraps `MKStandardMapConfiguration.elevationStyle`.
    pub fn with_elevation_style(mut self, elevation_style: MKMapElevationStyle) -> Self {
        self.elevation_style = elevation_style;
        self
    }

    /// Wraps `MKStandardMapConfiguration.emphasisStyle`.
    pub fn with_emphasis_style(mut self, emphasis_style: MKStandardMapEmphasisStyle) -> Self {
        self.emphasis_style = emphasis_style;
        self
    }

    /// Wraps `MKStandardMapConfiguration.pointOfInterestFilter`.
    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    /// Wraps `MKStandardMapConfiguration.showsTraffic`.
    pub fn with_shows_traffic(mut self, shows_traffic: bool) -> Self {
        self.shows_traffic = shows_traffic;
        self
    }
}

impl Default for MKStandardMapConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MKStandardMapConfiguration> for MKMapConfiguration {
    fn from(configuration: MKStandardMapConfiguration) -> Self {
        Self {
            kind: MKMapConfigurationKind::Standard,
            elevation_style: configuration.elevation_style,
            emphasis_style: Some(configuration.emphasis_style),
            point_of_interest_filter: configuration.point_of_interest_filter,
            shows_traffic: Some(configuration.shows_traffic),
        }
    }
}

/// Wraps `MKHybridMapConfiguration`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKHybridMapConfiguration {
    /// Wraps `MKHybridMapConfiguration.elevationStyle`.
    pub elevation_style: MKMapElevationStyle,
    /// Wraps `MKHybridMapConfiguration.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
    /// Wraps `MKHybridMapConfiguration.showsTraffic`.
    pub shows_traffic: bool,
}

impl MKHybridMapConfiguration {
    /// Creates a wrapper for `MKHybridMapConfiguration`.
    pub fn new() -> Self {
        Self {
            elevation_style: MKMapElevationStyle::Flat,
            point_of_interest_filter: None,
            shows_traffic: false,
        }
    }

    /// Wraps `MKHybridMapConfiguration.elevationStyle`.
    pub fn with_elevation_style(mut self, elevation_style: MKMapElevationStyle) -> Self {
        self.elevation_style = elevation_style;
        self
    }

    /// Wraps `MKHybridMapConfiguration.pointOfInterestFilter`.
    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    /// Wraps `MKHybridMapConfiguration.showsTraffic`.
    pub fn with_shows_traffic(mut self, shows_traffic: bool) -> Self {
        self.shows_traffic = shows_traffic;
        self
    }
}

impl Default for MKHybridMapConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MKHybridMapConfiguration> for MKMapConfiguration {
    fn from(configuration: MKHybridMapConfiguration) -> Self {
        Self {
            kind: MKMapConfigurationKind::Hybrid,
            elevation_style: configuration.elevation_style,
            emphasis_style: None,
            point_of_interest_filter: configuration.point_of_interest_filter,
            shows_traffic: Some(configuration.shows_traffic),
        }
    }
}

/// Wraps `MKImageryMapConfiguration`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKImageryMapConfiguration {
    /// Wraps `MKImageryMapConfiguration.elevationStyle`.
    pub elevation_style: MKMapElevationStyle,
}

impl MKImageryMapConfiguration {
    /// Creates a wrapper for `MKImageryMapConfiguration`.
    pub fn new() -> Self {
        Self {
            elevation_style: MKMapElevationStyle::Flat,
        }
    }

    /// Wraps `MKImageryMapConfiguration.elevationStyle`.
    pub fn with_elevation_style(mut self, elevation_style: MKMapElevationStyle) -> Self {
        self.elevation_style = elevation_style;
        self
    }
}

impl Default for MKImageryMapConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl From<MKImageryMapConfiguration> for MKMapConfiguration {
    fn from(configuration: MKImageryMapConfiguration) -> Self {
        Self {
            kind: MKMapConfigurationKind::Imagery,
            elevation_style: configuration.elevation_style,
            emphasis_style: None,
            point_of_interest_filter: None,
            shows_traffic: None,
        }
    }
}
