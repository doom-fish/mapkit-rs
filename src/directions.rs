use core::ffi::c_void;
use std::ops::{BitOr, BitOrAssign};
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::map_item::MKMapItem;
use crate::private::{json_cstring, owned_handle, parse_json_ptr};

/// Wraps `MKDirectionsTransportType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKDirectionsTransportType(pub u64);

impl MKDirectionsTransportType {
    /// Wraps `MKDirectionsTransportType.automobile`.
    pub const AUTOMOBILE: Self = Self(1 << 0);
    /// Wraps `MKDirectionsTransportType.walking`.
    pub const WALKING: Self = Self(1 << 1);
    /// Wraps `MKDirectionsTransportType.transit`.
    pub const TRANSIT: Self = Self(1 << 2);
    /// Wraps `MKDirectionsTransportType.cycling`.
    pub const CYCLING: Self = Self(1 << 3);
    /// Wraps `MKDirectionsTransportType.any`.
    pub const ANY: Self = Self(0x0FFF_FFFF);

    /// Wraps `MKDirectionsTransportType.bits`.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps `MKDirectionsTransportType.contains`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Default for MKDirectionsTransportType {
    fn default() -> Self {
        Self::ANY
    }
}

impl BitOr for MKDirectionsTransportType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MKDirectionsTransportType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Wraps `MKDirectionsRoutePreference`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKDirectionsRoutePreference {
    #[default]
    Any,
    Avoid,
}

/// Wraps `MKDirectionsRequest`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKDirectionsRequest {
    /// Wraps `MKDirectionsRequest.source`.
    pub source: MKMapItem,
    /// Wraps `MKDirectionsRequest.destination`.
    pub destination: MKMapItem,
    /// Wraps `MKDirectionsRequest.transportType`.
    #[serde(default)]
    pub transport_type: MKDirectionsTransportType,
    /// Wraps `MKDirectionsRequest.requestsAlternateRoutes`.
    #[serde(default)]
    pub requests_alternate_routes: bool,
    /// Wraps `MKDirectionsRequest.tollPreference`.
    #[serde(default)]
    pub toll_preference: MKDirectionsRoutePreference,
    /// Wraps `MKDirectionsRequest.highwayPreference`.
    #[serde(default)]
    pub highway_preference: MKDirectionsRoutePreference,
}

impl MKDirectionsRequest {
    /// Creates a wrapper for `MKDirectionsRequest`.
    pub fn new(source: MKMapItem, destination: MKMapItem) -> Self {
        Self {
            source,
            destination,
            transport_type: MKDirectionsTransportType::default(),
            requests_alternate_routes: false,
            toll_preference: MKDirectionsRoutePreference::default(),
            highway_preference: MKDirectionsRoutePreference::default(),
        }
    }

    /// Wraps `MKDirectionsRequest.transportType`.
    pub fn with_transport_type(mut self, transport_type: MKDirectionsTransportType) -> Self {
        self.transport_type = transport_type;
        self
    }

    /// Wraps `MKDirectionsRequest.alternateRoutes`.
    pub fn with_alternate_routes(mut self, requests_alternate_routes: bool) -> Self {
        self.requests_alternate_routes = requests_alternate_routes;
        self
    }

    /// Wraps `MKDirectionsRequest.tollPreference`.
    pub fn with_toll_preference(mut self, toll_preference: MKDirectionsRoutePreference) -> Self {
        self.toll_preference = toll_preference;
        self
    }

    /// Wraps `MKDirectionsRequest.highwayPreference`.
    pub fn with_highway_preference(
        mut self,
        highway_preference: MKDirectionsRoutePreference,
    ) -> Self {
        self.highway_preference = highway_preference;
        self
    }
}

/// Wraps `MKRouteStep`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKRouteStep {
    /// Wraps `MKRouteStep.instructions`.
    pub instructions: String,
    /// Wraps `MKRouteStep.notice`.
    pub notice: Option<String>,
    /// Wraps `MKRouteStep.distance`.
    pub distance: f64,
    /// Wraps `MKRouteStep.transportType`.
    pub transport_type: MKDirectionsTransportType,
}

/// Wraps `MKRoute`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKRoute {
    /// Wraps `MKRoute.name`.
    pub name: String,
    /// Wraps `MKRoute.advisoryNotices`.
    pub advisory_notices: Vec<String>,
    /// Wraps `MKRoute.distance`.
    pub distance: f64,
    /// Wraps `MKRoute.expectedTravelTime`.
    pub expected_travel_time: f64,
    /// Wraps `MKRoute.transportType`.
    pub transport_type: MKDirectionsTransportType,
    /// Wraps `MKRoute.hasTolls`.
    pub has_tolls: bool,
    /// Wraps `MKRoute.hasHighways`.
    pub has_highways: bool,
    /// Wraps `MKRoute.steps`.
    pub steps: Vec<MKRouteStep>,
}

/// Wraps `MKDirectionsResponse`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKDirectionsResponse {
    /// Wraps `MKDirectionsResponse.source`.
    pub source: MKMapItem,
    /// Wraps `MKDirectionsResponse.destination`.
    pub destination: MKMapItem,
    /// Wraps `MKDirectionsResponse.routes`.
    pub routes: Vec<MKRoute>,
}

/// Wraps `MKETAResponse`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKETAResponse {
    /// Wraps `MKETAResponse.source`.
    pub source: MKMapItem,
    /// Wraps `MKETAResponse.destination`.
    pub destination: MKMapItem,
    /// Wraps `MKETAResponse.expectedTravelTime`.
    pub expected_travel_time: f64,
    /// Wraps `MKETAResponse.distance`.
    pub distance: f64,
    /// Wraps `MKETAResponse.expectedArrivalDate`.
    pub expected_arrival_date: Option<String>,
    /// Wraps `MKETAResponse.expectedDepartureDate`.
    pub expected_departure_date: Option<String>,
    /// Wraps `MKETAResponse.transportType`.
    pub transport_type: MKDirectionsTransportType,
}

/// Wraps `MKDirections`.
#[derive(Debug)]
pub struct MKDirections {
    raw: NonNull<c_void>,
}

impl MKDirections {
    /// Creates a wrapper for `MKDirections`.
    pub fn new(request: &MKDirectionsRequest) -> Result<Self, MapKitError> {
        let request_json = json_cstring(request, "MKDirectionsRequest")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_directions_new(request_json.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKDirections")?;
        Ok(Self { raw })
    }

    /// Wraps `MKDirections.calculate`.
    pub fn calculate(&self) -> Result<MKDirectionsResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_directions_calculate_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKDirections calculate failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKDirectionsResponse") }
        }
    }

    /// Wraps `MKDirections.calculateEta`.
    pub fn calculate_eta(&self) -> Result<MKETAResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_directions_calculate_eta_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKDirections ETA failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKETAResponse") }
        }
    }

    /// Wraps `MKDirections.isCalculating`.
    pub fn is_calculating(&self) -> bool {
        unsafe { ffi::mk_directions_is_calculating(self.raw.as_ptr()) }
    }

    /// Wraps `MKDirections.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_directions_cancel(self.raw.as_ptr()) };
    }

    #[cfg(feature = "async")]
    pub(crate) fn into_raw(self) -> *mut c_void {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for MKDirections {
    fn drop(&mut self) {
        unsafe { ffi::mk_directions_release(self.raw.as_ptr()) };
    }
}
