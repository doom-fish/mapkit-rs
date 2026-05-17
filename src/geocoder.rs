use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::Deserialize;

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKCoordinateRegion};
use crate::map_item::MKMapItem;
use crate::private::{cstring_from_str, json_cstring, owned_handle, parse_json_ptr};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKGeocodingRequestState {
    address_string: String,
    region: MKCoordinateRegion,
    preferred_locale_identifier: Option<String>,
    cancelled: bool,
    loading: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKReverseGeocodingRequestState {
    location: MKCoordinate,
    preferred_locale_identifier: Option<String>,
    cancelled: bool,
    loading: bool,
}

#[derive(Debug)]
pub struct MKGeocodingRequest {
    raw: NonNull<c_void>,
}

impl MKGeocodingRequest {
    pub fn new(address_string: &str) -> Result<Self, MapKitError> {
        let address_string = cstring_from_str(address_string, "MKGeocodingRequest address string")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_geocoding_request_new(address_string.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKGeocodingRequest")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKGeocodingRequestState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_geocoding_request_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKGeocodingRequest state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKGeocodingRequest state") }
        }
    }

    pub fn address_string(&self) -> Result<String, MapKitError> {
        Ok(self.state()?.address_string)
    }

    pub fn region(&self) -> Result<MKCoordinateRegion, MapKitError> {
        Ok(self.state()?.region)
    }

    pub fn set_region(&self, region: MKCoordinateRegion) -> Result<(), MapKitError> {
        let region = json_cstring(&region, "MKCoordinateRegion")?;
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_geocoding_request_set_region_json(self.raw.as_ptr(), region.as_ptr(), &mut error) };
        unsafe { crate::private::unit_result(error, "failed to set MKGeocodingRequest region") }
    }

    pub fn preferred_locale_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.preferred_locale_identifier)
    }

    pub fn set_preferred_locale_identifier(
        &self,
        preferred_locale_identifier: Option<&str>,
    ) -> Result<(), MapKitError> {
        let locale = preferred_locale_identifier.map(|value| {
            cstring_from_str(value, "MKGeocodingRequest preferred locale")
        }).transpose()?;
        let locale_ptr = locale.as_ref().map_or(std::ptr::null(), |value| value.as_ptr());
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_geocoding_request_set_preferred_locale(
                self.raw.as_ptr(),
                locale_ptr,
                &mut error,
            );
        };
        unsafe {
            crate::private::unit_result(error, "failed to set MKGeocodingRequest locale")
        }
    }

    pub fn is_cancelled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.cancelled)
    }

    pub fn is_loading(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.loading)
    }

    pub fn map_items(&self) -> Result<Vec<MKMapItem>, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_geocoding_request_get_map_items_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKGeocodingRequest getMapItems failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapItem array") }
        }
    }

    pub fn cancel(&self) {
        unsafe { ffi::mk_geocoding_request_cancel(self.raw.as_ptr()) };
    }

    pub(crate) fn into_raw(self) -> *mut c_void {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for MKGeocodingRequest {
    fn drop(&mut self) {
        unsafe { ffi::mk_geocoding_request_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKReverseGeocodingRequest {
    raw: NonNull<c_void>,
}

impl MKReverseGeocodingRequest {
    pub fn new(location: MKCoordinate) -> Result<Self, MapKitError> {
        let location = json_cstring(&location, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_reverse_geocoding_request_new_json(location.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKReverseGeocodingRequest")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKReverseGeocodingRequestState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_reverse_geocoding_request_state_json(self.raw.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKReverseGeocodingRequest state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKReverseGeocodingRequest state") }
        }
    }

    pub fn location(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.location)
    }

    pub fn preferred_locale_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.preferred_locale_identifier)
    }

    pub fn set_preferred_locale_identifier(
        &self,
        preferred_locale_identifier: Option<&str>,
    ) -> Result<(), MapKitError> {
        let locale = preferred_locale_identifier.map(|value| {
            cstring_from_str(value, "MKReverseGeocodingRequest preferred locale")
        }).transpose()?;
        let locale_ptr = locale.as_ref().map_or(std::ptr::null(), |value| value.as_ptr());
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_reverse_geocoding_request_set_preferred_locale(
                self.raw.as_ptr(),
                locale_ptr,
                &mut error,
            );
        };
        unsafe {
            crate::private::unit_result(
                error,
                "failed to set MKReverseGeocodingRequest locale",
            )
        }
    }

    pub fn is_cancelled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.cancelled)
    }

    pub fn is_loading(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.loading)
    }

    pub fn map_items(&self) -> Result<Vec<MKMapItem>, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_reverse_geocoding_request_get_map_items_json(self.raw.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(
                    error,
                    "MKReverseGeocodingRequest getMapItems failed",
                )
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapItem array") }
        }
    }

    pub fn cancel(&self) {
        unsafe { ffi::mk_reverse_geocoding_request_cancel(self.raw.as_ptr()) };
    }

    pub(crate) fn into_raw(self) -> *mut c_void {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for MKReverseGeocodingRequest {
    fn drop(&mut self) {
        unsafe { ffi::mk_reverse_geocoding_request_release(self.raw.as_ptr()) };
    }
}
