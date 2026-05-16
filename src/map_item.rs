use serde::{Deserialize, Serialize};

use crate::address::{MKAddress, MKAddressRepresentations};
use crate::geometry::MKCoordinate;

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
}
