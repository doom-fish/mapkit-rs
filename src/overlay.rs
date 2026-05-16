use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::annotation_view::MKAnnotation;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKMapRect, MKScreenSize};
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKOverlayLevel {
    AboveRoads,
    AboveLabels,
}

pub trait MKOverlay: MKAnnotation {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError>;
    fn can_replace_map_content(&self) -> Result<bool, MapKitError>;
    #[doc(hidden)]
    fn as_raw_overlay(&self) -> *mut c_void;
}

pub trait MKShape: MKAnnotation {}

pub trait MKMultiPoint: MKOverlay {
    fn point_count(&self) -> Result<usize, MapKitError>;
    fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError>;
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
    can_replace_map_content: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMultiPointState {
    coordinate: MKCoordinate,
    bounding_map_rect: MKMapRect,
    can_replace_map_content: bool,
    point_count: usize,
    coordinates: Vec<MKCoordinate>,
    interior_polygon_count: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKTileOverlayPath {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub content_scale_factor: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKTileOverlayState {
    coordinate: MKCoordinate,
    bounding_map_rect: MKMapRect,
    url_template: Option<String>,
    tile_size: MKScreenSize,
    geometry_flipped: bool,
    minimum_z: i64,
    maximum_z: i64,
    can_replace_map_content: bool,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKTileOverlayOptions {
    tile_size: Option<MKScreenSize>,
    geometry_flipped: Option<bool>,
    minimum_z: Option<i64>,
    maximum_z: Option<i64>,
    can_replace_map_content: Option<bool>,
}

#[derive(Debug)]
pub struct MKCircle {
    raw: NonNull<c_void>,
}

impl MKCircle {
    pub fn new(center: MKCoordinate, radius: f64) -> Result<Self, MapKitError> {
        let payload = json_cstring(
            &MKCirclePayload {
                coordinate: center,
                radius,
            },
            "MKCircle",
        )?;
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

    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl MKAnnotation for MKCircle {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKOverlay for MKCircle {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Self::bounding_map_rect(self)
    }

    fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Self::can_replace_map_content(self)
    }

    fn as_raw_overlay(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKShape for MKCircle {}

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
            Err(unsafe { MapKitError::from_error_ptr(error, "failed to read MKPolyline state") })
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

    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
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

impl MKAnnotation for MKPolyline {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKOverlay for MKPolyline {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Self::bounding_map_rect(self)
    }

    fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Self::can_replace_map_content(self)
    }

    fn as_raw_overlay(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKShape for MKPolyline {}
impl MKMultiPoint for MKPolyline {
    fn point_count(&self) -> Result<usize, MapKitError> {
        Self::point_count(self)
    }

    fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError> {
        Self::coordinates(self)
    }
}

impl Drop for MKPolyline {
    fn drop(&mut self) {
        unsafe { ffi::mk_polyline_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKGeodesicPolyline {
    raw: NonNull<c_void>,
}

impl MKGeodesicPolyline {
    pub fn new(coordinates: &[MKCoordinate]) -> Result<Self, MapKitError> {
        let payload = json_cstring(coordinates, "MKGeodesicPolyline coordinates")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_geodesic_polyline_new_json(payload.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKGeodesicPolyline")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMultiPointState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_geodesic_polyline_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKGeodesicPolyline state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKGeodesicPolyline state") }
        }
    }

    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
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

impl MKAnnotation for MKGeodesicPolyline {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKOverlay for MKGeodesicPolyline {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Self::bounding_map_rect(self)
    }

    fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Self::can_replace_map_content(self)
    }

    fn as_raw_overlay(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKShape for MKGeodesicPolyline {}
impl MKMultiPoint for MKGeodesicPolyline {
    fn point_count(&self) -> Result<usize, MapKitError> {
        Self::point_count(self)
    }

    fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError> {
        Self::coordinates(self)
    }
}

impl Drop for MKGeodesicPolyline {
    fn drop(&mut self) {
        unsafe { ffi::mk_geodesic_polyline_release(self.raw.as_ptr()) };
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
            Err(unsafe { MapKitError::from_error_ptr(error, "failed to read MKPolygon state") })
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

    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
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

impl MKAnnotation for MKPolygon {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKOverlay for MKPolygon {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Self::bounding_map_rect(self)
    }

    fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Self::can_replace_map_content(self)
    }

    fn as_raw_overlay(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKShape for MKPolygon {}
impl MKMultiPoint for MKPolygon {
    fn point_count(&self) -> Result<usize, MapKitError> {
        Self::point_count(self)
    }

    fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError> {
        Self::coordinates(self)
    }
}

impl Drop for MKPolygon {
    fn drop(&mut self) {
        unsafe { ffi::mk_polygon_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKTileOverlay {
    raw: NonNull<c_void>,
}

impl MKTileOverlay {
    pub fn new(url_template: Option<&str>) -> Result<Self, MapKitError> {
        let url_template = url_template
            .map(|value| crate::private::cstring_from_str(value, "MKTileOverlay URLTemplate"))
            .transpose()?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_tile_overlay_new(
                url_template
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        let raw = owned_handle(raw, error, "failed to create MKTileOverlay")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKTileOverlayState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_tile_overlay_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "failed to read MKTileOverlay state") })
        } else {
            unsafe { parse_json_ptr(payload, "MKTileOverlay state") }
        }
    }

    fn apply_options(&self, options: &MKTileOverlayOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKTileOverlay options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_tile_overlay_apply_options_json(self.raw.as_ptr(), options.as_ptr(), &mut error);
        };
        unsafe { unit_result(error, "failed to update MKTileOverlay") }
    }

    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    pub fn url_template(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.url_template)
    }

    pub fn tile_size(&self) -> Result<MKScreenSize, MapKitError> {
        Ok(self.state()?.tile_size)
    }

    pub fn set_tile_size(&self, tile_size: MKScreenSize) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            tile_size: Some(tile_size),
            ..MKTileOverlayOptions::default()
        })
    }

    pub fn is_geometry_flipped(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.geometry_flipped)
    }

    pub fn set_geometry_flipped(&self, geometry_flipped: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            geometry_flipped: Some(geometry_flipped),
            ..MKTileOverlayOptions::default()
        })
    }

    pub fn minimum_z(&self) -> Result<i64, MapKitError> {
        Ok(self.state()?.minimum_z)
    }

    pub fn set_minimum_z(&self, minimum_z: i64) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            minimum_z: Some(minimum_z),
            ..MKTileOverlayOptions::default()
        })
    }

    pub fn maximum_z(&self) -> Result<i64, MapKitError> {
        Ok(self.state()?.maximum_z)
    }

    pub fn set_maximum_z(&self, maximum_z: i64) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            maximum_z: Some(maximum_z),
            ..MKTileOverlayOptions::default()
        })
    }

    pub fn set_can_replace_map_content(
        &self,
        can_replace_map_content: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            can_replace_map_content: Some(can_replace_map_content),
            ..MKTileOverlayOptions::default()
        })
    }

    pub fn url_for_tile_path(
        &self,
        path: MKTileOverlayPath,
    ) -> Result<Option<String>, MapKitError> {
        let path = json_cstring(&path, "MKTileOverlayPath")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_tile_overlay_url_for_tile_path_json(
                self.raw.as_ptr(),
                path.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKTileOverlay tile URL")
            })
        } else {
            unsafe { parse_json_ptr(payload, "Optional<String>") }
        }
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl MKAnnotation for MKTileOverlay {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(None)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKOverlay for MKTileOverlay {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Self::bounding_map_rect(self)
    }

    fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Self::can_replace_map_content(self)
    }

    fn as_raw_overlay(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl Drop for MKTileOverlay {
    fn drop(&mut self) {
        unsafe { ffi::mk_tile_overlay_release(self.raw.as_ptr()) };
    }
}
