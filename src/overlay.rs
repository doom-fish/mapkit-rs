use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKMapRect};
use crate::private::{json_cstring, owned_handle, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKOverlayLevel {
    AboveRoads,
    AboveLabels,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKCirclePayload {
    coordinate: MKCoordinate,
    radius: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKCircleState {
    coordinate: MKCoordinate,
    radius: f64,
    bounding_map_rect: MKMapRect,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMultiPointState {
    coordinate: MKCoordinate,
    bounding_map_rect: MKMapRect,
    point_count: usize,
    coordinates: Vec<MKCoordinate>,
    interior_polygon_count: Option<usize>,
}

#[derive(Debug)]
pub struct MKCircle {
    raw: NonNull<c_void>,
}

impl MKCircle {
    pub fn new(center: MKCoordinate, radius: f64) -> Result<Self, MapKitError> {
        let payload = json_cstring(&MKCirclePayload { coordinate: center, radius }, "MKCircle")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_circle_new_json(payload.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKCircle")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKCircleState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_circle_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "failed to read MKCircle state") })
        } else {
            unsafe { parse_json_ptr(payload, "MKCircle state") }
        }
    }

    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    pub fn radius(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.radius)
    }

    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKCircle {
    fn drop(&mut self) {
        unsafe { ffi::mk_circle_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKPolyline {
    raw: NonNull<c_void>,
}

impl MKPolyline {
    pub fn new(coordinates: &[MKCoordinate]) -> Result<Self, MapKitError> {
        let payload = json_cstring(coordinates, "MKPolyline coordinates")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_polyline_new_json(payload.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKPolyline")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMultiPointState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_polyline_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKPolyline state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKPolyline state") }
        }
    }

    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    pub fn point_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.point_count)
    }

    pub fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError> {
        Ok(self.state()?.coordinates)
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKPolyline {
    fn drop(&mut self) {
        unsafe { ffi::mk_polyline_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKPolygon {
    raw: NonNull<c_void>,
}

impl MKPolygon {
    pub fn new(coordinates: &[MKCoordinate]) -> Result<Self, MapKitError> {
        let payload = json_cstring(coordinates, "MKPolygon coordinates")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_polygon_new_json(payload.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKPolygon")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMultiPointState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_polygon_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKPolygon state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKPolygon state") }
        }
    }

    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    pub fn point_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.point_count)
    }

    pub fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError> {
        Ok(self.state()?.coordinates)
    }

    pub fn interior_polygon_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.interior_polygon_count.unwrap_or_default())
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKPolygon {
    fn drop(&mut self) {
        unsafe { ffi::mk_polygon_release(self.raw.as_ptr()) };
    }
}
