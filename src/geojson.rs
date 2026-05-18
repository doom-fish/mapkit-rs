use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinate;
use crate::private::parse_json_ptr;

/// Wraps `MKGeoJSONObject`.
pub trait MKGeoJSONObject {}

/// Wraps `MKGeoJSONPointAnnotation`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONPointAnnotation {
    /// Wraps `MKGeoJSONPointAnnotation.coordinate`.
    pub coordinate: MKCoordinate,
    /// Wraps `MKGeoJSONPointAnnotation.title`.
    pub title: Option<String>,
    /// Wraps `MKGeoJSONPointAnnotation.subtitle`.
    pub subtitle: Option<String>,
}

impl MKGeoJSONObject for MKGeoJSONPointAnnotation {}

/// Wraps `MKGeoJSONPolyline`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONPolyline {
    /// Wraps `MKGeoJSONPolyline.coordinates`.
    pub coordinates: Vec<MKCoordinate>,
}

impl MKGeoJSONObject for MKGeoJSONPolyline {}

/// Wraps `MKGeoJSONPolygon`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONPolygon {
    /// Wraps `MKGeoJSONPolygon.coordinates`.
    pub coordinates: Vec<MKCoordinate>,
}

impl MKGeoJSONObject for MKGeoJSONPolygon {}

/// Wraps `MKGeoJSONMultiPolyline`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONMultiPolyline {
    /// Wraps `MKGeoJSONMultiPolyline.polylines`.
    pub polylines: Vec<Vec<MKCoordinate>>,
}

impl MKGeoJSONObject for MKGeoJSONMultiPolyline {}

/// Wraps `MKGeoJSONMultiPolygon`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONMultiPolygon {
    /// Wraps `MKGeoJSONMultiPolygon.polygons`.
    pub polygons: Vec<Vec<MKCoordinate>>,
}

impl MKGeoJSONObject for MKGeoJSONMultiPolygon {}

/// Wraps `MKGeoJSONFeature`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONFeature {
    /// Wraps `MKGeoJSONFeature.identifier`.
    pub identifier: Option<String>,
    /// Wraps `MKGeoJSONFeature.properties`.
    pub properties: Option<String>,
    /// Wraps `MKGeoJSONFeature.geometry`.
    pub geometry: Vec<MKGeoJSONObjectValue>,
}

impl MKGeoJSONObject for MKGeoJSONFeature {}

/// Wraps `MKGeoJSONObjectValue`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MKGeoJSONObjectValue {
    Feature(MKGeoJSONFeature),
    PointAnnotation(MKGeoJSONPointAnnotation),
    Polyline(MKGeoJSONPolyline),
    Polygon(MKGeoJSONPolygon),
    MultiPolyline(MKGeoJSONMultiPolyline),
    MultiPolygon(MKGeoJSONMultiPolygon),
}

impl MKGeoJSONObject for MKGeoJSONObjectValue {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKGeoJSONObjectPayload {
    kind: String,
    feature: Option<MKGeoJSONFeaturePayload>,
    point_annotation: Option<MKGeoJSONPointAnnotation>,
    polyline: Option<MKGeoJSONPolyline>,
    polygon: Option<MKGeoJSONPolygon>,
    multi_polyline: Option<MKGeoJSONMultiPolyline>,
    multi_polygon: Option<MKGeoJSONMultiPolygon>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKGeoJSONFeaturePayload {
    identifier: Option<String>,
    properties: Option<String>,
    geometry: Vec<MKGeoJSONObjectPayload>,
}

impl TryFrom<MKGeoJSONObjectPayload> for MKGeoJSONObjectValue {
    type Error = MapKitError;

    fn try_from(payload: MKGeoJSONObjectPayload) -> Result<Self, Self::Error> {
        match payload.kind.as_str() {
            "feature" => payload.feature.map_or_else(
                || {
                    Err(MapKitError::OperationFailed(
                        "missing MKGeoJSONFeature payload".into(),
                    ))
                },
                |feature| {
                    Ok(Self::Feature(MKGeoJSONFeature {
                        identifier: feature.identifier,
                        properties: feature.properties,
                        geometry: feature
                            .geometry
                            .into_iter()
                            .map(TryInto::try_into)
                            .collect::<Result<Vec<_>, _>>()?,
                    }))
                },
            ),
            "pointAnnotation" => payload.point_annotation.map_or_else(
                || {
                    Err(MapKitError::OperationFailed(
                        "missing MKGeoJSONPointAnnotation payload".into(),
                    ))
                },
                |point_annotation| Ok(Self::PointAnnotation(point_annotation)),
            ),
            "polyline" => payload.polyline.map_or_else(
                || {
                    Err(MapKitError::OperationFailed(
                        "missing MKGeoJSONPolyline payload".into(),
                    ))
                },
                |polyline| Ok(Self::Polyline(polyline)),
            ),
            "polygon" => payload.polygon.map_or_else(
                || {
                    Err(MapKitError::OperationFailed(
                        "missing MKGeoJSONPolygon payload".into(),
                    ))
                },
                |polygon| Ok(Self::Polygon(polygon)),
            ),
            "multiPolyline" => payload.multi_polyline.map_or_else(
                || {
                    Err(MapKitError::OperationFailed(
                        "missing MKGeoJSONMultiPolyline payload".into(),
                    ))
                },
                |multi_polyline| Ok(Self::MultiPolyline(multi_polyline)),
            ),
            "multiPolygon" => payload.multi_polygon.map_or_else(
                || {
                    Err(MapKitError::OperationFailed(
                        "missing MKGeoJSONMultiPolygon payload".into(),
                    ))
                },
                |multi_polygon| Ok(Self::MultiPolygon(multi_polygon)),
            ),
            other => Err(MapKitError::OperationFailed(format!(
                "unsupported MKGeoJSONObject payload kind: {other}",
            ))),
        }
    }
}

/// Wraps `MKGeoJSONDecoder`.
#[derive(Debug, Default, Clone, Copy)]
pub struct MKGeoJSONDecoder;

impl MKGeoJSONDecoder {
    /// Creates a wrapper for `MKGeoJSONDecoder`.
    pub const fn new() -> Self {
        Self
    }

    /// Wraps `MKGeoJSONDecoder.decode`.
    pub fn decode(&self, data: &[u8]) -> Result<Vec<MKGeoJSONObjectValue>, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_geojson_decode_json(data.as_ptr(), data.len(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKGeoJSONDecoder decode failed") })
        } else {
            let payloads: Vec<MKGeoJSONObjectPayload> =
                unsafe { parse_json_ptr(payload, "[MKGeoJSONObject]") }?;
            payloads.into_iter().map(TryInto::try_into).collect()
        }
    }

    /// Wraps `MKGeoJSONDecoder.decodeStr`.
    pub fn decode_str(&self, geojson: &str) -> Result<Vec<MKGeoJSONObjectValue>, MapKitError> {
        self.decode(geojson.as_bytes())
    }
}
