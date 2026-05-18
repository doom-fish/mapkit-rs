use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::annotation_view::MKAnnotation;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKMapRect, MKScreenSize};
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result};

/// Wraps `MKOverlayLevel`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKOverlayLevel {
    AboveRoads,
    AboveLabels,
}

/// Wraps `MKOverlay`.
pub trait MKOverlay: MKAnnotation {
    fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError>;
    fn can_replace_map_content(&self) -> Result<bool, MapKitError>;
    #[doc(hidden)]
    fn as_raw_overlay(&self) -> *mut c_void;
}

/// Wraps `MKShape`.
pub trait MKShape: MKAnnotation {}

/// Wraps `MKMultiPoint`.
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

/// Wraps `MKTileOverlayPath`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKTileOverlayPath {
    /// Wraps `MKTileOverlayPath.x`.
    pub x: i64,
    /// Wraps `MKTileOverlayPath.y`.
    pub y: i64,
    /// Wraps `MKTileOverlayPath.z`.
    pub z: i64,
    /// Wraps `MKTileOverlayPath.contentScaleFactor`.
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMultiPolylineState {
    coordinate: MKCoordinate,
    bounding_map_rect: MKMapRect,
    can_replace_map_content: bool,
    polyline_count: usize,
    polylines: Vec<Vec<MKCoordinate>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMultiPolygonState {
    coordinate: MKCoordinate,
    bounding_map_rect: MKMapRect,
    can_replace_map_content: bool,
    polygon_count: usize,
    polygons: Vec<Vec<MKCoordinate>>,
}

/// Wraps `MKCircle`.
#[derive(Debug)]
pub struct MKCircle {
    raw: NonNull<c_void>,
}

impl MKCircle {
    /// Creates a wrapper for `MKCircle`.
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

    /// Wraps `MKCircle.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKCircle.radius`.
    pub fn radius(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.radius)
    }

    /// Wraps `MKCircle.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKCircle.canReplaceMapContent`.
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

/// Wraps `MKPolyline`.
#[derive(Debug)]
pub struct MKPolyline {
    raw: NonNull<c_void>,
}

impl MKPolyline {
    /// Creates a wrapper for `MKPolyline`.
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

    /// Wraps `MKPolyline.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKPolyline.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKPolyline.canReplaceMapContent`.
    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    /// Wraps `MKPolyline.pointCount`.
    pub fn point_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.point_count)
    }

    /// Wraps `MKPolyline.coordinates`.
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

/// Wraps `MKGeodesicPolyline`.
#[derive(Debug)]
pub struct MKGeodesicPolyline {
    raw: NonNull<c_void>,
}

impl MKGeodesicPolyline {
    /// Creates a wrapper for `MKGeodesicPolyline`.
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

    /// Wraps `MKGeodesicPolyline.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKGeodesicPolyline.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKGeodesicPolyline.canReplaceMapContent`.
    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    /// Wraps `MKGeodesicPolyline.pointCount`.
    pub fn point_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.point_count)
    }

    /// Wraps `MKGeodesicPolyline.coordinates`.
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

/// Wraps `MKPolygon`.
#[derive(Debug)]
pub struct MKPolygon {
    raw: NonNull<c_void>,
}

impl MKPolygon {
    /// Creates a wrapper for `MKPolygon`.
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

    /// Wraps `MKPolygon.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKPolygon.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKPolygon.canReplaceMapContent`.
    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    /// Wraps `MKPolygon.pointCount`.
    pub fn point_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.point_count)
    }

    /// Wraps `MKPolygon.coordinates`.
    pub fn coordinates(&self) -> Result<Vec<MKCoordinate>, MapKitError> {
        Ok(self.state()?.coordinates)
    }

    /// Wraps `MKPolygon.interiorPolygonCount`.
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

/// Wraps `MKTileOverlay`.
#[derive(Debug)]
pub struct MKTileOverlay {
    raw: NonNull<c_void>,
}

impl MKTileOverlay {
    /// Creates a wrapper for `MKTileOverlay`.
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
            ffi::mk_tile_overlay_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKTileOverlay") }
    }

    /// Wraps `MKTileOverlay.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKTileOverlay.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKTileOverlay.canReplaceMapContent`.
    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    /// Wraps `MKTileOverlay.urlTemplate`.
    pub fn url_template(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.url_template)
    }

    /// Wraps `MKTileOverlay.tileSize`.
    pub fn tile_size(&self) -> Result<MKScreenSize, MapKitError> {
        Ok(self.state()?.tile_size)
    }

    /// Wraps `MKTileOverlay.tileSize`.
    pub fn set_tile_size(&self, tile_size: MKScreenSize) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            tile_size: Some(tile_size),
            ..MKTileOverlayOptions::default()
        })
    }

    /// Wraps `MKTileOverlay.isGeometryFlipped`.
    pub fn is_geometry_flipped(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.geometry_flipped)
    }

    /// Wraps `MKTileOverlay.geometryFlipped`.
    pub fn set_geometry_flipped(&self, geometry_flipped: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            geometry_flipped: Some(geometry_flipped),
            ..MKTileOverlayOptions::default()
        })
    }

    /// Wraps `MKTileOverlay.minimumZ`.
    pub fn minimum_z(&self) -> Result<i64, MapKitError> {
        Ok(self.state()?.minimum_z)
    }

    /// Wraps `MKTileOverlay.minimumZ`.
    pub fn set_minimum_z(&self, minimum_z: i64) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            minimum_z: Some(minimum_z),
            ..MKTileOverlayOptions::default()
        })
    }

    /// Wraps `MKTileOverlay.maximumZ`.
    pub fn maximum_z(&self) -> Result<i64, MapKitError> {
        Ok(self.state()?.maximum_z)
    }

    /// Wraps `MKTileOverlay.maximumZ`.
    pub fn set_maximum_z(&self, maximum_z: i64) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            maximum_z: Some(maximum_z),
            ..MKTileOverlayOptions::default()
        })
    }

    /// Wraps `MKTileOverlay.canReplaceMapContent`.
    pub fn set_can_replace_map_content(
        &self,
        can_replace_map_content: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKTileOverlayOptions {
            can_replace_map_content: Some(can_replace_map_content),
            ..MKTileOverlayOptions::default()
        })
    }

    /// Wraps `MKTileOverlay.urlForTilePath`.
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

/// Wraps `MKMultiPolyline`.
#[derive(Debug)]
pub struct MKMultiPolyline {
    raw: NonNull<c_void>,
}

impl MKMultiPolyline {
    /// Creates a wrapper for `MKMultiPolyline`.
    pub fn new(polylines: &[&MKPolyline]) -> Result<Self, MapKitError> {
        let raw_polylines: Vec<*mut c_void> =
            polylines.iter().map(|polyline| polyline.as_raw()).collect();
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_multi_polyline_new(raw_polylines.as_ptr(), raw_polylines.len(), &mut error)
        };
        let raw = owned_handle(raw, error, "failed to create MKMultiPolyline")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMultiPolylineState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_multi_polyline_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKMultiPolyline state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMultiPolyline state") }
        }
    }

    /// Wraps `MKMultiPolyline.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKMultiPolyline.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKMultiPolyline.canReplaceMapContent`.
    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    /// Wraps `MKMultiPolyline.polylineCount`.
    pub fn polyline_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.polyline_count)
    }

    /// Wraps `MKMultiPolyline.polylines`.
    pub fn polylines(&self) -> Result<Vec<Vec<MKCoordinate>>, MapKitError> {
        Ok(self.state()?.polylines)
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl MKAnnotation for MKMultiPolyline {
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

impl MKOverlay for MKMultiPolyline {
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

impl MKShape for MKMultiPolyline {}

impl Drop for MKMultiPolyline {
    fn drop(&mut self) {
        unsafe { ffi::mk_multi_polyline_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKMultiPolygon`.
#[derive(Debug)]
pub struct MKMultiPolygon {
    raw: NonNull<c_void>,
}

impl MKMultiPolygon {
    /// Creates a wrapper for `MKMultiPolygon`.
    pub fn new(polygons: &[&MKPolygon]) -> Result<Self, MapKitError> {
        let raw_polygons: Vec<*mut c_void> =
            polygons.iter().map(|polygon| polygon.as_raw()).collect();
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_multi_polygon_new(raw_polygons.as_ptr(), raw_polygons.len(), &mut error)
        };
        let raw = owned_handle(raw, error, "failed to create MKMultiPolygon")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMultiPolygonState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_multi_polygon_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKMultiPolygon state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMultiPolygon state") }
        }
    }

    /// Wraps `MKMultiPolygon.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKMultiPolygon.boundingMapRect`.
    pub fn bounding_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.bounding_map_rect)
    }

    /// Wraps `MKMultiPolygon.canReplaceMapContent`.
    pub fn can_replace_map_content(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_replace_map_content)
    }

    /// Wraps `MKMultiPolygon.polygonCount`.
    pub fn polygon_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.polygon_count)
    }

    /// Wraps `MKMultiPolygon.polygons`.
    pub fn polygons(&self) -> Result<Vec<Vec<MKCoordinate>>, MapKitError> {
        Ok(self.state()?.polygons)
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl MKAnnotation for MKMultiPolygon {
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

impl MKOverlay for MKMultiPolygon {
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

impl MKShape for MKMultiPolygon {}

impl Drop for MKMultiPolygon {
    fn drop(&mut self) {
        unsafe { ffi::mk_multi_polygon_release(self.raw.as_ptr()) };
    }
}
