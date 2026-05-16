use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKCoordinateRegion};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKPointOfInterestCategory(pub String);

impl MKPointOfInterestCategory {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn animal_service() -> Self {
        Self::new("animalService")
    }

    pub fn airport() -> Self {
        Self::new("airport")
    }

    pub fn amusement_park() -> Self {
        Self::new("amusementPark")
    }

    pub fn aquarium() -> Self {
        Self::new("aquarium")
    }

    pub fn atm() -> Self {
        Self::new("atm")
    }

    pub fn automotive_repair() -> Self {
        Self::new("automotiveRepair")
    }

    pub fn bakery() -> Self {
        Self::new("bakery")
    }

    pub fn bank() -> Self {
        Self::new("bank")
    }

    pub fn baseball() -> Self {
        Self::new("baseball")
    }

    pub fn basketball() -> Self {
        Self::new("basketball")
    }

    pub fn beach() -> Self {
        Self::new("beach")
    }

    pub fn beauty() -> Self {
        Self::new("beauty")
    }

    pub fn bowling() -> Self {
        Self::new("bowling")
    }

    pub fn brewery() -> Self {
        Self::new("brewery")
    }

    pub fn cafe() -> Self {
        Self::new("cafe")
    }

    pub fn campground() -> Self {
        Self::new("campground")
    }

    pub fn car_rental() -> Self {
        Self::new("carRental")
    }

    pub fn castle() -> Self {
        Self::new("castle")
    }

    pub fn convention_center() -> Self {
        Self::new("conventionCenter")
    }

    pub fn distillery() -> Self {
        Self::new("distillery")
    }

    pub fn ev_charger() -> Self {
        Self::new("evCharger")
    }

    pub fn fairground() -> Self {
        Self::new("fairground")
    }

    pub fn fire_station() -> Self {
        Self::new("fireStation")
    }

    pub fn fishing() -> Self {
        Self::new("fishing")
    }

    pub fn fitness_center() -> Self {
        Self::new("fitnessCenter")
    }

    pub fn food_market() -> Self {
        Self::new("foodMarket")
    }

    pub fn fortress() -> Self {
        Self::new("fortress")
    }

    pub fn gas_station() -> Self {
        Self::new("gasStation")
    }

    pub fn golf() -> Self {
        Self::new("golf")
    }

    pub fn go_kart() -> Self {
        Self::new("goKart")
    }

    pub fn hiking() -> Self {
        Self::new("hiking")
    }

    pub fn hospital() -> Self {
        Self::new("hospital")
    }

    pub fn hotel() -> Self {
        Self::new("hotel")
    }

    pub fn kayaking() -> Self {
        Self::new("kayaking")
    }

    pub fn landmark() -> Self {
        Self::new("landmark")
    }

    pub fn laundry() -> Self {
        Self::new("laundry")
    }

    pub fn library() -> Self {
        Self::new("library")
    }

    pub fn mailbox() -> Self {
        Self::new("mailbox")
    }

    pub fn marina() -> Self {
        Self::new("marina")
    }

    pub fn mini_golf() -> Self {
        Self::new("miniGolf")
    }

    pub fn movie_theater() -> Self {
        Self::new("movieTheater")
    }

    pub fn museum() -> Self {
        Self::new("museum")
    }

    pub fn music_venue() -> Self {
        Self::new("musicVenue")
    }

    pub fn national_monument() -> Self {
        Self::new("nationalMonument")
    }

    pub fn national_park() -> Self {
        Self::new("nationalPark")
    }

    pub fn nightlife() -> Self {
        Self::new("nightlife")
    }

    pub fn park() -> Self {
        Self::new("park")
    }

    pub fn parking() -> Self {
        Self::new("parking")
    }

    pub fn pharmacy() -> Self {
        Self::new("pharmacy")
    }

    pub fn planetarium() -> Self {
        Self::new("planetarium")
    }

    pub fn police() -> Self {
        Self::new("police")
    }

    pub fn post_office() -> Self {
        Self::new("postOffice")
    }

    pub fn public_transport() -> Self {
        Self::new("publicTransport")
    }

    pub fn restaurant() -> Self {
        Self::new("restaurant")
    }

    pub fn restroom() -> Self {
        Self::new("restroom")
    }

    pub fn rock_climbing() -> Self {
        Self::new("rockClimbing")
    }

    pub fn rv_park() -> Self {
        Self::new("rvPark")
    }

    pub fn school() -> Self {
        Self::new("school")
    }

    pub fn skate_park() -> Self {
        Self::new("skatePark")
    }

    pub fn skating() -> Self {
        Self::new("skating")
    }

    pub fn skiing() -> Self {
        Self::new("skiing")
    }

    pub fn soccer() -> Self {
        Self::new("soccer")
    }

    pub fn spa() -> Self {
        Self::new("spa")
    }

    pub fn stadium() -> Self {
        Self::new("stadium")
    }

    pub fn store() -> Self {
        Self::new("store")
    }

    pub fn surfing() -> Self {
        Self::new("surfing")
    }

    pub fn swimming() -> Self {
        Self::new("swimming")
    }

    pub fn tennis() -> Self {
        Self::new("tennis")
    }

    pub fn theater() -> Self {
        Self::new("theater")
    }

    pub fn university() -> Self {
        Self::new("university")
    }

    pub fn winery() -> Self {
        Self::new("winery")
    }

    pub fn volleyball() -> Self {
        Self::new("volleyball")
    }

    pub fn zoo() -> Self {
        Self::new("zoo")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKPointOfInterestFilterMode {
    Including,
    Excluding,
    IncludingAll,
    ExcludingAll,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKPointOfInterestFilter {
    pub mode: MKPointOfInterestFilterMode,
    pub categories: Vec<MKPointOfInterestCategory>,
}

impl MKPointOfInterestFilter {
    pub fn including_all() -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::IncludingAll,
            categories: Vec::new(),
        }
    }

    pub fn excluding_all() -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::ExcludingAll,
            categories: Vec::new(),
        }
    }

    pub fn including(categories: Vec<MKPointOfInterestCategory>) -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::Including,
            categories,
        }
    }

    pub fn excluding(categories: Vec<MKPointOfInterestCategory>) -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::Excluding,
            categories,
        }
    }

    pub fn includes_category(&self, category: &MKPointOfInterestCategory) -> bool {
        match self.mode {
            MKPointOfInterestFilterMode::Including => self.categories.contains(category),
            MKPointOfInterestFilterMode::IncludingAll => true,
            MKPointOfInterestFilterMode::Excluding | MKPointOfInterestFilterMode::ExcludingAll => {
                false
            }
        }
    }

    pub fn excludes_category(&self, category: &MKPointOfInterestCategory) -> bool {
        match self.mode {
            MKPointOfInterestFilterMode::Excluding => self.categories.contains(category),
            MKPointOfInterestFilterMode::ExcludingAll => true,
            MKPointOfInterestFilterMode::Including | MKPointOfInterestFilterMode::IncludingAll => {
                false
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalPointsOfInterestRequest {
    pub coordinate: Option<MKCoordinate>,
    pub radius: Option<f64>,
    pub region: Option<MKCoordinateRegion>,
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
}

impl MKLocalPointsOfInterestRequest {
    pub fn max_radius() -> f64 {
        unsafe { ffi::mk_points_of_interest_request_max_radius() }
    }

    pub fn with_radius(center: MKCoordinate, radius: f64) -> Self {
        Self {
            coordinate: Some(center),
            radius: Some(radius),
            region: None,
            point_of_interest_filter: None,
        }
    }

    pub fn with_region(region: MKCoordinateRegion) -> Self {
        Self {
            coordinate: None,
            radius: None,
            region: Some(region),
            point_of_interest_filter: None,
        }
    }

    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    pub fn validate(&self) -> Result<(), MapKitError> {
        if let Some(radius) = self.radius {
            let max_radius = Self::max_radius();
            if radius > max_radius {
                return Err(MapKitError::InvalidArgument(format!(
                    "radius {radius} exceeds MKPointsOfInterestRequestMaxRadius={max_radius}"
                )));
            }
        }
        Ok(())
    }
}
