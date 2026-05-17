use crate::error::MapKitError;
use crate::map_view::{MKFeatureVisibility, MKMapView};

#[derive(Debug)]
pub struct MKCompassButton<'a> {
    map_view: Option<&'a MKMapView>,
    compass_visibility: MKFeatureVisibility,
}

impl<'a> MKCompassButton<'a> {
    pub const fn new(map_view: Option<&'a MKMapView>) -> Self {
        Self {
            map_view,
            compass_visibility: MKFeatureVisibility::Adaptive,
        }
    }

    pub const fn map_view(&self) -> Option<&'a MKMapView> {
        self.map_view
    }

    pub fn set_map_view(&mut self, map_view: Option<&'a MKMapView>) {
        self.map_view = map_view;
    }

    pub const fn compass_visibility(&self) -> MKFeatureVisibility {
        self.compass_visibility
    }

    pub fn set_compass_visibility(&mut self, compass_visibility: MKFeatureVisibility) {
        self.compass_visibility = compass_visibility;
    }

    pub fn shows_compass(&self) -> Result<bool, MapKitError> {
        self.map_view_required()?.shows_compass()
    }

    pub fn set_shows_compass(&self, shows_compass: bool) -> Result<(), MapKitError> {
        self.map_view_required()?.set_shows_compass(shows_compass)
    }

    fn map_view_required(&self) -> Result<&MKMapView, MapKitError> {
        self.map_view.ok_or_else(|| {
            MapKitError::InvalidArgument(
                "MKCompassButton must be attached to an MKMapView".to_owned(),
            )
        })
    }
}

#[derive(Debug)]
pub struct MKPitchControl<'a> {
    map_view: Option<&'a MKMapView>,
}

impl<'a> MKPitchControl<'a> {
    pub const fn new(map_view: Option<&'a MKMapView>) -> Self {
        Self { map_view }
    }

    pub const fn map_view(&self) -> Option<&'a MKMapView> {
        self.map_view
    }

    pub fn set_map_view(&mut self, map_view: Option<&'a MKMapView>) {
        self.map_view = map_view;
    }

    pub fn is_pitch_enabled(&self) -> Result<bool, MapKitError> {
        self.map_view_required()?.is_pitch_enabled()
    }

    pub fn set_pitch_enabled(&self, pitch_enabled: bool) -> Result<(), MapKitError> {
        self.map_view_required()?.set_pitch_enabled(pitch_enabled)
    }

    fn map_view_required(&self) -> Result<&MKMapView, MapKitError> {
        self.map_view.ok_or_else(|| {
            MapKitError::InvalidArgument(
                "MKPitchControl must be attached to an MKMapView".to_owned(),
            )
        })
    }
}

#[derive(Debug)]
pub struct MKZoomControl<'a> {
    map_view: Option<&'a MKMapView>,
}

impl<'a> MKZoomControl<'a> {
    pub const fn new(map_view: Option<&'a MKMapView>) -> Self {
        Self { map_view }
    }

    pub const fn map_view(&self) -> Option<&'a MKMapView> {
        self.map_view
    }

    pub fn set_map_view(&mut self, map_view: Option<&'a MKMapView>) {
        self.map_view = map_view;
    }

    pub fn shows_zoom_controls(&self) -> Result<bool, MapKitError> {
        self.map_view_required()?.shows_zoom_controls()
    }

    pub fn set_shows_zoom_controls(&self, shows_zoom_controls: bool) -> Result<(), MapKitError> {
        self.map_view_required()?.set_shows_zoom_controls(shows_zoom_controls)
    }

    fn map_view_required(&self) -> Result<&MKMapView, MapKitError> {
        self.map_view.ok_or_else(|| {
            MapKitError::InvalidArgument(
                "MKZoomControl must be attached to an MKMapView".to_owned(),
            )
        })
    }
}
