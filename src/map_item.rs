use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::address::{MKAddress, MKAddressRepresentations};
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinate;
use crate::private::{cstring_from_str, owned_handle, parse_json_ptr, take_string};

/// Wraps `MKPlacemark`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKPlacemark {
    /// Wraps `MKPlacemark.coordinate`.
    pub coordinate: MKCoordinate,
    /// Wraps `MKPlacemark.countryCode`.
    pub country_code: Option<String>,
    /// Wraps `MKPlacemark.title`.
    pub title: Option<String>,
}

impl MKPlacemark {
    /// Creates a wrapper for `MKPlacemark`.
    pub const fn new(coordinate: MKCoordinate) -> Self {
        Self {
            coordinate,
            country_code: None,
            title: None,
        }
    }

    /// Wraps `MKPlacemark.countryCode`.
    pub fn with_country_code(mut self, country_code: impl Into<String>) -> Self {
        self.country_code = Some(country_code.into());
        self
    }

    /// Wraps `MKPlacemark.title`.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Wraps `MKMapItemIdentifier`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKMapItemIdentifier(pub String);

impl MKMapItemIdentifier {
    /// Creates a wrapper for `MKMapItemIdentifier`.
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }

    /// Wraps `MKMapItemIdentifier.rawValue`.
    pub fn raw_value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MKMapItemIdentifier {
    fn as_ref(&self) -> &str {
        self.raw_value()
    }
}

/// Wraps `MKMapItem`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapItem {
    /// Wraps `MKMapItem.identifier`.
    pub identifier: Option<String>,
    /// Wraps `MKMapItem.alternateIdentifiers`.
    #[serde(default)]
    pub alternate_identifiers: Vec<String>,
    /// Wraps `MKMapItem.name`.
    pub name: Option<String>,
    /// Wraps `MKMapItem.phoneNumber`.
    pub phone_number: Option<String>,
    /// Wraps `MKMapItem.url`.
    pub url: Option<String>,
    /// Wraps `MKMapItem.timeZoneIdentifier`.
    pub time_zone_identifier: Option<String>,
    /// Wraps `MKMapItem.pointOfInterestCategory`.
    pub point_of_interest_category: Option<String>,
    /// Wraps `MKMapItem.isCurrentLocation`.
    pub is_current_location: bool,
    /// Wraps `MKMapItem.placemark`.
    pub placemark: Option<MKPlacemark>,
    /// Wraps `MKMapItem.location`.
    pub location: Option<MKCoordinate>,
    /// Wraps `MKMapItem.address`.
    pub address: Option<MKAddress>,
    /// Wraps `MKMapItem.addressRepresentations`.
    pub address_representations: Option<MKAddressRepresentations>,
}

impl MKMapItem {
    /// Creates a wrapper for `MKMapItem`.
    pub fn new(placemark: MKPlacemark) -> Self {
        let coordinate = placemark.coordinate;
        Self {
            identifier: None,
            alternate_identifiers: Vec::new(),
            name: None,
            phone_number: None,
            url: None,
            time_zone_identifier: None,
            point_of_interest_category: None,
            is_current_location: false,
            placemark: Some(placemark),
            location: Some(coordinate),
            address: None,
            address_representations: None,
        }
    }

    /// Wraps `MKMapItem.fromLocation`.
    pub fn from_location(location: MKCoordinate, address: Option<MKAddress>) -> Self {
        Self {
            identifier: None,
            alternate_identifiers: Vec::new(),
            name: None,
            phone_number: None,
            url: None,
            time_zone_identifier: None,
            point_of_interest_category: None,
            is_current_location: false,
            placemark: None,
            location: Some(location),
            address,
            address_representations: None,
        }
    }

    /// Wraps `MKMapItem.currentLocation`.
    pub fn current_location() -> Self {
        Self {
            identifier: None,
            alternate_identifiers: Vec::new(),
            name: None,
            phone_number: None,
            url: None,
            time_zone_identifier: None,
            point_of_interest_category: None,
            is_current_location: true,
            placemark: None,
            location: None,
            address: None,
            address_representations: None,
        }
    }

    /// Wraps `MKMapItem.name`.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Wraps `MKMapItem.phoneNumber`.
    pub fn with_phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.phone_number = Some(phone_number.into());
        self
    }

    /// Wraps `MKMapItem.url`.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Wraps `MKMapItem.timeZoneIdentifier`.
    pub fn with_time_zone_identifier(mut self, time_zone_identifier: impl Into<String>) -> Self {
        self.time_zone_identifier = Some(time_zone_identifier.into());
        self
    }

    /// Wraps `MKMapItem.pointOfInterestCategory`.
    pub fn with_point_of_interest_category(
        mut self,
        point_of_interest_category: impl Into<String>,
    ) -> Self {
        self.point_of_interest_category = Some(point_of_interest_category.into());
        self
    }

    /// Wraps `MKMapItem.address`.
    pub fn with_address(mut self, address: MKAddress) -> Self {
        self.address = Some(address);
        self
    }

    /// Wraps `MKMapItem.coordinate`.
    pub fn coordinate(&self) -> Option<MKCoordinate> {
        self.location.or_else(|| {
            self.placemark
                .as_ref()
                .map(|placemark| placemark.coordinate)
        })
    }

    /// Wraps `MKMapItem.identifierValue`.
    pub fn identifier_value(&self) -> Option<MKMapItemIdentifier> {
        self.identifier.clone().map(MKMapItemIdentifier)
    }

    /// Wraps `MKMapItem.alternateIdentifierValues`.
    pub fn alternate_identifier_values(&self) -> Vec<MKMapItemIdentifier> {
        self.alternate_identifiers
            .iter()
            .cloned()
            .map(MKMapItemIdentifier)
            .collect()
    }

    /// Wraps `MKMapItem.launchOptionsCameraKey`.
    pub fn launch_options_camera_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsCameraKey,
            "MKLaunchOptionsCameraKey",
        )
    }

    /// Wraps `MKMapItem.launchOptionsDirectionsModeCycling`.
    pub fn launch_options_directions_mode_cycling() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeCycling,
            "MKLaunchOptionsDirectionsModeCycling",
        )
    }

    /// Wraps `MKMapItem.launchOptionsDirectionsModeDefault`.
    pub fn launch_options_directions_mode_default() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeDefault,
            "MKLaunchOptionsDirectionsModeDefault",
        )
    }

    /// Wraps `MKMapItem.launchOptionsDirectionsModeDriving`.
    pub fn launch_options_directions_mode_driving() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeDriving,
            "MKLaunchOptionsDirectionsModeDriving",
        )
    }

    /// Wraps `MKMapItem.launchOptionsDirectionsModeKey`.
    pub fn launch_options_directions_mode_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeKey,
            "MKLaunchOptionsDirectionsModeKey",
        )
    }

    /// Wraps `MKMapItem.launchOptionsDirectionsModeTransit`.
    pub fn launch_options_directions_mode_transit() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeTransit,
            "MKLaunchOptionsDirectionsModeTransit",
        )
    }

    /// Wraps `MKMapItem.launchOptionsDirectionsModeWalking`.
    pub fn launch_options_directions_mode_walking() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeWalking,
            "MKLaunchOptionsDirectionsModeWalking",
        )
    }

    /// Wraps `MKMapItem.launchOptionsMapCenterKey`.
    pub fn launch_options_map_center_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsMapCenterKey,
            "MKLaunchOptionsMapCenterKey",
        )
    }

    /// Wraps `MKMapItem.launchOptionsMapSpanKey`.
    pub fn launch_options_map_span_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsMapSpanKey,
            "MKLaunchOptionsMapSpanKey",
        )
    }

    /// Wraps `MKMapItem.launchOptionsMapTypeKey`.
    pub fn launch_options_map_type_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsMapTypeKey,
            "MKLaunchOptionsMapTypeKey",
        )
    }

    /// Wraps `MKMapItem.launchOptionsShowsTrafficKey`.
    pub fn launch_options_shows_traffic_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsShowsTrafficKey,
            "MKLaunchOptionsShowsTrafficKey",
        )
    }

    /// Wraps `MKMapItem.typeIdentifier`.
    pub fn type_identifier() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::MapItemTypeIdentifier,
            "MKMapItemTypeIdentifier",
        )
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMapItemRequestState {
    map_item_identifier: Option<String>,
    cancelled: bool,
    loading: bool,
}

/// Wraps `MKMapItemRequest`.
#[derive(Debug)]
pub struct MKMapItemRequest {
    raw: NonNull<c_void>,
}

impl MKMapItemRequest {
    /// Creates a wrapper for `MKMapItemRequest`.
    pub fn new(map_item_identifier: &MKMapItemIdentifier) -> Result<Self, MapKitError> {
        let identifier = cstring_from_str(map_item_identifier.raw_value(), "MKMapItemIdentifier")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_map_item_request_new(identifier.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKMapItemRequest")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMapItemRequestState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_map_item_request_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKMapItemRequest state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapItemRequest state") }
        }
    }

    /// Wraps `MKMapItemRequest.mapItemIdentifier`.
    pub fn map_item_identifier(&self) -> Result<Option<MKMapItemIdentifier>, MapKitError> {
        Ok(self.state()?.map_item_identifier.map(MKMapItemIdentifier))
    }

    /// Wraps `MKMapItemRequest.isCancelled`.
    pub fn is_cancelled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.cancelled)
    }

    /// Wraps `MKMapItemRequest.isLoading`.
    pub fn is_loading(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.loading)
    }

    /// Wraps `MKMapItemRequest.mapItem`.
    pub fn map_item(&self) -> Result<MKMapItem, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_map_item_request_get_map_item_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapItemRequest getMapItem failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapItem") }
        }
    }

    /// Wraps `MKMapItemRequest.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_map_item_request_cancel(self.raw.as_ptr()) };
    }
}

impl Drop for MKMapItemRequest {
    fn drop(&mut self) {
        unsafe { ffi::mk_map_item_request_release(self.raw.as_ptr()) };
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
enum MKMapItemStringConstantKind {
    LaunchOptionsCameraKey = 0,
    LaunchOptionsDirectionsModeCycling = 1,
    LaunchOptionsDirectionsModeDefault = 2,
    LaunchOptionsDirectionsModeDriving = 3,
    LaunchOptionsDirectionsModeKey = 4,
    LaunchOptionsDirectionsModeTransit = 5,
    LaunchOptionsDirectionsModeWalking = 6,
    LaunchOptionsMapCenterKey = 7,
    LaunchOptionsMapSpanKey = 8,
    LaunchOptionsMapTypeKey = 9,
    LaunchOptionsShowsTrafficKey = 10,
    MapItemTypeIdentifier = 11,
}

fn map_item_string_constant(
    kind: MKMapItemStringConstantKind,
    context: &str,
) -> Result<String, MapKitError> {
    let mut error = ptr::null_mut();
    let value = unsafe { ffi::mk_map_item_string_constant(kind as i32, &mut error) };
    if value.is_null() {
        Err(unsafe { MapKitError::from_error_ptr(error, context) })
    } else {
        unsafe {
            take_string(value).ok_or_else(|| {
                MapKitError::OperationFailed(format!("missing string payload for {context}"))
            })
        }
    }
}
