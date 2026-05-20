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

/// Wraps `MKLocalSearchResultType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKLocalSearchResultType(pub u64);

impl MKLocalSearchResultType {
    /// Wraps `MKLocalSearchResultType.address`.
    pub const ADDRESS: Self = Self(1 << 0);
    /// Wraps `MKLocalSearchResultType.pointOfInterest`.
    pub const POINT_OF_INTEREST: Self = Self(1 << 1);
    /// Wraps `MKLocalSearchResultType.physicalFeature`.
    pub const PHYSICAL_FEATURE: Self = Self(1 << 2);
    /// Wraps `MKLocalSearchResultType.all`.
    pub const ALL: Self =
        Self(Self::ADDRESS.0 | Self::POINT_OF_INTEREST.0 | Self::PHYSICAL_FEATURE.0);

    /// Wraps `MKLocalSearchResultType.bits`.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps `MKLocalSearchResultType.contains`.
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

/// Wraps `MKLocalSearchRegionPriority`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKLocalSearchRegionPriority {
    #[default]
    Default,
    Required,
}

/// Wraps `MKLocalSearchRequest`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchRequest {
    /// Wraps `MKLocalSearchRequest.naturalLanguageQuery`.
    pub natural_language_query: String,
    /// Wraps `MKLocalSearchRequest.region`.
    pub region: Option<MKCoordinateRegion>,
    /// Wraps `MKLocalSearchRequest.resultTypes`.
    #[serde(default)]
    pub result_types: MKLocalSearchResultType,
    /// Wraps `MKLocalSearchRequest.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
    /// Wraps `MKLocalSearchRequest.addressFilter`.
    pub address_filter: Option<MKAddressFilter>,
    /// Wraps `MKLocalSearchRequest.regionPriority`.
    #[serde(default)]
    pub region_priority: MKLocalSearchRegionPriority,
}

impl MKLocalSearchRequest {
    /// Creates a wrapper for `MKLocalSearchRequest`.
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

    /// Wraps `MKLocalSearchRequest.region`.
    pub fn with_region(mut self, region: MKCoordinateRegion) -> Self {
        self.region = Some(region);
        self
    }

    /// Wraps `MKLocalSearchRequest.resultTypes`.
    pub fn with_result_types(mut self, result_types: MKLocalSearchResultType) -> Self {
        self.result_types = result_types;
        self
    }

    /// Wraps `MKLocalSearchRequest.pointOfInterestFilter`.
    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    /// Wraps `MKLocalSearchRequest.addressFilter`.
    pub fn with_address_filter(mut self, address_filter: MKAddressFilter) -> Self {
        self.address_filter = Some(address_filter);
        self
    }

    /// Wraps `MKLocalSearchRequest.regionPriority`.
    pub fn with_region_priority(mut self, region_priority: MKLocalSearchRegionPriority) -> Self {
        self.region_priority = region_priority;
        self
    }
}

/// Wraps `MKLocalSearchResponse`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchResponse {
    /// Wraps `MKLocalSearchResponse.mapItems`.
    pub map_items: Vec<MKMapItem>,
    /// Wraps `MKLocalSearchResponse.boundingRegion`.
    pub bounding_region: MKCoordinateRegion,
}

/// Wraps `MKLocalSearch`.
#[derive(Debug)]
pub struct MKLocalSearch {
    raw: NonNull<c_void>,
}

impl MKLocalSearch {
    /// Creates a wrapper for `MKLocalSearch`.
    pub fn new(request: &MKLocalSearchRequest) -> Result<Self, MapKitError> {
        let request_json = json_cstring(request, "MKLocalSearchRequest")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_local_search_new(request_json.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKLocalSearch")?;
        Ok(Self { raw })
    }

    /// Wraps `MKLocalSearch.fromPointsOfInterestRequest`.
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

    /// Wraps `MKLocalSearch.search`.
    pub fn search(request: &MKLocalSearchRequest) -> Result<MKLocalSearchResponse, MapKitError> {
        Self::new(request)?.start()
    }

    /// Wraps `MKLocalSearch.searchPointsOfInterest`.
    pub fn search_points_of_interest(
        request: &MKLocalPointsOfInterestRequest,
    ) -> Result<MKLocalSearchResponse, MapKitError> {
        Self::from_points_of_interest_request(request)?.start()
    }

    /// Wraps `MKLocalSearch.start`.
    pub fn start(&self) -> Result<MKLocalSearchResponse, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_local_search_start_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKLocalSearch start failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKLocalSearchResponse") }
        }
    }

    /// Wraps `MKLocalSearch.isSearching`.
    pub fn is_searching(&self) -> bool {
        unsafe { ffi::mk_local_search_is_searching(self.raw.as_ptr()) }
    }

    /// Wraps `MKLocalSearch.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_local_search_cancel(self.raw.as_ptr()) };
    }

    /// Transfer ownership of the underlying handle to the caller.
    /// The caller becomes responsible for releasing it.
    #[cfg(feature = "async")]
    pub(crate) fn into_raw(self) -> *mut c_void {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for MKLocalSearch {
    fn drop(&mut self) {
        unsafe { ffi::mk_local_search_release(self.raw.as_ptr()) };
    }
}
