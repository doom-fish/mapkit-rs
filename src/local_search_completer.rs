use core::ffi::c_void;
use std::ops::{BitOr, BitOrAssign};
use std::ptr::{self, NonNull};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::address::MKAddressFilter;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinateRegion;
use crate::local_search::MKLocalSearchRegionPriority;
use crate::point_of_interest::MKPointOfInterestFilter;
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result};

/// Wraps `MKLocalSearchCompleterResultType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKLocalSearchCompleterResultType(pub u64);

impl MKLocalSearchCompleterResultType {
    /// Wraps `MKLocalSearchCompleterResultType.address`.
    pub const ADDRESS: Self = Self(1 << 0);
    /// Wraps `MKLocalSearchCompleterResultType.pointOfInterest`.
    pub const POINT_OF_INTEREST: Self = Self(1 << 1);
    /// Wraps `MKLocalSearchCompleterResultType.query`.
    pub const QUERY: Self = Self(1 << 2);
    /// Wraps `MKLocalSearchCompleterResultType.physicalFeature`.
    pub const PHYSICAL_FEATURE: Self = Self(1 << 3);
    /// Wraps `MKLocalSearchCompleterResultType.all`.
    pub const ALL: Self = Self(
        Self::ADDRESS.0 | Self::POINT_OF_INTEREST.0 | Self::QUERY.0 | Self::PHYSICAL_FEATURE.0,
    );

    /// Wraps `MKLocalSearchCompleterResultType.bits`.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Wraps `MKLocalSearchCompleterResultType.contains`.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Default for MKLocalSearchCompleterResultType {
    fn default() -> Self {
        Self(Self::ADDRESS.0 | Self::POINT_OF_INTEREST.0 | Self::QUERY.0)
    }
}

impl BitOr for MKLocalSearchCompleterResultType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for MKLocalSearchCompleterResultType {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Wraps `MKTextHighlightRange`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKTextHighlightRange {
    /// Wraps `MKTextHighlightRange.location`.
    pub location: usize,
    /// Wraps `MKTextHighlightRange.length`.
    pub length: usize,
}

/// Wraps `MKLocalSearchCompletion`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchCompletion {
    /// Wraps `MKLocalSearchCompletion.title`.
    pub title: String,
    /// Wraps `MKLocalSearchCompletion.titleHighlightRanges`.
    pub title_highlight_ranges: Vec<MKTextHighlightRange>,
    /// Wraps `MKLocalSearchCompletion.subtitle`.
    pub subtitle: String,
    /// Wraps `MKLocalSearchCompletion.subtitleHighlightRanges`.
    pub subtitle_highlight_ranges: Vec<MKTextHighlightRange>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKLocalSearchCompleterState {
    query_fragment: String,
    region: MKCoordinateRegion,
    region_priority: MKLocalSearchRegionPriority,
    result_types: MKLocalSearchCompleterResultType,
    point_of_interest_filter: Option<MKPointOfInterestFilter>,
    address_filter: Option<MKAddressFilter>,
    results: Vec<MKLocalSearchCompletion>,
    searching: bool,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKLocalSearchCompleterOptions {
    query_fragment_present: bool,
    query_fragment: Option<String>,
    region: Option<MKCoordinateRegion>,
    region_priority: Option<MKLocalSearchRegionPriority>,
    result_types: Option<MKLocalSearchCompleterResultType>,
    point_of_interest_filter_present: bool,
    point_of_interest_filter: Option<MKPointOfInterestFilter>,
    address_filter_present: bool,
    address_filter: Option<MKAddressFilter>,
}

/// Wraps `MKLocalSearchCompleterDelegate`.
pub trait MKLocalSearchCompleterDelegate {
    fn completer_did_update_results(
        &mut self,
        _completer: &MKLocalSearchCompleter,
        _results: &[MKLocalSearchCompletion],
    ) {
    }

    fn completer_did_fail_with_error(
        &mut self,
        _completer: &MKLocalSearchCompleter,
        _error: &MapKitError,
    ) {
    }
}

/// Wraps `MKLocalSearchCompleter`.
#[derive(Debug)]
pub struct MKLocalSearchCompleter {
    raw: NonNull<c_void>,
}

impl MKLocalSearchCompleter {
    /// Creates a wrapper for `MKLocalSearchCompleter`.
    pub fn new() -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_local_search_completer_new(&mut error) };
        let raw = owned_handle(raw, error, "failed to create MKLocalSearchCompleter")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKLocalSearchCompleterState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_local_search_completer_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKLocalSearchCompleter state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKLocalSearchCompleter state") }
        }
    }

    fn apply_options(&self, options: &MKLocalSearchCompleterOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKLocalSearchCompleter options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_local_search_completer_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKLocalSearchCompleter") }
    }

    /// Wraps `MKLocalSearchCompleter.queryFragment`.
    pub fn query_fragment(&self) -> Result<String, MapKitError> {
        Ok(self.state()?.query_fragment)
    }

    /// Wraps `MKLocalSearchCompleter.queryFragment`.
    pub fn set_query_fragment(&self, query_fragment: impl Into<String>) -> Result<(), MapKitError> {
        self.apply_options(&MKLocalSearchCompleterOptions {
            query_fragment_present: true,
            query_fragment: Some(query_fragment.into()),
            ..MKLocalSearchCompleterOptions::default()
        })
    }

    /// Wraps `MKLocalSearchCompleter.region`.
    pub fn region(&self) -> Result<MKCoordinateRegion, MapKitError> {
        Ok(self.state()?.region)
    }

    /// Wraps `MKLocalSearchCompleter.region`.
    pub fn set_region(&self, region: MKCoordinateRegion) -> Result<(), MapKitError> {
        self.apply_options(&MKLocalSearchCompleterOptions {
            region: Some(region),
            ..MKLocalSearchCompleterOptions::default()
        })
    }

    /// Wraps `MKLocalSearchCompleter.regionPriority`.
    pub fn region_priority(&self) -> Result<MKLocalSearchRegionPriority, MapKitError> {
        Ok(self.state()?.region_priority)
    }

    /// Wraps `MKLocalSearchCompleter.regionPriority`.
    pub fn set_region_priority(
        &self,
        region_priority: MKLocalSearchRegionPriority,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKLocalSearchCompleterOptions {
            region_priority: Some(region_priority),
            ..MKLocalSearchCompleterOptions::default()
        })
    }

    /// Wraps `MKLocalSearchCompleter.resultTypes`.
    pub fn result_types(&self) -> Result<MKLocalSearchCompleterResultType, MapKitError> {
        Ok(self.state()?.result_types)
    }

    /// Wraps `MKLocalSearchCompleter.resultTypes`.
    pub fn set_result_types(
        &self,
        result_types: MKLocalSearchCompleterResultType,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKLocalSearchCompleterOptions {
            result_types: Some(result_types),
            ..MKLocalSearchCompleterOptions::default()
        })
    }

    /// Wraps `MKLocalSearchCompleter.pointOfInterestFilter`.
    pub fn point_of_interest_filter(&self) -> Result<Option<MKPointOfInterestFilter>, MapKitError> {
        Ok(self.state()?.point_of_interest_filter)
    }

    /// Wraps `MKLocalSearchCompleter.pointOfInterestFilter`.
    pub fn set_point_of_interest_filter(
        &self,
        point_of_interest_filter: Option<MKPointOfInterestFilter>,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKLocalSearchCompleterOptions {
            point_of_interest_filter_present: true,
            point_of_interest_filter,
            ..MKLocalSearchCompleterOptions::default()
        })
    }

    /// Wraps `MKLocalSearchCompleter.addressFilter`.
    pub fn address_filter(&self) -> Result<Option<MKAddressFilter>, MapKitError> {
        Ok(self.state()?.address_filter)
    }

    /// Wraps `MKLocalSearchCompleter.addressFilter`.
    pub fn set_address_filter(
        &self,
        address_filter: Option<MKAddressFilter>,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKLocalSearchCompleterOptions {
            address_filter_present: true,
            address_filter,
            ..MKLocalSearchCompleterOptions::default()
        })
    }

    /// Wraps `MKLocalSearchCompleter.results`.
    pub fn results(&self) -> Result<Vec<MKLocalSearchCompletion>, MapKitError> {
        Ok(self.state()?.results)
    }

    /// Wraps `MKLocalSearchCompleter.isSearching`.
    pub fn is_searching(&self) -> bool {
        self.state().is_ok_and(|state| state.searching)
    }

    /// Wraps `MKLocalSearchCompleter.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_local_search_completer_cancel(self.raw.as_ptr()) };
    }

    /// Wraps `MKLocalSearchCompleter.refresh`.
    pub fn refresh(&self) -> Result<Vec<MKLocalSearchCompletion>, MapKitError> {
        self.refresh_with_timeout(Duration::from_secs(30))
    }

    /// Wraps `MKLocalSearchCompleter.refreshWithTimeout`.
    pub fn refresh_with_timeout(
        &self,
        timeout: Duration,
    ) -> Result<Vec<MKLocalSearchCompletion>, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_local_search_completer_refresh_json(
                self.raw.as_ptr(),
                timeout.as_millis().try_into().unwrap_or(u64::MAX),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKLocalSearchCompleter refresh failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "[MKLocalSearchCompletion]") }
        }
    }

    /// Wraps `MKLocalSearchCompleter.refreshWithDelegate`.
    pub fn refresh_with_delegate<D: MKLocalSearchCompleterDelegate>(
        &self,
        delegate: &mut D,
        timeout: Duration,
    ) -> Result<Vec<MKLocalSearchCompletion>, MapKitError> {
        match self.refresh_with_timeout(timeout) {
            Ok(results) => {
                delegate.completer_did_update_results(self, &results);
                Ok(results)
            }
            Err(error) => {
                delegate.completer_did_fail_with_error(self, &error);
                Err(error)
            }
        }
    }
}

impl Drop for MKLocalSearchCompleter {
    fn drop(&mut self) {
        unsafe { ffi::mk_local_search_completer_release(self.raw.as_ptr()) };
    }
}
