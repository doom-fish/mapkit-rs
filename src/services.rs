use core::ffi::c_void;
use std::ptr::{self, NonNull};

use crate::error::MapKitError;
use crate::ffi;
use crate::private::{cstring_from_str, json_cstring, parse_json_ptr, take_string};
use crate::types::{
    MKCoordinate, MKCoordinateRegion, MKDirectionsRequest, MKDirectionsResponse,
    MKDistanceFormatterUnitStyle, MKDistanceFormatterUnits, MKETAResponse,
    MKLocalSearchRequest, MKLocalSearchResponse, MKMapPoint,
};

impl MKCoordinateRegion {
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

impl MKMapPoint {
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

#[derive(Debug)]
pub struct MKLocalSearch {
    raw: NonNull<c_void>,
}

impl MKLocalSearch {
    pub fn new(request: &MKLocalSearchRequest) -> Result<Self, MapKitError> {
        let request_json = json_cstring(request, "MKLocalSearchRequest")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_local_search_new(request_json.as_ptr(), &mut error) };
        let raw = NonNull::new(raw).ok_or_else(|| unsafe {
            MapKitError::from_error_ptr(error, "failed to create MKLocalSearch")
        })?;
        Ok(Self { raw })
    }

    pub fn search(request: &MKLocalSearchRequest) -> Result<MKLocalSearchResponse, MapKitError> {
        Self::new(request)?.start()
    }

    pub fn start(&self) -> Result<MKLocalSearchResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_local_search_start_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKLocalSearch start failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKLocalSearchResponse") }
        }
    }

    pub fn is_searching(&self) -> bool {
        unsafe { ffi::mk_local_search_is_searching(self.raw.as_ptr()) }
    }

    pub fn cancel(&self) {
        unsafe { ffi::mk_local_search_cancel(self.raw.as_ptr()) };
    }
}

impl Drop for MKLocalSearch {
    fn drop(&mut self) {
        unsafe { ffi::mk_local_search_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKDirections {
    raw: NonNull<c_void>,
}

impl MKDirections {
    pub fn new(request: &MKDirectionsRequest) -> Result<Self, MapKitError> {
        let request_json = json_cstring(request, "MKDirectionsRequest")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_directions_new(request_json.as_ptr(), &mut error) };
        let raw = NonNull::new(raw).ok_or_else(|| unsafe {
            MapKitError::from_error_ptr(error, "failed to create MKDirections")
        })?;
        Ok(Self { raw })
    }

    pub fn calculate(&self) -> Result<MKDirectionsResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_directions_calculate_json(self.raw.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKDirections calculate failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKDirectionsResponse") }
        }
    }

    pub fn calculate_eta(&self) -> Result<MKETAResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_directions_calculate_eta_json(self.raw.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKDirections ETA failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKETAResponse") }
        }
    }

    pub fn is_calculating(&self) -> bool {
        unsafe { ffi::mk_directions_is_calculating(self.raw.as_ptr()) }
    }

    pub fn cancel(&self) {
        unsafe { ffi::mk_directions_cancel(self.raw.as_ptr()) };
    }
}

impl Drop for MKDirections {
    fn drop(&mut self) {
        unsafe { ffi::mk_directions_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKDistanceFormatter {
    raw: NonNull<c_void>,
}

impl MKDistanceFormatter {
    pub fn new() -> Result<Self, MapKitError> {
        let raw = NonNull::new(unsafe { ffi::mk_distance_formatter_new() }).ok_or_else(|| {
            MapKitError::OperationFailed("failed to create MKDistanceFormatter".to_owned())
        })?;
        Ok(Self { raw })
    }

    pub fn with_units(mut self, units: MKDistanceFormatterUnits) -> Self {
        self.set_units(units);
        self
    }

    pub fn with_unit_style(mut self, unit_style: MKDistanceFormatterUnitStyle) -> Self {
        self.set_unit_style(unit_style);
        self
    }

    pub fn set_units(&mut self, units: MKDistanceFormatterUnits) {
        unsafe { ffi::mk_distance_formatter_set_units(self.raw.as_ptr(), units.as_raw()) };
    }

    pub fn set_unit_style(&mut self, unit_style: MKDistanceFormatterUnitStyle) {
        unsafe {
            ffi::mk_distance_formatter_set_unit_style(
                self.raw.as_ptr(),
                unit_style.as_raw(),
            );
        };
    }

    pub fn string_from_distance(&self, distance: f64) -> Result<String, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_distance_formatter_string_from_distance(
                self.raw.as_ptr(),
                distance,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKDistanceFormatter string failed")
            })
        } else {
            unsafe { take_string(payload) }.ok_or_else(|| {
                MapKitError::OperationFailed(
                    "missing distance formatter string payload".to_owned(),
                )
            })
        }
    }

    pub fn distance_from_string(
        &self,
        distance_string: &str,
    ) -> Result<f64, MapKitError> {
        let distance_string = cstring_from_str(distance_string, "distance string")?;
        let mut error = ptr::null_mut();
        let distance = unsafe {
            ffi::mk_distance_formatter_distance_from_string(
                self.raw.as_ptr(),
                distance_string.as_ptr(),
                &mut error,
            )
        };
        if error.is_null() {
            Ok(distance)
        } else {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKDistanceFormatter distance failed")
            })
        }
    }
}

impl Drop for MKDistanceFormatter {
    fn drop(&mut self) {
        unsafe { ffi::mk_distance_formatter_release(self.raw.as_ptr()) };
    }
}
