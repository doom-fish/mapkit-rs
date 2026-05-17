use std::ptr;

use serde::de::DeserializeOwned;
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

    pub fn from_map_rect(map_rect: MKMapRect) -> Result<Self, MapKitError> {
        let map_rect_json = json_cstring(&map_rect, "MKMapRect")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_coordinate_region_for_map_rect_json(map_rect_json.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKCoordinateRegion for MKMapRect failed")
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
        scalar_result(distance, error, "meters between MKMapPoints failed")
    }

    pub fn equal_to(self, other: Self) -> bool {
        self == other
    }

    pub fn string_representation(self) -> String {
        format!("{{{:.1}, {:.1}}}", self.x, self.y)
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

    pub fn world() -> Result<Self, MapKitError> {
        geometry_constant(MKGeometryConstantKind::MapSizeWorld, "MKMapSizeWorld")
    }

    pub fn equal_to(self, other: Self) -> bool {
        self == other
    }

    pub fn string_representation(self) -> String {
        format!("{{{:.1}, {:.1}}}", self.width, self.height)
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
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

    pub fn world() -> Result<Self, MapKitError> {
        geometry_constant(MKGeometryConstantKind::MapRectWorld, "MKMapRectWorld")
    }

    pub const fn null() -> Self {
        Self {
            origin: MKMapPoint::new(f64::INFINITY, f64::INFINITY),
            size: MKMapSize::new(0.0, 0.0),
        }
    }

    pub const fn min_x(self) -> f64 {
        self.origin.x
    }

    pub const fn min_y(self) -> f64 {
        self.origin.y
    }

    pub const fn mid_x(self) -> f64 {
        self.origin.x + self.size.width / 2.0
    }

    pub const fn mid_y(self) -> f64 {
        self.origin.y + self.size.height / 2.0
    }

    pub const fn max_x(self) -> f64 {
        self.origin.x + self.size.width
    }

    pub const fn max_y(self) -> f64 {
        self.origin.y + self.size.height
    }

    pub const fn width(self) -> f64 {
        self.size.width
    }

    pub const fn height(self) -> f64 {
        self.size.height
    }

    pub fn equal_to(self, other: Self) -> bool {
        self == other
    }

    pub fn is_null(self) -> bool {
        self.origin.equal_to(Self::null().origin)
    }

    pub fn is_empty(self) -> bool {
        self.is_null() || self.size.width == 0.0 || self.size.height == 0.0
    }

    pub fn union(self, other: Self) -> Result<Self, MapKitError> {
        map_rect_binary_transform(
            self,
            other,
            MKMapRectTransformKind::Union,
            "MKMapRect union failed",
        )
    }

    pub fn intersection(self, other: Self) -> Result<Self, MapKitError> {
        map_rect_binary_transform(
            self,
            other,
            MKMapRectTransformKind::Intersection,
            "MKMapRect intersection failed",
        )
    }

    pub fn inset(self, dx: f64, dy: f64) -> Result<Self, MapKitError> {
        map_rect_delta_transform(
            self,
            dx,
            dy,
            MKMapRectTransformKind::Inset,
            "MKMapRect inset failed",
        )
    }

    pub fn offset(self, dx: f64, dy: f64) -> Result<Self, MapKitError> {
        map_rect_delta_transform(
            self,
            dx,
            dy,
            MKMapRectTransformKind::Offset,
            "MKMapRect offset failed",
        )
    }

    pub fn divide(self, amount: f64, edge: MKMapRectEdge) -> Result<MKMapRectDivision, MapKitError> {
        let rect_json = json_cstring(&self, "MKMapRect")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_rect_transform_json(
                rect_json.as_ptr(),
                ptr::null(),
                0.0,
                0.0,
                amount,
                edge as i32,
                MKMapRectTransformKind::Divide as i32,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapRect divide failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapRectDivision") }
        }
    }

    pub fn contains_point(self, point: MKMapPoint) -> Result<bool, MapKitError> {
        map_rect_predicate(
            self,
            Some(&point),
            MKMapRectPredicateKind::ContainsPoint,
            "MKMapRect contains point failed",
        )
    }

    pub fn contains_rect(self, other: Self) -> Result<bool, MapKitError> {
        map_rect_predicate(
            self,
            Some(&other),
            MKMapRectPredicateKind::ContainsRect,
            "MKMapRect contains rect failed",
        )
    }

    pub fn intersects_rect(self, other: Self) -> Result<bool, MapKitError> {
        map_rect_predicate(
            self,
            Some(&other),
            MKMapRectPredicateKind::IntersectsRect,
            "MKMapRect intersects rect failed",
        )
    }

    pub fn spans_180th_meridian(self) -> Result<bool, MapKitError> {
        map_rect_predicate::<Self>(
            self,
            None,
            MKMapRectPredicateKind::Spans180thMeridian,
            "MKMapRect spans 180th meridian failed",
        )
    }

    pub fn remainder(self) -> Result<Self, MapKitError> {
        let rect_json = json_cstring(&self, "MKMapRect")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_rect_transform_json(
                rect_json.as_ptr(),
                ptr::null(),
                0.0,
                0.0,
                0.0,
                MKMapRectEdge::MinX as i32,
                MKMapRectTransformKind::Remainder as i32,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapRect remainder failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapRect") }
        }
    }

    pub fn string_representation(self) -> String {
        format!(
            "{{{}, {}}}",
            self.origin.string_representation(),
            self.size.string_representation()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(i32)]
pub enum MKMapRectEdge {
    MinX = 0,
    MinY = 1,
    MaxX = 2,
    MaxY = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapRectDivision {
    pub slice: MKMapRect,
    pub remainder: MKMapRect,
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

#[derive(Clone, Copy)]
#[repr(i32)]
enum MKGeometryConstantKind {
    MapSizeWorld = 0,
    MapRectWorld = 1,
}

#[derive(Clone, Copy)]
#[repr(i32)]
enum MKMapRectPredicateKind {
    ContainsPoint = 0,
    ContainsRect = 1,
    IntersectsRect = 2,
    Spans180thMeridian = 3,
}

#[derive(Clone, Copy)]
#[repr(i32)]
enum MKMapRectTransformKind {
    Union = 0,
    Intersection = 1,
    Inset = 2,
    Offset = 3,
    Divide = 4,
    Remainder = 5,
}

fn geometry_constant<T: DeserializeOwned>(
    kind: MKGeometryConstantKind,
    context: &str,
) -> Result<T, MapKitError> {
    let mut error = ptr::null_mut();
    let payload = unsafe { ffi::mk_geometry_constant_json(kind as i32, &mut error) };
    if payload.is_null() {
        Err(unsafe { MapKitError::from_error_ptr(error, context) })
    } else {
        unsafe { parse_json_ptr(payload, context) }
    }
}

fn scalar_result(
    value: f64,
    error: *mut core::ffi::c_char,
    context: &str,
) -> Result<f64, MapKitError> {
    if error.is_null() {
        Ok(value)
    } else {
        Err(unsafe { MapKitError::from_error_ptr(error, context) })
    }
}

fn map_rect_predicate<T: Serialize>(
    rect: MKMapRect,
    auxiliary: Option<&T>,
    kind: MKMapRectPredicateKind,
    context: &str,
) -> Result<bool, MapKitError> {
    let rect_json = json_cstring(&rect, "MKMapRect")?;
    let auxiliary_json = auxiliary
        .map(|value| json_cstring(value, context))
        .transpose()?;
    let mut error = ptr::null_mut();
    let value = unsafe {
        ffi::mk_map_rect_predicate_json(
            rect_json.as_ptr(),
            auxiliary_json.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
            kind as i32,
            &mut error,
        )
    };
    if error.is_null() {
        Ok(value)
    } else {
        Err(unsafe { MapKitError::from_error_ptr(error, context) })
    }
}

fn map_rect_binary_transform(
    rect: MKMapRect,
    other: MKMapRect,
    kind: MKMapRectTransformKind,
    context: &str,
) -> Result<MKMapRect, MapKitError> {
    let rect_json = json_cstring(&rect, "MKMapRect")?;
    let other_json = json_cstring(&other, "other MKMapRect")?;
    let mut error = ptr::null_mut();
    let payload = unsafe {
        ffi::mk_map_rect_transform_json(
            rect_json.as_ptr(),
            other_json.as_ptr(),
            0.0,
            0.0,
            0.0,
            MKMapRectEdge::MinX as i32,
            kind as i32,
            &mut error,
        )
    };
    if payload.is_null() {
        Err(unsafe { MapKitError::from_error_ptr(error, context) })
    } else {
        unsafe { parse_json_ptr(payload, "MKMapRect") }
    }
}

fn map_rect_delta_transform(
    rect: MKMapRect,
    dx: f64,
    dy: f64,
    kind: MKMapRectTransformKind,
    context: &str,
) -> Result<MKMapRect, MapKitError> {
    let rect_json = json_cstring(&rect, "MKMapRect")?;
    let mut error = ptr::null_mut();
    let payload = unsafe {
        ffi::mk_map_rect_transform_json(
            rect_json.as_ptr(),
            ptr::null(),
            dx,
            dy,
            0.0,
            MKMapRectEdge::MinX as i32,
            kind as i32,
            &mut error,
        )
    };
    if payload.is_null() {
        Err(unsafe { MapKitError::from_error_ptr(error, context) })
    } else {
        unsafe { parse_json_ptr(payload, "MKMapRect") }
    }
}

pub fn mk_map_points_per_meter_at_latitude(latitude: f64) -> Result<f64, MapKitError> {
    let mut error = ptr::null_mut();
    let value = unsafe { ffi::mk_map_points_per_meter_at_latitude(latitude, &mut error) };
    scalar_result(value, error, "MKMapPointsPerMeterAtLatitude failed")
}

pub fn mk_meters_per_map_point_at_latitude(latitude: f64) -> Result<f64, MapKitError> {
    let mut error = ptr::null_mut();
    let value = unsafe { ffi::mk_meters_per_map_point_at_latitude(latitude, &mut error) };
    scalar_result(value, error, "MKMetersPerMapPointAtLatitude failed")
}

pub fn mk_string_from_map_point(point: MKMapPoint) -> String {
    point.string_representation()
}

pub fn mk_string_from_map_size(size: MKMapSize) -> String {
    size.string_representation()
}

pub fn mk_string_from_map_rect(rect: MKMapRect) -> String {
    rect.string_representation()
}
