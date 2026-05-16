use serde::{Deserialize, Serialize};

use crate::geometry::{MKCoordinate, MKCoordinateRegion};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKPointOfInterestCategory(pub String);

impl MKPointOfInterestCategory {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn airport() -> Self {
        Self::new("airport")
    }

    pub fn cafe() -> Self {
        Self::new("cafe")
    }

    pub fn hotel() -> Self {
        Self::new("hotel")
    }

    pub fn library() -> Self {
        Self::new("library")
    }

    pub fn museum() -> Self {
        Self::new("museum")
    }

    pub fn park() -> Self {
        Self::new("park")
    }

    pub fn restaurant() -> Self {
        Self::new("restaurant")
    }

    pub fn school() -> Self {
        Self::new("school")
    }

    pub fn store() -> Self {
        Self::new("store")
    }

    pub fn university() -> Self {
        Self::new("university")
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
            MKPointOfInterestFilterMode::Excluding | MKPointOfInterestFilterMode::ExcludingAll => false,
        }
    }

    pub fn excludes_category(&self, category: &MKPointOfInterestCategory) -> bool {
        match self.mode {
            MKPointOfInterestFilterMode::Excluding => self.categories.contains(category),
            MKPointOfInterestFilterMode::ExcludingAll => true,
            MKPointOfInterestFilterMode::Including | MKPointOfInterestFilterMode::IncludingAll => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLocalPointsOfInterestRequest {
    pub coordinate: Option<MKCoordinate>,
    pub radius: Option<f64>,
    pub region: Option<MKCoordinateRegion>,
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
}

impl MKLocalPointsOfInterestRequest {
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

}
