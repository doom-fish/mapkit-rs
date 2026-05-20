use std::ptr;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::private::{json_cstring, parse_json_ptr};

/// Wraps `MKCoordinate`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKCoordinate {
    /// Wraps `MKCoordinate.latitude`.
    pub latitude: f64,
    /// Wraps `MKCoordinate.longitude`.
    pub longitude: f64,
}

impl MKCoordinate {
    /// Creates a wrapper for `MKCoordinate`.
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

/// Wraps `MKCoordinateSpan`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKCoordinateSpan {
    /// Wraps `MKCoordinateSpan.latitudeDelta`.
    pub latitude_delta: f64,
    /// Wraps `MKCoordinateSpan.longitudeDelta`.
    pub longitude_delta: f64,
}

impl MKCoordinateSpan {
    /// Creates a wrapper for `MKCoordinateSpan`.
    pub const fn new(latitude_delta: f64, longitude_delta: f64) -> Self {
        Self {
            latitude_delta,
            longitude_delta,
        }
    }
}

/// Wraps `MKCoordinateRegion`.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKCoordinateRegion {
    /// Wraps `MKCoordinateRegion.center`.
    pub center: MKCoordinate,
    /// Wraps `MKCoordinateRegion.span`.
    pub span: MKCoordinateSpan,
}

impl MKCoordinateRegion {
    /// Creates a wrapper for `MKCoordinateRegion`.
    pub const fn new(center: MKCoordinate, span: MKCoordinateSpan) -> Self {
        Self { center, span }
    }

    /// Wraps `MKCoordinateRegion.distance`.
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

    /// Wraps `MKCoordinateRegion.fromMapRect`.
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

/// Wraps `MKMapPoint`.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKMapPoint {
    /// Wraps `MKMapPoint.x`.
    pub x: f64,
    /// Wraps `MKMapPoint.y`.
    pub y: f64,
}

impl MKMapPoint {
    /// Creates a wrapper for `MKMapPoint`.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Wraps `MKMapPoint.fromCoordinate`.
    pub fn from_coordinate(coordinate: MKCoordinate) -> Result<Self, MapKitError> {
        let coordinate_json = json_cstring(&coordinate, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_map_point_for_coordinate_json(coordinate_json.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapPoint for coordinate failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapPoint") }
        }
    }

    /// Wraps `MKMapPoint.coordinate`.
    pub fn coordinate(self) -> Result<MKCoordinate, MapKitError> {
        let map_point_json = json_cstring(&self, "MKMapPoint")?;
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_coordinate_for_map_point_json(map_point_json.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "coordinate for MKMapPoint failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKCoordinate") }
        }
    }

    /// Wraps `MKMapPoint.distanceTo`.
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

    /// Wraps `MKMapPoint.equalTo`.
    pub fn equal_to(self, other: Self) -> bool {
        self == other
    }

    /// Wraps `MKMapPoint.stringRepresentation`.
    pub fn string_representation(self) -> String {
        format!("{{{:.1}, {:.1}}}", self.x, self.y)
    }
}

/// Wraps `MKMapSize`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKMapSize {
    /// Wraps `MKMapSize.width`.
    pub width: f64,
    /// Wraps `MKMapSize.height`.
    pub height: f64,
}

impl MKMapSize {
    /// Creates a wrapper for `MKMapSize`.
    pub const fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    /// Wraps `MKMapSize.world`.
    pub fn world() -> Result<Self, MapKitError> {
        geometry_constant(MKGeometryConstantKind::MapSizeWorld, "MKMapSizeWorld")
    }

    /// Wraps `MKMapSize.equalTo`.
    pub fn equal_to(self, other: Self) -> bool {
        self == other
    }

    /// Wraps `MKMapSize.stringRepresentation`.
    pub fn string_representation(self) -> String {
        format!("{{{:.1}, {:.1}}}", self.width, self.height)
    }
}

/// Wraps `MKMapRect`.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapRect {
    /// Wraps `MKMapRect.origin`.
    pub origin: MKMapPoint,
    /// Wraps `MKMapRect.size`.
    pub size: MKMapSize,
}

impl MKMapRect {
    /// Creates a wrapper for `MKMapRect`.
    pub const fn new(origin: MKMapPoint, size: MKMapSize) -> Self {
        Self { origin, size }
    }

    /// Wraps `MKMapRect.world`.
    pub fn world() -> Result<Self, MapKitError> {
        geometry_constant(MKGeometryConstantKind::MapRectWorld, "MKMapRectWorld")
    }

    /// Wraps `MKMapRect.null`.
    pub const fn null() -> Self {
        Self {
            origin: MKMapPoint::new(f64::INFINITY, f64::INFINITY),
            size: MKMapSize::new(0.0, 0.0),
        }
    }

    /// Wraps `MKMapRect.minX`.
    pub const fn min_x(self) -> f64 {
        self.origin.x
    }

    /// Wraps `MKMapRect.minY`.
    pub const fn min_y(self) -> f64 {
        self.origin.y
    }

    /// Wraps `MKMapRect.midX`.
    pub const fn mid_x(self) -> f64 {
        self.origin.x + self.size.width / 2.0
    }

    /// Wraps `MKMapRect.midY`.
    pub const fn mid_y(self) -> f64 {
        self.origin.y + self.size.height / 2.0
    }

    /// Wraps `MKMapRect.maxX`.
    pub const fn max_x(self) -> f64 {
        self.origin.x + self.size.width
    }

    /// Wraps `MKMapRect.maxY`.
    pub const fn max_y(self) -> f64 {
        self.origin.y + self.size.height
    }

    /// Wraps `MKMapRect.width`.
    pub const fn width(self) -> f64 {
        self.size.width
    }

    /// Wraps `MKMapRect.height`.
    pub const fn height(self) -> f64 {
        self.size.height
    }

    /// Wraps `MKMapRect.equalTo`.
    pub fn equal_to(self, other: Self) -> bool {
        self == other
    }

    /// Wraps `MKMapRect.isNull`.
    pub fn is_null(self) -> bool {
        self.origin.equal_to(Self::null().origin)
    }

    /// Wraps `MKMapRect.isEmpty`.
    pub fn is_empty(self) -> bool {
        self.is_null() || self.size.width == 0.0 || self.size.height == 0.0
    }

    /// Wraps `MKMapRect.union`.
    pub fn union(self, other: Self) -> Result<Self, MapKitError> {
        map_rect_binary_transform(
            self,
            other,
            MKMapRectTransformKind::Union,
            "MKMapRect union failed",
        )
    }

    /// Wraps `MKMapRect.intersection`.
    pub fn intersection(self, other: Self) -> Result<Self, MapKitError> {
        map_rect_binary_transform(
            self,
            other,
            MKMapRectTransformKind::Intersection,
            "MKMapRect intersection failed",
        )
    }

    /// Wraps `MKMapRect.inset`.
    pub fn inset(self, dx: f64, dy: f64) -> Result<Self, MapKitError> {
        map_rect_delta_transform(
            self,
            dx,
            dy,
            MKMapRectTransformKind::Inset,
            "MKMapRect inset failed",
        )
    }

    /// Wraps `MKMapRect.offset`.
    pub fn offset(self, dx: f64, dy: f64) -> Result<Self, MapKitError> {
        map_rect_delta_transform(
            self,
            dx,
            dy,
            MKMapRectTransformKind::Offset,
            "MKMapRect offset failed",
        )
    }

    /// Wraps `MKMapRect.divide`.
    pub fn divide(
        self,
        amount: f64,
        edge: MKMapRectEdge,
    ) -> Result<MKMapRectDivision, MapKitError> {
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

    /// Wraps `MKMapRect.containsPoint`.
    pub fn contains_point(self, point: MKMapPoint) -> Result<bool, MapKitError> {
        map_rect_predicate(
            self,
            Some(&point),
            MKMapRectPredicateKind::ContainsPoint,
            "MKMapRect contains point failed",
        )
    }

    /// Wraps `MKMapRect.containsRect`.
    pub fn contains_rect(self, other: Self) -> Result<bool, MapKitError> {
        map_rect_predicate(
            self,
            Some(&other),
            MKMapRectPredicateKind::ContainsRect,
            "MKMapRect contains rect failed",
        )
    }

    /// Wraps `MKMapRect.intersectsRect`.
    pub fn intersects_rect(self, other: Self) -> Result<bool, MapKitError> {
        map_rect_predicate(
            self,
            Some(&other),
            MKMapRectPredicateKind::IntersectsRect,
            "MKMapRect intersects rect failed",
        )
    }

    /// Wraps `MKMapRect.spans180thMeridian`.
    pub fn spans_180th_meridian(self) -> Result<bool, MapKitError> {
        map_rect_predicate::<Self>(
            self,
            None,
            MKMapRectPredicateKind::Spans180thMeridian,
            "MKMapRect spans 180th meridian failed",
        )
    }

    /// Wraps `MKMapRect.remainder`.
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

    /// Wraps `MKMapRect.stringRepresentation`.
    pub fn string_representation(self) -> String {
        format!(
            "{{{}, {}}}",
            self.origin.string_representation(),
            self.size.string_representation()
        )
    }
}

/// Wraps `MKMapRectEdge`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[repr(i32)]
pub enum MKMapRectEdge {
    MinX = 0,
    MinY = 1,
    MaxX = 2,
    MaxY = 3,
}

/// Wraps `MKMapRectDivision`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapRectDivision {
    /// Wraps `MKMapRectDivision.slice`.
    pub slice: MKMapRect,
    /// Wraps `MKMapRectDivision.remainder`.
    pub remainder: MKMapRect,
}

/// Wraps `MKScreenPoint`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKScreenPoint {
    /// Wraps `MKScreenPoint.x`.
    pub x: f64,
    /// Wraps `MKScreenPoint.y`.
    pub y: f64,
}

impl MKScreenPoint {
    /// Creates a wrapper for `MKScreenPoint`.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Wraps `MKScreenSize`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKScreenSize {
    /// Wraps `MKScreenSize.width`.
    pub width: f64,
    /// Wraps `MKScreenSize.height`.
    pub height: f64,
}

impl MKScreenSize {
    /// Creates a wrapper for `MKScreenSize`.
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
            auxiliary_json
                .as_ref()
                .map_or(ptr::null(), |value| value.as_ptr()),
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

/// Wraps `MKMapPointsPerMeterAtLatitude`.
pub fn mk_map_points_per_meter_at_latitude(latitude: f64) -> Result<f64, MapKitError> {
    let mut error = ptr::null_mut();
    let value = unsafe { ffi::mk_map_points_per_meter_at_latitude(latitude, &mut error) };
    scalar_result(value, error, "MKMapPointsPerMeterAtLatitude failed")
}

/// Wraps `MKMetersPerMapPointAtLatitude`.
pub fn mk_meters_per_map_point_at_latitude(latitude: f64) -> Result<f64, MapKitError> {
    let mut error = ptr::null_mut();
    let value = unsafe { ffi::mk_meters_per_map_point_at_latitude(latitude, &mut error) };
    scalar_result(value, error, "MKMetersPerMapPointAtLatitude failed")
}

/// Wraps `NSStringFromMKMapPoint`.
pub fn mk_string_from_map_point(point: MKMapPoint) -> String {
    point.string_representation()
}

/// Wraps `NSStringFromMKMapSize`.
pub fn mk_string_from_map_size(size: MKMapSize) -> String {
    size.string_representation()
}

/// Wraps `NSStringFromMKMapRect`.
pub fn mk_string_from_map_rect(rect: MKMapRect) -> String {
    rect.string_representation()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn assert_close(left: f64, right: f64) {
        assert!((left - right).abs() < 1.0e-6, "left={left}, right={right}");
    }

    #[test]
    fn coordinate_constructor_sets_fields() {
        let coordinate = MKCoordinate::new(37.3349, -122.0090);

        assert_close(coordinate.latitude, 37.3349);
        assert_close(coordinate.longitude, -122.0090);
    }

    #[test]
    fn coordinate_span_constructor_sets_deltas() {
        let span = MKCoordinateSpan::new(0.25, 0.5);

        assert_close(span.latitude_delta, 0.25);
        assert_close(span.longitude_delta, 0.5);
    }

    #[test]
    fn coordinate_region_serializes_with_camel_case_fields() {
        let region = MKCoordinateRegion::new(
            MKCoordinate::new(37.3349, -122.0090),
            MKCoordinateSpan::new(0.25, 0.5),
        );

        let value = serde_json::to_value(region).unwrap();

        assert_eq!(
            value,
            json!({
                "center": {
                    "latitude": 37.3349,
                    "longitude": -122.0090,
                },
                "span": {
                    "latitudeDelta": 0.25,
                    "longitudeDelta": 0.5,
                }
            })
        );
    }

    #[test]
    fn map_point_round_trips_through_coordinate_conversion() {
        let coordinate = MKCoordinate::new(37.3349, -122.0090);

        let point = MKMapPoint::from_coordinate(coordinate).unwrap();
        let round_trip = point.coordinate().unwrap();

        assert_close(round_trip.latitude, coordinate.latitude);
        assert_close(round_trip.longitude, coordinate.longitude);
    }

    #[test]
    fn map_rect_metrics_reflect_origin_and_size() {
        let rect = MKMapRect::new(MKMapPoint::new(10.0, 20.0), MKMapSize::new(30.0, 40.0));

        assert_close(rect.min_x(), 10.0);
        assert_close(rect.min_y(), 20.0);
        assert_close(rect.mid_x(), 25.0);
        assert_close(rect.mid_y(), 40.0);
        assert_close(rect.max_x(), 40.0);
        assert_close(rect.max_y(), 60.0);
        assert_close(rect.width(), 30.0);
        assert_close(rect.height(), 40.0);
    }

    #[test]
    fn null_map_rect_is_null_and_empty() {
        let rect = MKMapRect::null();

        assert!(rect.is_null());
        assert!(rect.is_empty());
        assert!(rect.min_x().is_infinite());
        assert!(rect.min_y().is_infinite());
    }

    #[test]
    fn string_helpers_match_type_representations() {
        let point = MKMapPoint::new(10.0, 20.0);
        let size = MKMapSize::new(30.0, 40.0);
        let rect = MKMapRect::new(point, size);

        assert_eq!(mk_string_from_map_point(point), point.string_representation());
        assert_eq!(mk_string_from_map_size(size), size.string_representation());
        assert_eq!(mk_string_from_map_rect(rect), rect.string_representation());
    }
}
