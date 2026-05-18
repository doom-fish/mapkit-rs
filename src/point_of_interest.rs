use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKCoordinateRegion};

/// Wraps `MKPointOfInterestCategory`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKPointOfInterestCategory(pub String);

impl MKPointOfInterestCategory {
    /// Creates a wrapper for `MKPointOfInterestCategory`.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Wraps `MKPointOfInterestCategory.animalService`.
    pub fn animal_service() -> Self {
        Self::new("animalService")
    }

    /// Wraps `MKPointOfInterestCategory.airport`.
    pub fn airport() -> Self {
        Self::new("airport")
    }

    /// Wraps `MKPointOfInterestCategory.amusementPark`.
    pub fn amusement_park() -> Self {
        Self::new("amusementPark")
    }

    /// Wraps `MKPointOfInterestCategory.aquarium`.
    pub fn aquarium() -> Self {
        Self::new("aquarium")
    }

    /// Wraps `MKPointOfInterestCategory.atm`.
    pub fn atm() -> Self {
        Self::new("atm")
    }

    /// Wraps `MKPointOfInterestCategory.automotiveRepair`.
    pub fn automotive_repair() -> Self {
        Self::new("automotiveRepair")
    }

    /// Wraps `MKPointOfInterestCategory.bakery`.
    pub fn bakery() -> Self {
        Self::new("bakery")
    }

    /// Wraps `MKPointOfInterestCategory.bank`.
    pub fn bank() -> Self {
        Self::new("bank")
    }

    /// Wraps `MKPointOfInterestCategory.baseball`.
    pub fn baseball() -> Self {
        Self::new("baseball")
    }

    /// Wraps `MKPointOfInterestCategory.basketball`.
    pub fn basketball() -> Self {
        Self::new("basketball")
    }

    /// Wraps `MKPointOfInterestCategory.beach`.
    pub fn beach() -> Self {
        Self::new("beach")
    }

    /// Wraps `MKPointOfInterestCategory.beauty`.
    pub fn beauty() -> Self {
        Self::new("beauty")
    }

    /// Wraps `MKPointOfInterestCategory.bowling`.
    pub fn bowling() -> Self {
        Self::new("bowling")
    }

    /// Wraps `MKPointOfInterestCategory.brewery`.
    pub fn brewery() -> Self {
        Self::new("brewery")
    }

    /// Wraps `MKPointOfInterestCategory.cafe`.
    pub fn cafe() -> Self {
        Self::new("cafe")
    }

    /// Wraps `MKPointOfInterestCategory.campground`.
    pub fn campground() -> Self {
        Self::new("campground")
    }

    /// Wraps `MKPointOfInterestCategory.carRental`.
    pub fn car_rental() -> Self {
        Self::new("carRental")
    }

    /// Wraps `MKPointOfInterestCategory.castle`.
    pub fn castle() -> Self {
        Self::new("castle")
    }

    /// Wraps `MKPointOfInterestCategory.conventionCenter`.
    pub fn convention_center() -> Self {
        Self::new("conventionCenter")
    }

    /// Wraps `MKPointOfInterestCategory.distillery`.
    pub fn distillery() -> Self {
        Self::new("distillery")
    }

    /// Wraps `MKPointOfInterestCategory.evCharger`.
    pub fn ev_charger() -> Self {
        Self::new("evCharger")
    }

    /// Wraps `MKPointOfInterestCategory.fairground`.
    pub fn fairground() -> Self {
        Self::new("fairground")
    }

    /// Wraps `MKPointOfInterestCategory.fireStation`.
    pub fn fire_station() -> Self {
        Self::new("fireStation")
    }

    /// Wraps `MKPointOfInterestCategory.fishing`.
    pub fn fishing() -> Self {
        Self::new("fishing")
    }

    /// Wraps `MKPointOfInterestCategory.fitnessCenter`.
    pub fn fitness_center() -> Self {
        Self::new("fitnessCenter")
    }

    /// Wraps `MKPointOfInterestCategory.foodMarket`.
    pub fn food_market() -> Self {
        Self::new("foodMarket")
    }

    /// Wraps `MKPointOfInterestCategory.fortress`.
    pub fn fortress() -> Self {
        Self::new("fortress")
    }

    /// Wraps `MKPointOfInterestCategory.gasStation`.
    pub fn gas_station() -> Self {
        Self::new("gasStation")
    }

    /// Wraps `MKPointOfInterestCategory.golf`.
    pub fn golf() -> Self {
        Self::new("golf")
    }

    /// Wraps `MKPointOfInterestCategory.goKart`.
    pub fn go_kart() -> Self {
        Self::new("goKart")
    }

    /// Wraps `MKPointOfInterestCategory.hiking`.
    pub fn hiking() -> Self {
        Self::new("hiking")
    }

    /// Wraps `MKPointOfInterestCategory.hospital`.
    pub fn hospital() -> Self {
        Self::new("hospital")
    }

    /// Wraps `MKPointOfInterestCategory.hotel`.
    pub fn hotel() -> Self {
        Self::new("hotel")
    }

    /// Wraps `MKPointOfInterestCategory.kayaking`.
    pub fn kayaking() -> Self {
        Self::new("kayaking")
    }

    /// Wraps `MKPointOfInterestCategory.landmark`.
    pub fn landmark() -> Self {
        Self::new("landmark")
    }

    /// Wraps `MKPointOfInterestCategory.laundry`.
    pub fn laundry() -> Self {
        Self::new("laundry")
    }

    /// Wraps `MKPointOfInterestCategory.library`.
    pub fn library() -> Self {
        Self::new("library")
    }

    /// Wraps `MKPointOfInterestCategory.mailbox`.
    pub fn mailbox() -> Self {
        Self::new("mailbox")
    }

    /// Wraps `MKPointOfInterestCategory.marina`.
    pub fn marina() -> Self {
        Self::new("marina")
    }

    /// Wraps `MKPointOfInterestCategory.miniGolf`.
    pub fn mini_golf() -> Self {
        Self::new("miniGolf")
    }

    /// Wraps `MKPointOfInterestCategory.movieTheater`.
    pub fn movie_theater() -> Self {
        Self::new("movieTheater")
    }

    /// Wraps `MKPointOfInterestCategory.museum`.
    pub fn museum() -> Self {
        Self::new("museum")
    }

    /// Wraps `MKPointOfInterestCategory.musicVenue`.
    pub fn music_venue() -> Self {
        Self::new("musicVenue")
    }

    /// Wraps `MKPointOfInterestCategory.nationalMonument`.
    pub fn national_monument() -> Self {
        Self::new("nationalMonument")
    }

    /// Wraps `MKPointOfInterestCategory.nationalPark`.
    pub fn national_park() -> Self {
        Self::new("nationalPark")
    }

    /// Wraps `MKPointOfInterestCategory.nightlife`.
    pub fn nightlife() -> Self {
        Self::new("nightlife")
    }

    /// Wraps `MKPointOfInterestCategory.park`.
    pub fn park() -> Self {
        Self::new("park")
    }

    /// Wraps `MKPointOfInterestCategory.parking`.
    pub fn parking() -> Self {
        Self::new("parking")
    }

    /// Wraps `MKPointOfInterestCategory.pharmacy`.
    pub fn pharmacy() -> Self {
        Self::new("pharmacy")
    }

    /// Wraps `MKPointOfInterestCategory.planetarium`.
    pub fn planetarium() -> Self {
        Self::new("planetarium")
    }

    /// Wraps `MKPointOfInterestCategory.police`.
    pub fn police() -> Self {
        Self::new("police")
    }

    /// Wraps `MKPointOfInterestCategory.postOffice`.
    pub fn post_office() -> Self {
        Self::new("postOffice")
    }

    /// Wraps `MKPointOfInterestCategory.publicTransport`.
    pub fn public_transport() -> Self {
        Self::new("publicTransport")
    }

    /// Wraps `MKPointOfInterestCategory.restaurant`.
    pub fn restaurant() -> Self {
        Self::new("restaurant")
    }

    /// Wraps `MKPointOfInterestCategory.restroom`.
    pub fn restroom() -> Self {
        Self::new("restroom")
    }

    /// Wraps `MKPointOfInterestCategory.rockClimbing`.
    pub fn rock_climbing() -> Self {
        Self::new("rockClimbing")
    }

    /// Wraps `MKPointOfInterestCategory.rvPark`.
    pub fn rv_park() -> Self {
        Self::new("rvPark")
    }

    /// Wraps `MKPointOfInterestCategory.school`.
    pub fn school() -> Self {
        Self::new("school")
    }

    /// Wraps `MKPointOfInterestCategory.skatePark`.
    pub fn skate_park() -> Self {
        Self::new("skatePark")
    }

    /// Wraps `MKPointOfInterestCategory.skating`.
    pub fn skating() -> Self {
        Self::new("skating")
    }

    /// Wraps `MKPointOfInterestCategory.skiing`.
    pub fn skiing() -> Self {
        Self::new("skiing")
    }

    /// Wraps `MKPointOfInterestCategory.soccer`.
    pub fn soccer() -> Self {
        Self::new("soccer")
    }

    /// Wraps `MKPointOfInterestCategory.spa`.
    pub fn spa() -> Self {
        Self::new("spa")
    }

    /// Wraps `MKPointOfInterestCategory.stadium`.
    pub fn stadium() -> Self {
        Self::new("stadium")
    }

    /// Wraps `MKPointOfInterestCategory.store`.
    pub fn store() -> Self {
        Self::new("store")
    }

    /// Wraps `MKPointOfInterestCategory.surfing`.
    pub fn surfing() -> Self {
        Self::new("surfing")
    }

    /// Wraps `MKPointOfInterestCategory.swimming`.
    pub fn swimming() -> Self {
        Self::new("swimming")
    }

    /// Wraps `MKPointOfInterestCategory.tennis`.
    pub fn tennis() -> Self {
        Self::new("tennis")
    }

    /// Wraps `MKPointOfInterestCategory.theater`.
    pub fn theater() -> Self {
        Self::new("theater")
    }

    /// Wraps `MKPointOfInterestCategory.university`.
    pub fn university() -> Self {
        Self::new("university")
    }

    /// Wraps `MKPointOfInterestCategory.winery`.
    pub fn winery() -> Self {
        Self::new("winery")
    }

    /// Wraps `MKPointOfInterestCategory.volleyball`.
    pub fn volleyball() -> Self {
        Self::new("volleyball")
    }

    /// Wraps `MKPointOfInterestCategory.zoo`.
    pub fn zoo() -> Self {
        Self::new("zoo")
    }
}

/// Wraps `MKPointOfInterestFilterMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKPointOfInterestFilterMode {
    Including,
    Excluding,
    IncludingAll,
    ExcludingAll,
}

/// Wraps `MKPointOfInterestFilter`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKPointOfInterestFilter {
    /// Wraps `MKPointOfInterestFilter.mode`.
    pub mode: MKPointOfInterestFilterMode,
    /// Wraps `MKPointOfInterestFilter.categories`.
    pub categories: Vec<MKPointOfInterestCategory>,
}

impl MKPointOfInterestFilter {
    /// Wraps `MKPointOfInterestFilter.includingAll`.
    pub fn including_all() -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::IncludingAll,
            categories: Vec::new(),
        }
    }

    /// Wraps `MKPointOfInterestFilter.excludingAll`.
    pub fn excluding_all() -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::ExcludingAll,
            categories: Vec::new(),
        }
    }

    /// Wraps `MKPointOfInterestFilter.including`.
    pub fn including(categories: Vec<MKPointOfInterestCategory>) -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::Including,
            categories,
        }
    }

    /// Wraps `MKPointOfInterestFilter.excluding`.
    pub fn excluding(categories: Vec<MKPointOfInterestCategory>) -> Self {
        Self {
            mode: MKPointOfInterestFilterMode::Excluding,
            categories,
        }
    }

    /// Wraps `MKPointOfInterestFilter.includesCategory`.
    pub fn includes_category(&self, category: &MKPointOfInterestCategory) -> bool {
        match self.mode {
            MKPointOfInterestFilterMode::Including => self.categories.contains(category),
            MKPointOfInterestFilterMode::IncludingAll => true,
            MKPointOfInterestFilterMode::Excluding | MKPointOfInterestFilterMode::ExcludingAll => {
                false
            }
        }
    }

    /// Wraps `MKPointOfInterestFilter.excludesCategory`.
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

/// Wraps `MKLocalPointsOfInterestRequest`.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalPointsOfInterestRequest {
    /// Wraps `MKLocalPointsOfInterestRequest.coordinate`.
    pub coordinate: Option<MKCoordinate>,
    /// Wraps `MKLocalPointsOfInterestRequest.radius`.
    pub radius: Option<f64>,
    /// Wraps `MKLocalPointsOfInterestRequest.region`.
    pub region: Option<MKCoordinateRegion>,
    /// Wraps `MKLocalPointsOfInterestRequest.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
}

impl MKLocalPointsOfInterestRequest {
    /// Wraps `MKLocalPointsOfInterestRequest.maxRadius`.
    pub fn max_radius() -> f64 {
        unsafe { ffi::mk_points_of_interest_request_max_radius() }
    }

    /// Wraps `MKLocalPointsOfInterestRequest.radius`.
    pub fn with_radius(center: MKCoordinate, radius: f64) -> Self {
        Self {
            coordinate: Some(center),
            radius: Some(radius),
            region: None,
            point_of_interest_filter: None,
        }
    }

    /// Wraps `MKLocalPointsOfInterestRequest.region`.
    pub fn with_region(region: MKCoordinateRegion) -> Self {
        Self {
            coordinate: None,
            radius: None,
            region: Some(region),
            point_of_interest_filter: None,
        }
    }

    /// Wraps `MKLocalPointsOfInterestRequest.pointOfInterestFilter`.
    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    /// Wraps `MKLocalPointsOfInterestRequest.validate`.
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
