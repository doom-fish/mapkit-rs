use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl MKCoordinate {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKCoordinateSpan {
    pub latitude_delta: f64,
    pub longitude_delta: f64,
}

impl MKCoordinateSpan {
    pub const fn new(latitude_delta: f64, longitude_delta: f64) -> Self {
        Self {
            latitude_delta,
            longitude_delta,
        }
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKCoordinateRegion {
    pub center: MKCoordinate,
    pub span: MKCoordinateSpan,
}

impl MKCoordinateRegion {
    pub const fn new(center: MKCoordinate, span: MKCoordinateSpan) -> Self {
        Self { center, span }
    }

    pub fn with_distance(
        center: MKCoordinate,
        latitudinal_meters: f64,
        longitudinal_meters: f64,
    ) -> Result<Self, MapKitError> {
        let center_json = json_cstring(&center, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_coordinate_region_make_with_distance_json(
                center_json.as_ptr(),
                latitudinal_meters,
                longitudinal_meters,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKCoordinateRegion with distance failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKCoordinateRegion") }
        }
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKMapPoint {
    pub x: f64,
    pub y: f64,
}

impl MKMapPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn from_coordinate(coordinate: MKCoordinate) -> Result<Self, MapKitError> {
        let coordinate_json = json_cstring(&coordinate, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_point_for_coordinate_json(coordinate_json.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKMapPoint for coordinate failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapPoint") }
        }
    }

    pub fn coordinate(self) -> Result<MKCoordinate, MapKitError> {
        let map_point_json = json_cstring(&self, "MKMapPoint")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_coordinate_for_map_point_json(map_point_json.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "coordinate for MKMapPoint failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKCoordinate") }
        }
    }

    pub fn distance_to(self, other: Self) -> Result<f64, MapKitError> {
        let first_map_point_json = json_cstring(&self, "first MKMapPoint")?;
        let second_map_point_json = json_cstring(&other, "second MKMapPoint")?;
        let mut error = ptr::null_mut();
        let distance = unsafe {
            ffi::mk_meters_between_map_points(
                first_map_point_json.as_ptr(),
                second_map_point_json.as_ptr(),
                &mut error,
            )
        };
        if error.is_null() {
            Ok(distance)
        } else {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "meters between MKMapPoints failed")
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKMapSize {
    pub width: f64,
    pub height: f64,
}

impl MKMapSize {
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapRect {
    pub origin: MKMapPoint,
    pub size: MKMapSize,
}

impl MKMapRect {
    pub const fn new(origin: MKMapPoint, size: MKMapSize) -> Self {
        Self { origin, size }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKScreenPoint {
    pub x: f64,
    pub y: f64,
}

impl MKScreenPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKScreenSize {
    pub width: f64,
    pub height: f64,
}

impl MKScreenSize {
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}
