use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::annotation::{MKPointAnnotation, MKUserLocation};
use crate::annotation_view::MKAnnotation;
use crate::cluster_annotation::MKClusterAnnotation;
use crate::configuration::{
    MKMapCamera, MKMapCameraBoundary, MKMapCameraZoomRange, MKMapConfiguration,
};
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKCoordinateRegion, MKMapRect, MKScreenPoint, MKScreenSize};
use crate::overlay::{MKCircle, MKOverlay, MKOverlayLevel, MKPolygon, MKPolyline};
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
    camera: MKMapCamera,
    camera_zoom_range: Option<MKMapCameraZoomRange>,
    camera_boundary: Option<MKMapCameraBoundary>,
    preferred_configuration: Option<MKMapConfiguration>,
    zoom_enabled: bool,
    scroll_enabled: bool,
    rotate_enabled: bool,
    pitch_enabled: bool,
    shows_zoom_controls: bool,
    shows_compass: bool,
    shows_scale: bool,
    shows_points_of_interest: bool,
    shows_user_location: bool,
    user_location_visible: bool,
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
    camera: Option<MKMapCamera>,
    camera_zoom_range_present: bool,
    camera_zoom_range: Option<MKMapCameraZoomRange>,
    camera_boundary_present: bool,
    camera_boundary: Option<MKMapCameraBoundary>,
    preferred_configuration: Option<MKMapConfiguration>,
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
        unsafe {
            ffi::mk_map_view_apply_options_json(self.raw.as_ptr(), options.as_ptr(), &mut error);
        };
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

    pub fn camera(&self) -> Result<MKMapCamera, MapKitError> {
        Ok(self.state()?.camera)
    }

    pub fn set_camera(&self, camera: MKMapCamera, animated: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            camera: Some(camera),
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn camera_zoom_range(&self) -> Result<Option<MKMapCameraZoomRange>, MapKitError> {
        Ok(self.state()?.camera_zoom_range)
    }

    pub fn set_camera_zoom_range(
        &self,
        camera_zoom_range: Option<MKMapCameraZoomRange>,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            camera_zoom_range_present: true,
            camera_zoom_range,
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn camera_boundary(&self) -> Result<Option<MKMapCameraBoundary>, MapKitError> {
        Ok(self.state()?.camera_boundary)
    }

    pub fn set_camera_boundary(
        &self,
        camera_boundary: Option<MKMapCameraBoundary>,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            camera_boundary_present: true,
            camera_boundary,
            animated: Some(animated),
            ..MKMapViewOptions::default()
        })
    }

    pub fn preferred_configuration(&self) -> Result<Option<MKMapConfiguration>, MapKitError> {
        Ok(self.state()?.preferred_configuration)
    }

    pub fn set_preferred_configuration(
        &self,
        preferred_configuration: MKMapConfiguration,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMapViewOptions {
            preferred_configuration: Some(preferred_configuration),
            ..MKMapViewOptions::default()
        })
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
            Err(unsafe { MapKitError::from_error_ptr(error, "MKMapView convertCoordinate failed") })
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

    pub fn is_user_location_visible(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.user_location_visible)
    }

    pub fn user_location(&self) -> Result<MKUserLocation, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_map_view_user_location(self.raw.as_ptr(), &mut error) };
        if raw.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKMapView userLocation")
            })
        } else {
            MKUserLocation::from_owned_raw(raw)
        }
    }

    pub fn default_annotation_view_reuse_identifier() -> Result<String, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_map_view_default_annotation_view_reuse_identifier(&mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(
                    error,
                    "failed to read MKMapViewDefaultAnnotationViewReuseIdentifier",
                )
            })
        } else {
            Ok(unsafe { crate::private::take_string(payload) }.unwrap_or_default())
        }
    }

    pub fn default_cluster_annotation_view_reuse_identifier() -> Result<String, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_view_default_cluster_annotation_view_reuse_identifier(&mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(
                    error,
                    "failed to read MKMapViewDefaultClusterAnnotationViewReuseIdentifier",
                )
            })
        } else {
            Ok(unsafe { crate::private::take_string(payload) }.unwrap_or_default())
        }
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

    pub fn add_annotation<A: MKAnnotation + ?Sized>(
        &self,
        annotation: &A,
    ) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_map_view_add_annotation(
                self.raw.as_ptr(),
                annotation.as_raw_annotation(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to add MKAnnotation") }
    }

    pub fn remove_annotation<A: MKAnnotation + ?Sized>(
        &self,
        annotation: &A,
    ) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_map_view_remove_annotation(
                self.raw.as_ptr(),
                annotation.as_raw_annotation(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to remove MKAnnotation") }
    }

    pub fn add_point_annotation(&self, annotation: &MKPointAnnotation) -> Result<(), MapKitError> {
        self.add_annotation(annotation)
    }

    pub fn remove_point_annotation(
        &self,
        annotation: &MKPointAnnotation,
    ) -> Result<(), MapKitError> {
        self.remove_annotation(annotation)
    }

    pub fn add_cluster_annotation(
        &self,
        annotation: &MKClusterAnnotation,
    ) -> Result<(), MapKitError> {
        self.add_annotation(annotation)
    }

    pub fn remove_cluster_annotation(
        &self,
        annotation: &MKClusterAnnotation,
    ) -> Result<(), MapKitError> {
        self.remove_annotation(annotation)
    }

    pub fn add_overlay<O: MKOverlay + ?Sized>(
        &self,
        overlay: &O,
        level: MKOverlayLevel,
    ) -> Result<(), MapKitError> {
        let level = json_cstring(&level, "MKOverlayLevel")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_map_view_add_overlay(
                self.raw.as_ptr(),
                overlay.as_raw_overlay(),
                level.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to add MKOverlay") }
    }

    pub fn remove_overlay<O: MKOverlay + ?Sized>(&self, overlay: &O) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_map_view_remove_overlay(
                self.raw.as_ptr(),
                overlay.as_raw_overlay(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to remove MKOverlay") }
    }

    pub fn add_circle(&self, circle: &MKCircle, level: MKOverlayLevel) -> Result<(), MapKitError> {
        self.add_overlay(circle, level)
    }

    pub fn remove_circle(&self, circle: &MKCircle) -> Result<(), MapKitError> {
        self.remove_overlay(circle)
    }

    pub fn add_polyline(
        &self,
        polyline: &MKPolyline,
        level: MKOverlayLevel,
    ) -> Result<(), MapKitError> {
        self.add_overlay(polyline, level)
    }

    pub fn remove_polyline(&self, polyline: &MKPolyline) -> Result<(), MapKitError> {
        self.remove_overlay(polyline)
    }

    pub fn add_polygon(
        &self,
        polygon: &MKPolygon,
        level: MKOverlayLevel,
    ) -> Result<(), MapKitError> {
        self.add_overlay(polygon, level)
    }

    pub fn remove_polygon(&self, polygon: &MKPolygon) -> Result<(), MapKitError> {
        self.remove_overlay(polygon)
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
