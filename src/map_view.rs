use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::annotation::MKPointAnnotation;
use crate::cluster_annotation::MKClusterAnnotation;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{
    MKCoordinate, MKCoordinateRegion, MKMapRect, MKScreenPoint, MKScreenSize,
};
use crate::overlay::{MKCircle, MKOverlayLevel, MKPolygon, MKPolyline};
use crate::point_of_interest::MKPointOfInterestFilter;
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKMapType {
    Standard,
    Satellite,
    Hybrid,
    SatelliteFlyover,
    HybridFlyover,
    MutedStandard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKFeatureVisibility {
    Adaptive,
    Hidden,
    Visible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKUserTrackingMode {
    None,
    Follow,
    FollowWithHeading,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMapViewState {
    map_type: MKMapType,
    region: MKCoordinateRegion,
    center_coordinate: MKCoordinate,
    visible_map_rect: MKMapRect,
    zoom_enabled: bool,
    scroll_enabled: bool,
    rotate_enabled: bool,
    pitch_enabled: bool,
    shows_zoom_controls: bool,
    shows_compass: bool,
    shows_scale: bool,
    shows_points_of_interest: bool,
    shows_user_location: bool,
    shows_user_tracking_button: bool,
    pitch_button_visibility: Option<MKFeatureVisibility>,
    user_tracking_mode: MKUserTrackingMode,
    annotation_count: usize,
    overlay_count: usize,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKMapViewOptions {
    map_type: Option<MKMapType>,
    region: Option<MKCoordinateRegion>,
    center_coordinate: Option<MKCoordinate>,
    visible_map_rect: Option<MKMapRect>,
    zoom_enabled: Option<bool>,
    scroll_enabled: Option<bool>,
    rotate_enabled: Option<bool>,
    pitch_enabled: Option<bool>,
    shows_zoom_controls: Option<bool>,
    shows_compass: Option<bool>,
    shows_scale: Option<bool>,
    shows_points_of_interest: Option<bool>,
    shows_user_location: Option<bool>,
    shows_user_tracking_button: Option<bool>,
    point_of_interest_filter_present: bool,
    point_of_interest_filter: Option<MKPointOfInterestFilter>,
    pitch_button_visibility: Option<MKFeatureVisibility>,
    user_tracking_mode: Option<MKUserTrackingMode>,
    animated: Option<bool>,
}

#[derive(Debug)]
pub struct MKMapView {
    raw: NonNull<c_void>,
}

impl MKMapView {
    pub fn new(size: MKScreenSize) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_map_view_new(size.width, size.height, &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKMapView")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMapViewState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_map_view_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "failed to read MKMapView state") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapView state") }
        }
    }

    fn apply_options(&self, options: &MKMapViewOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKMapView options")?;
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_apply_options_json(self.raw.as_ptr(), options.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to update MKMapView") }
    }

    pub fn map_type(&self) -> Result<MKMapType, MapKitError> {
        Ok(self.state()?.map_type)
    }

    pub fn set_map_type(&self, map_type: MKMapType) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            map_type: Some(map_type),
            ..MKMapViewOptions::default()
        })
    }

    pub fn region(&self) -> Result<MKCoordinateRegion, MapKitError> {
        Ok(self.state()?.region)
    }

    pub fn set_region(
        &self,
        region: MKCoordinateRegion,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            region: Some(region),
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn center_coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.center_coordinate)
    }

    pub fn set_center_coordinate(
        &self,
        center_coordinate: MKCoordinate,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            center_coordinate: Some(center_coordinate),
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn region_that_fits(
        &self,
        region: MKCoordinateRegion,
    ) -> Result<MKCoordinateRegion, MapKitError> {
        let region = json_cstring(&region, "MKCoordinateRegion")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_view_region_that_fits_json(self.raw.as_ptr(), region.as_ptr(), &mut error)
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapView regionThatFits failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKCoordinateRegion") }
        }
    }

    pub fn visible_map_rect(&self) -> Result<MKMapRect, MapKitError> {
        Ok(self.state()?.visible_map_rect)
    }

    pub fn set_visible_map_rect(
        &self,
        visible_map_rect: MKMapRect,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            visible_map_rect: Some(visible_map_rect),
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn map_rect_that_fits(&self, map_rect: MKMapRect) -> Result<MKMapRect, MapKitError> {
        let map_rect = json_cstring(&map_rect, "MKMapRect")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_view_map_rect_that_fits_json(
                self.raw.as_ptr(),
                map_rect.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapView mapRectThatFits failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapRect") }
        }
    }

    pub fn convert_coordinate_to_point(
        &self,
        coordinate: MKCoordinate,
    ) -> Result<MKScreenPoint, MapKitError> {
        let coordinate = json_cstring(&coordinate, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_view_convert_coordinate_to_point_json(
                self.raw.as_ptr(),
                coordinate.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKMapView convertCoordinate failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKScreenPoint") }
        }
    }

    pub fn convert_point_to_coordinate(
        &self,
        point: MKScreenPoint,
    ) -> Result<MKCoordinate, MapKitError> {
        let point = json_cstring(&point, "MKScreenPoint")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_view_convert_point_to_coordinate_json(
                self.raw.as_ptr(),
                point.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapView convertPoint failed") })
        } else {
            unsafe { parse_json_ptr(payload, "MKCoordinate") }
        }
    }

    pub fn is_zoom_enabled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.zoom_enabled)
    }

    pub fn set_zoom_enabled(&self, zoom_enabled: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            zoom_enabled: Some(zoom_enabled),
            ..MKMapViewOptions::default()
        })
    }

    pub fn is_scroll_enabled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.scroll_enabled)
    }

    pub fn set_scroll_enabled(&self, scroll_enabled: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            scroll_enabled: Some(scroll_enabled),
            ..MKMapViewOptions::default()
        })
    }

    pub fn is_rotate_enabled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.rotate_enabled)
    }

    pub fn set_rotate_enabled(&self, rotate_enabled: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            rotate_enabled: Some(rotate_enabled),
            ..MKMapViewOptions::default()
        })
    }

    pub fn is_pitch_enabled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.pitch_enabled)
    }

    pub fn set_pitch_enabled(&self, pitch_enabled: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            pitch_enabled: Some(pitch_enabled),
            ..MKMapViewOptions::default()
        })
    }

    pub fn shows_zoom_controls(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.shows_zoom_controls)
    }

    pub fn set_shows_zoom_controls(&self, shows_zoom_controls: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            shows_zoom_controls: Some(shows_zoom_controls),
            ..MKMapViewOptions::default()
        })
    }

    pub fn shows_compass(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.shows_compass)
    }

    pub fn set_shows_compass(&self, shows_compass: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            shows_compass: Some(shows_compass),
            ..MKMapViewOptions::default()
        })
    }

    pub fn shows_scale(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.shows_scale)
    }

    pub fn set_shows_scale(&self, shows_scale: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            shows_scale: Some(shows_scale),
            ..MKMapViewOptions::default()
        })
    }

    pub fn shows_points_of_interest(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.shows_points_of_interest)
    }

    pub fn set_shows_points_of_interest(
        &self,
        shows_points_of_interest: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            shows_points_of_interest: Some(shows_points_of_interest),
            ..MKMapViewOptions::default()
        })
    }

    pub fn set_point_of_interest_filter(
        &self,
        point_of_interest_filter: Option<MKPointOfInterestFilter>,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            point_of_interest_filter_present: true,
            point_of_interest_filter,
            ..MKMapViewOptions::default()
        })
    }

    pub fn shows_user_location(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.shows_user_location)
    }

    pub fn set_shows_user_location(&self, shows_user_location: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            shows_user_location: Some(shows_user_location),
            ..MKMapViewOptions::default()
        })
    }

    pub fn user_tracking_mode(&self) -> Result<MKUserTrackingMode, MapKitError> {
        Ok(self.state()?.user_tracking_mode)
    }

    pub fn set_user_tracking_mode(
        &self,
        user_tracking_mode: MKUserTrackingMode,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            user_tracking_mode: Some(user_tracking_mode),
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn shows_user_tracking_button(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.shows_user_tracking_button)
    }

    pub fn set_shows_user_tracking_button(
        &self,
        shows_user_tracking_button: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            shows_user_tracking_button: Some(shows_user_tracking_button),
            ..MKMapViewOptions::default()
        })
    }

    pub fn pitch_button_visibility(&self) -> Result<Option<MKFeatureVisibility>, MapKitError> {
        Ok(self.state()?.pitch_button_visibility)
    }

    pub fn set_pitch_button_visibility(
        &self,
        pitch_button_visibility: MKFeatureVisibility,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            pitch_button_visibility: Some(pitch_button_visibility),
            ..MKMapViewOptions::default()
        })
    }

    pub fn annotation_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.annotation_count)
    }

    pub fn overlay_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.overlay_count)
    }

    pub fn add_point_annotation(
        &self,
        annotation: &MKPointAnnotation,
    ) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_add_point_annotation(self.raw.as_ptr(), annotation.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to add MKPointAnnotation") }
    }

    pub fn remove_point_annotation(
        &self,
        annotation: &MKPointAnnotation,
    ) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_remove_point_annotation(self.raw.as_ptr(), annotation.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to remove MKPointAnnotation") }
    }

    pub fn add_cluster_annotation(
        &self,
        annotation: &MKClusterAnnotation,
    ) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_add_cluster_annotation(self.raw.as_ptr(), annotation.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to add MKClusterAnnotation") }
    }

    pub fn remove_cluster_annotation(
        &self,
        annotation: &MKClusterAnnotation,
    ) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_remove_cluster_annotation(self.raw.as_ptr(), annotation.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to remove MKClusterAnnotation") }
    }

    pub fn add_circle(&self, circle: &MKCircle, level: MKOverlayLevel) -> Result<(), MapKitError> {
        let level = json_cstring(&level, "MKOverlayLevel")?;
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_add_circle(self.raw.as_ptr(), circle.as_raw(), level.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to add MKCircle") }
    }

    pub fn remove_circle(&self, circle: &MKCircle) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_remove_circle(self.raw.as_ptr(), circle.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to remove MKCircle") }
    }

    pub fn add_polyline(
        &self,
        polyline: &MKPolyline,
        level: MKOverlayLevel,
    ) -> Result<(), MapKitError> {
        let level = json_cstring(&level, "MKOverlayLevel")?;
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_add_polyline(self.raw.as_ptr(), polyline.as_raw(), level.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to add MKPolyline") }
    }

    pub fn remove_polyline(&self, polyline: &MKPolyline) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_remove_polyline(self.raw.as_ptr(), polyline.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to remove MKPolyline") }
    }

    pub fn add_polygon(
        &self,
        polygon: &MKPolygon,
        level: MKOverlayLevel,
    ) -> Result<(), MapKitError> {
        let level = json_cstring(&level, "MKOverlayLevel")?;
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_add_polygon(self.raw.as_ptr(), polygon.as_raw(), level.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to add MKPolygon") }
    }

    pub fn remove_polygon(&self, polygon: &MKPolygon) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_map_view_remove_polygon(self.raw.as_ptr(), polygon.as_raw(), &mut error) };
        unsafe { unit_result(error, "failed to remove MKPolygon") }
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKMapView {
    fn drop(&mut self) {
        unsafe { ffi::mk_map_view_release(self.raw.as_ptr()) };
    }
}
