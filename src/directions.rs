use core::ffi::c_void;
use std::ops::{BitOr, BitOrAssign};
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::map_item::MKMapItem;
use crate::private::{json_cstring, owned_handle, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKDirectionsTransportType(pub u64);

impl MKDirectionsTransportType {
    pub const AUTOMOBILE: Self = Self(1 << 0);
    pub const WALKING: Self = Self(1 << 1);
    pub const TRANSIT: Self = Self(1 << 2);
    pub const CYCLING: Self = Self(1 << 3);
    pub const ANY: Self = Self(0x0FFF_FFFF);

    pub const fn bits(self) -> u64 {
        self.0
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKDirectionsRoutePreference {
    #[default]
    Any,
    Avoid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKDirectionsRequest {
    pub source: MKMapItem,
    pub destination: MKMapItem,
    #[serde(default)]
    pub transport_type: MKDirectionsTransportType,
    #[serde(default)]
    pub requests_alternate_routes: bool,
    #[serde(default)]
    pub toll_preference: MKDirectionsRoutePreference,
    #[serde(default)]
    pub highway_preference: MKDirectionsRoutePreference,
}

impl MKDirectionsRequest {
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

    pub fn with_transport_type(mut self, transport_type: MKDirectionsTransportType) -> Self {
        self.transport_type = transport_type;
        self
    }

    pub fn with_alternate_routes(mut self, requests_alternate_routes: bool) -> Self {
        self.requests_alternate_routes = requests_alternate_routes;
        self
    }

    pub fn with_toll_preference(
        mut self,
        toll_preference: MKDirectionsRoutePreference,
    ) -> Self {
        self.toll_preference = toll_preference;
        self
    }

    pub fn with_highway_preference(
        mut self,
        highway_preference: MKDirectionsRoutePreference,
    ) -> Self {
        self.highway_preference = highway_preference;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKRouteStep {
    pub instructions: String,
    pub notice: Option<String>,
    pub distance: f64,
    pub transport_type: MKDirectionsTransportType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKRoute {
    pub name: String,
    pub advisory_notices: Vec<String>,
    pub distance: f64,
    pub expected_travel_time: f64,
    pub transport_type: MKDirectionsTransportType,
    pub has_tolls: bool,
    pub has_highways: bool,
    pub steps: Vec<MKRouteStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKDirectionsResponse {
    pub source: MKMapItem,
    pub destination: MKMapItem,
    pub routes: Vec<MKRoute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKETAResponse {
    pub source: MKMapItem,
    pub destination: MKMapItem,
    pub expected_travel_time: f64,
    pub distance: f64,
    pub expected_arrival_date: Option<String>,
    pub expected_departure_date: Option<String>,
    pub transport_type: MKDirectionsTransportType,
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
        let raw = owned_handle(raw, error, "failed to create MKDirections")?;
        Ok(Self { raw })
    }

    pub fn calculate(&self) -> Result<MKDirectionsResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_directions_calculate_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKDirections calculate failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKDirectionsResponse") }
        }
    }

    pub fn calculate_eta(&self) -> Result<MKETAResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_directions_calculate_eta_json(self.raw.as_ptr(), &mut error) };
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
