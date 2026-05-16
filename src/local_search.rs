use core::ffi::c_void;
use std::ops::{BitOr, BitOrAssign};
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::address::MKAddressFilter;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinateRegion;
use crate::map_item::MKMapItem;
use crate::point_of_interest::{MKLocalPointsOfInterestRequest, MKPointOfInterestFilter};
use crate::private::{json_cstring, owned_handle, parse_json_ptr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKLocalSearchResultType(pub u64);

impl MKLocalSearchResultType {
    pub const ADDRESS: Self = Self(1 << 0);
    pub const POINT_OF_INTEREST: Self = Self(1 << 1);
    pub const PHYSICAL_FEATURE: Self = Self(1 << 2);
    pub const ALL: Self =
        Self(Self::ADDRESS.0 | Self::POINT_OF_INTEREST.0 | Self::PHYSICAL_FEATURE.0);

    pub const fn bits(self) -> u64 {
        self.0
    }

    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Default for MKLocalSearchResultType {
    fn default() -> Self {
        Self(Self::ADDRESS.0 | Self::POINT_OF_INTEREST.0)
    }
}

impl BitOr for MKLocalSearchResultType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MKLocalSearchResultType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKLocalSearchRegionPriority {
    #[default]
    Default,
    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchRequest {
    pub natural_language_query: String,
    pub region: Option<MKCoordinateRegion>,
    #[serde(default)]
    pub result_types: MKLocalSearchResultType,
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
    pub address_filter: Option<MKAddressFilter>,
    #[serde(default)]
    pub region_priority: MKLocalSearchRegionPriority,
}

impl MKLocalSearchRequest {
    pub fn new(natural_language_query: impl Into<String>) -> Self {
        Self {
            natural_language_query: natural_language_query.into(),
            region: None,
            result_types: MKLocalSearchResultType::default(),
            point_of_interest_filter: None,
            address_filter: None,
            region_priority: MKLocalSearchRegionPriority::default(),
        }
    }

    pub fn with_region(mut self, region: MKCoordinateRegion) -> Self {
        self.region = Some(region);
        self
    }

    pub fn with_result_types(mut self, result_types: MKLocalSearchResultType) -> Self {
        self.result_types = result_types;
        self
    }

    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    pub fn with_address_filter(mut self, address_filter: MKAddressFilter) -> Self {
        self.address_filter = Some(address_filter);
        self
    }

    pub fn with_region_priority(mut self, region_priority: MKLocalSearchRegionPriority) -> Self {
        self.region_priority = region_priority;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchResponse {
    pub map_items: Vec<MKMapItem>,
    pub bounding_region: MKCoordinateRegion,
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
        let raw = owned_handle(raw, error, "failed to create MKLocalSearch")?;
        Ok(Self { raw })
    }

    pub fn from_points_of_interest_request(
        request: &MKLocalPointsOfInterestRequest,
    ) -> Result<Self, MapKitError> {
        request.validate()?;
        let request_json = json_cstring(request, "MKLocalPointsOfInterestRequest")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_local_search_new_points_of_interest(request_json.as_ptr(), &mut error)
        };
        let raw = owned_handle(
            raw,
            error,
            "failed to create MKLocalSearch from MKLocalPointsOfInterestRequest",
        )?;
        Ok(Self { raw })
    }

    pub fn search(request: &MKLocalSearchRequest) -> Result<MKLocalSearchResponse, MapKitError> {
        Self::new(request)?.start()
    }

    pub fn search_points_of_interest(
        request: &MKLocalPointsOfInterestRequest,
    ) -> Result<MKLocalSearchResponse, MapKitError> {
        Self::from_points_of_interest_request(request)?.start()
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
