use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::address::{MKAddress, MKAddressRepresentations};
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinate;
use crate::private::{cstring_from_str, owned_handle, parse_json_ptr, take_string};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKPlacemark {
    pub coordinate: MKCoordinate,
    pub country_code: Option<String>,
    pub title: Option<String>,
}

impl MKPlacemark {
    pub const fn new(coordinate: MKCoordinate) -> Self {
        Self {
            coordinate,
            country_code: None,
            title: None,
        }
    }

    pub fn with_country_code(mut self, country_code: impl Into<String>) -> Self {
        self.country_code = Some(country_code.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKMapItemIdentifier(pub String);

impl MKMapItemIdentifier {
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }

    pub fn raw_value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MKMapItemIdentifier {
    fn as_ref(&self) -> &str {
        self.raw_value()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapItem {
    pub identifier: Option<String>,
    #[serde(default)]
    pub alternate_identifiers: Vec<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub url: Option<String>,
    pub time_zone_identifier: Option<String>,
    pub point_of_interest_category: Option<String>,
    pub is_current_location: bool,
    pub placemark: Option<MKPlacemark>,
    pub location: Option<MKCoordinate>,
    pub address: Option<MKAddress>,
    pub address_representations: Option<MKAddressRepresentations>,
}

impl MKMapItem {
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

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.phone_number = Some(phone_number.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn with_time_zone_identifier(
        mut self,
        time_zone_identifier: impl Into<String>,
    ) -> Self {
        self.time_zone_identifier = Some(time_zone_identifier.into());
        self
    }

    pub fn with_point_of_interest_category(
        mut self,
        point_of_interest_category: impl Into<String>,
    ) -> Self {
        self.point_of_interest_category = Some(point_of_interest_category.into());
        self
    }

    pub fn with_address(mut self, address: MKAddress) -> Self {
        self.address = Some(address);
        self
    }

    pub fn coordinate(&self) -> Option<MKCoordinate> {
        self.location
            .or_else(|| self.placemark.as_ref().map(|placemark| placemark.coordinate))
    }

    pub fn identifier_value(&self) -> Option<MKMapItemIdentifier> {
        self.identifier.clone().map(MKMapItemIdentifier)
    }

    pub fn alternate_identifier_values(&self) -> Vec<MKMapItemIdentifier> {
        self.alternate_identifiers
            .iter()
            .cloned()
            .map(MKMapItemIdentifier)
            .collect()
    }

    pub fn launch_options_camera_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsCameraKey,
            "MKLaunchOptionsCameraKey",
        )
    }

    pub fn launch_options_directions_mode_cycling() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeCycling,
            "MKLaunchOptionsDirectionsModeCycling",
        )
    }

    pub fn launch_options_directions_mode_default() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeDefault,
            "MKLaunchOptionsDirectionsModeDefault",
        )
    }

    pub fn launch_options_directions_mode_driving() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeDriving,
            "MKLaunchOptionsDirectionsModeDriving",
        )
    }

    pub fn launch_options_directions_mode_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeKey,
            "MKLaunchOptionsDirectionsModeKey",
        )
    }

    pub fn launch_options_directions_mode_transit() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeTransit,
            "MKLaunchOptionsDirectionsModeTransit",
        )
    }

    pub fn launch_options_directions_mode_walking() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsDirectionsModeWalking,
            "MKLaunchOptionsDirectionsModeWalking",
        )
    }

    pub fn launch_options_map_center_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsMapCenterKey,
            "MKLaunchOptionsMapCenterKey",
        )
    }

    pub fn launch_options_map_span_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsMapSpanKey,
            "MKLaunchOptionsMapSpanKey",
        )
    }

    pub fn launch_options_map_type_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsMapTypeKey,
            "MKLaunchOptionsMapTypeKey",
        )
    }

    pub fn launch_options_shows_traffic_key() -> Result<String, MapKitError> {
        map_item_string_constant(
            MKMapItemStringConstantKind::LaunchOptionsShowsTrafficKey,
            "MKLaunchOptionsShowsTrafficKey",
        )
    }

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

#[derive(Debug)]
pub struct MKMapItemRequest {
    raw: NonNull<c_void>,
}

impl MKMapItemRequest {
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

    pub fn map_item_identifier(&self) -> Result<Option<MKMapItemIdentifier>, MapKitError> {
        Ok(self.state()?.map_item_identifier.map(MKMapItemIdentifier))
    }

    pub fn is_cancelled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.cancelled)
    }

    pub fn is_loading(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.loading)
    }

    pub fn map_item(&self) -> Result<MKMapItem, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_map_item_request_get_map_item_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapItemRequest getMapItem failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapItem") }
        }
    }

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
