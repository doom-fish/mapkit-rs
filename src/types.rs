use std::ops::{BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl MKCoordinate {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKCoordinateSpan {
    pub latitude_delta: f64,
    pub longitude_delta: f64,
}

impl MKCoordinateSpan {
    pub const fn new(latitude_delta: f64, longitude_delta: f64) -> Self {
        Self {
            latitude_delta,
            longitude_delta,
        }
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKCoordinateRegion {
    pub center: MKCoordinate,
    pub span: MKCoordinateSpan,
}

impl MKCoordinateRegion {
    pub const fn new(center: MKCoordinate, span: MKCoordinateSpan) -> Self {
        Self { center, span }
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKMapPoint {
    pub x: f64,
    pub y: f64,
}

impl MKMapPoint {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKLocalSearchResultType(pub u64);

impl MKLocalSearchResultType {
    pub const ADDRESS: Self = Self(1 << 0);
    pub const POINT_OF_INTEREST: Self = Self(1 << 1);
    pub const PHYSICAL_FEATURE: Self = Self(1 << 2);
    pub const ALL: Self = Self(
        Self::ADDRESS.0 | Self::POINT_OF_INTEREST.0 | Self::PHYSICAL_FEATURE.0,
    );

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKDistanceFormatterUnits {
    #[default]
    Default,
    Metric,
    Imperial,
    ImperialWithYards,
}

impl MKDistanceFormatterUnits {
    pub(crate) const fn as_raw(self) -> u64 {
        match self {
            Self::Default => 0,
            Self::Metric => 1,
            Self::Imperial => 2,
            Self::ImperialWithYards => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKDistanceFormatterUnitStyle {
    #[default]
    Default,
    Abbreviated,
    Full,
}

impl MKDistanceFormatterUnitStyle {
    pub(crate) const fn as_raw(self) -> u64 {
        match self {
            Self::Default => 0,
            Self::Abbreviated => 1,
            Self::Full => 2,
        }
    }
}

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

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapItem {
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub url: Option<String>,
    pub time_zone_identifier: Option<String>,
    pub point_of_interest_category: Option<String>,
    pub is_current_location: bool,
    pub placemark: Option<MKPlacemark>,
}

impl MKMapItem {
    pub fn new(placemark: MKPlacemark) -> Self {
        Self {
            identifier: None,
            name: None,
            phone_number: None,
            url: None,
            time_zone_identifier: None,
            point_of_interest_category: None,
            is_current_location: false,
            placemark: Some(placemark),
        }
    }

    pub fn current_location() -> Self {
        Self {
            identifier: None,
            name: None,
            phone_number: None,
            url: None,
            time_zone_identifier: None,
            point_of_interest_category: None,
            is_current_location: true,
            placemark: None,
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

    pub fn coordinate(&self) -> Option<MKCoordinate> {
        self.placemark.as_ref().map(|placemark| placemark.coordinate)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchRequest {
    pub natural_language_query: String,
    pub region: Option<MKCoordinateRegion>,
    #[serde(default)]
    pub result_types: MKLocalSearchResultType,
}

impl MKLocalSearchRequest {
    pub fn new(natural_language_query: impl Into<String>) -> Self {
        Self {
            natural_language_query: natural_language_query.into(),
            region: None,
            result_types: MKLocalSearchResultType::default(),
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
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalSearchResponse {
    pub map_items: Vec<MKMapItem>,
    pub bounding_region: MKCoordinateRegion,
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
