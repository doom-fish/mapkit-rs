use std::ptr;

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinate;
use crate::private::parse_json_ptr;

pub trait MKGeoJSONObject {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONPointAnnotation {
    pub coordinate: MKCoordinate,
    pub title: Option<String>,
    pub subtitle: Option<String>,
}

impl MKGeoJSONObject for MKGeoJSONPointAnnotation {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONPolyline {
    pub coordinates: Vec<MKCoordinate>,
}

impl MKGeoJSONObject for MKGeoJSONPolyline {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONPolygon {
    pub coordinates: Vec<MKCoordinate>,
}

impl MKGeoJSONObject for MKGeoJSONPolygon {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONMultiPolyline {
    pub polylines: Vec<Vec<MKCoordinate>>,
}

impl MKGeoJSONObject for MKGeoJSONMultiPolyline {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONMultiPolygon {
    pub polygons: Vec<Vec<MKCoordinate>>,
}

impl MKGeoJSONObject for MKGeoJSONMultiPolygon {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKGeoJSONFeature {
    pub identifier: Option<String>,
    pub properties: Option<String>,
    pub geometry: Vec<MKGeoJSONObjectValue>,
}

impl MKGeoJSONObject for MKGeoJSONFeature {}

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
                || Err(MapKitError::OperationFailed("missing MKGeoJSONFeature payload".into())),
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
                || Err(MapKitError::OperationFailed("missing MKGeoJSONPolyline payload".into())),
                |polyline| Ok(Self::Polyline(polyline)),
            ),
            "polygon" => payload.polygon.map_or_else(
                || Err(MapKitError::OperationFailed("missing MKGeoJSONPolygon payload".into())),
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

#[derive(Debug, Default, Clone, Copy)]
pub struct MKGeoJSONDecoder;

impl MKGeoJSONDecoder {
    pub const fn new() -> Self {
        Self
    }

    pub fn decode(&self, data: &[u8]) -> Result<Vec<MKGeoJSONObjectValue>, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_geojson_decode_json(data.as_ptr(), data.len(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKGeoJSONDecoder decode failed") })
        } else {
            let payloads: Vec<MKGeoJSONObjectPayload> = unsafe {
                parse_json_ptr(payload, "[MKGeoJSONObject]")
            }?;
            payloads.into_iter().map(TryInto::try_into).collect()
        }
    }

    pub fn decode_str(&self, geojson: &str) -> Result<Vec<MKGeoJSONObjectValue>, MapKitError> {
        self.decode(geojson.as_bytes())
    }
}
