use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::annotation::MKPointAnnotation;
use crate::cluster_annotation::MKClusterAnnotation;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKScreenPoint};
use crate::map_view::MKFeatureVisibility;
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result};

/// Wraps `MKAnnotation`.
pub trait MKAnnotation {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError>;
    fn title(&self) -> Result<Option<String>, MapKitError>;
    fn subtitle(&self) -> Result<Option<String>, MapKitError>;
    #[doc(hidden)]
    fn as_raw_annotation(&self) -> *mut c_void;
}

impl MKAnnotation for MKPointAnnotation {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Self::title(self)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Self::subtitle(self)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

impl MKAnnotation for MKClusterAnnotation {
    fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Self::coordinate(self)
    }

    fn title(&self) -> Result<Option<String>, MapKitError> {
        Self::title(self)
    }

    fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Self::subtitle(self)
    }

    fn as_raw_annotation(&self) -> *mut c_void {
        self.as_raw()
    }
}

/// Wraps `MKAnnotationViewCollisionMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAnnotationViewCollisionMode {
    Rectangle,
    Circle,
    None,
}

/// Wraps `MKAnnotationViewDragState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAnnotationViewDragState {
    None,
    Starting,
    Dragging,
    Canceling,
    Ending,
}

/// Wraps `MKAnnotationViewZPriority`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKAnnotationViewZPriority(pub f32);

impl MKAnnotationViewZPriority {
    /// Wraps `MKAnnotationViewZPriority.max`.
    pub const MAX: Self = Self(1000.0);
    /// Wraps `MKAnnotationViewZPriority.defaultSelected`.
    pub const DEFAULT_SELECTED: Self = Self(1000.0);
    /// Wraps `MKAnnotationViewZPriority.defaultUnselected`.
    pub const DEFAULT_UNSELECTED: Self = Self(500.0);
    /// Wraps `MKAnnotationViewZPriority.min`.
    pub const MIN: Self = Self(0.0);
}

/// Wraps `MKFeatureDisplayPriority`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKFeatureDisplayPriority(pub f32);

impl MKFeatureDisplayPriority {
    /// Wraps `MKFeatureDisplayPriority.required`.
    pub const REQUIRED: Self = Self(1000.0);
    /// Wraps `MKFeatureDisplayPriority.defaultHigh`.
    pub const DEFAULT_HIGH: Self = Self(750.0);
    /// Wraps `MKFeatureDisplayPriority.defaultLow`.
    pub const DEFAULT_LOW: Self = Self(250.0);
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKAnnotationViewState {
    reuse_identifier: Option<String>,
    annotation_title: Option<String>,
    annotation_subtitle: Option<String>,
    center_offset: MKScreenPoint,
    callout_offset: MKScreenPoint,
    left_callout_offset: MKScreenPoint,
    right_callout_offset: MKScreenPoint,
    enabled: bool,
    highlighted: bool,
    selected: bool,
    can_show_callout: bool,
    draggable: bool,
    drag_state: MKAnnotationViewDragState,
    clustering_identifier: Option<String>,
    display_priority: MKFeatureDisplayPriority,
    z_priority: MKAnnotationViewZPriority,
    selected_z_priority: MKAnnotationViewZPriority,
    collision_mode: MKAnnotationViewCollisionMode,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKAnnotationViewOptions {
    center_offset: Option<MKScreenPoint>,
    callout_offset: Option<MKScreenPoint>,
    left_callout_offset: Option<MKScreenPoint>,
    right_callout_offset: Option<MKScreenPoint>,
    enabled: Option<bool>,
    highlighted: Option<bool>,
    selected: Option<bool>,
    selected_animated: Option<bool>,
    can_show_callout: Option<bool>,
    draggable: Option<bool>,
    drag_state: Option<MKAnnotationViewDragState>,
    drag_state_animated: Option<bool>,
    clustering_identifier_present: bool,
    clustering_identifier: Option<String>,
    display_priority: Option<MKFeatureDisplayPriority>,
    z_priority: Option<MKAnnotationViewZPriority>,
    selected_z_priority: Option<MKAnnotationViewZPriority>,
    collision_mode: Option<MKAnnotationViewCollisionMode>,
}

/// Wraps `MKAnnotationView`.
#[derive(Debug)]
pub struct MKAnnotationView {
    raw: NonNull<c_void>,
}

impl MKAnnotationView {
    /// Creates a wrapper for `MKAnnotationView`.
    pub fn new<A: MKAnnotation + ?Sized>(
        annotation: Option<&A>,
        reuse_identifier: Option<&str>,
    ) -> Result<Self, MapKitError> {
        let reuse_identifier = reuse_identifier
            .map(|value| {
                crate::private::cstring_from_str(value, "MKAnnotationView reuseIdentifier")
            })
            .transpose()?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_annotation_view_new(
                annotation.map_or(ptr::null_mut(), MKAnnotation::as_raw_annotation),
                reuse_identifier
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        let raw = owned_handle(raw, error, "failed to create MKAnnotationView")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKAnnotationViewState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_annotation_view_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKAnnotationView state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKAnnotationView state") }
        }
    }

    fn apply_options(&self, options: &MKAnnotationViewOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKAnnotationView options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_annotation_view_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKAnnotationView") }
    }

    /// Wraps `MKAnnotationView.calloutInfoDidChangeNotification`.
    pub fn callout_info_did_change_notification() -> Result<String, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_annotation_callout_info_did_change_notification(&mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(
                    error,
                    "failed to read MKAnnotationCalloutInfoDidChangeNotification",
                )
            })
        } else {
            Ok(unsafe { crate::private::take_string(payload) }.unwrap_or_default())
        }
    }

    /// Wraps `MKAnnotationView.reuseIdentifier`.
    pub fn reuse_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.reuse_identifier)
    }

    /// Wraps `MKAnnotationView.annotationTitle`.
    pub fn annotation_title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.annotation_title)
    }

    /// Wraps `MKAnnotationView.annotationSubtitle`.
    pub fn annotation_subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.annotation_subtitle)
    }

    /// Wraps `MKAnnotationView.centerOffset`.
    pub fn center_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.center_offset)
    }

    /// Wraps `MKAnnotationView.centerOffset`.
    pub fn set_center_offset(&self, center_offset: MKScreenPoint) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            center_offset: Some(center_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.calloutOffset`.
    pub fn callout_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.callout_offset)
    }

    /// Wraps `MKAnnotationView.calloutOffset`.
    pub fn set_callout_offset(&self, callout_offset: MKScreenPoint) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            callout_offset: Some(callout_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.leftCalloutOffset`.
    pub fn left_callout_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.left_callout_offset)
    }

    /// Wraps `MKAnnotationView.leftCalloutOffset`.
    pub fn set_left_callout_offset(
        &self,
        left_callout_offset: MKScreenPoint,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            left_callout_offset: Some(left_callout_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.rightCalloutOffset`.
    pub fn right_callout_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.right_callout_offset)
    }

    /// Wraps `MKAnnotationView.rightCalloutOffset`.
    pub fn set_right_callout_offset(
        &self,
        right_callout_offset: MKScreenPoint,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            right_callout_offset: Some(right_callout_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.isEnabled`.
    pub fn is_enabled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.enabled)
    }

    /// Wraps `MKAnnotationView.enabled`.
    pub fn set_enabled(&self, enabled: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            enabled: Some(enabled),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.isHighlighted`.
    pub fn is_highlighted(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.highlighted)
    }

    /// Wraps `MKAnnotationView.highlighted`.
    pub fn set_highlighted(&self, highlighted: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            highlighted: Some(highlighted),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.isSelected`.
    pub fn is_selected(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.selected)
    }

    /// Wraps `MKAnnotationView.selected`.
    pub fn set_selected(&self, selected: bool, animated: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            selected: Some(selected),
            selected_animated: Some(animated),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.canShowCallout`.
    pub fn can_show_callout(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_show_callout)
    }

    /// Wraps `MKAnnotationView.canShowCallout`.
    pub fn set_can_show_callout(&self, can_show_callout: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            can_show_callout: Some(can_show_callout),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.isDraggable`.
    pub fn is_draggable(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.draggable)
    }

    /// Wraps `MKAnnotationView.draggable`.
    pub fn set_draggable(&self, draggable: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            draggable: Some(draggable),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.dragState`.
    pub fn drag_state(&self) -> Result<MKAnnotationViewDragState, MapKitError> {
        Ok(self.state()?.drag_state)
    }

    /// Wraps `MKAnnotationView.dragState`.
    pub fn set_drag_state(
        &self,
        drag_state: MKAnnotationViewDragState,
        animated: bool,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            drag_state: Some(drag_state),
            drag_state_animated: Some(animated),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.clusteringIdentifier`.
    pub fn clustering_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.clustering_identifier)
    }

    /// Wraps `MKAnnotationView.clusteringIdentifier`.
    pub fn set_clustering_identifier(
        &self,
        clustering_identifier: Option<&str>,
    ) -> Result<(), MapKitError> {
        let clustering_identifier = clustering_identifier.map(ToOwned::to_owned);
        self.apply_options(&MKAnnotationViewOptions {
            clustering_identifier_present: true,
            clustering_identifier,
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.displayPriority`.
    pub fn display_priority(&self) -> Result<MKFeatureDisplayPriority, MapKitError> {
        Ok(self.state()?.display_priority)
    }

    /// Wraps `MKAnnotationView.displayPriority`.
    pub fn set_display_priority(
        &self,
        display_priority: MKFeatureDisplayPriority,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            display_priority: Some(display_priority),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.zPriority`.
    pub fn z_priority(&self) -> Result<MKAnnotationViewZPriority, MapKitError> {
        Ok(self.state()?.z_priority)
    }

    /// Wraps `MKAnnotationView.zPriority`.
    pub fn set_z_priority(&self, z_priority: MKAnnotationViewZPriority) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            z_priority: Some(z_priority),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.selectedZPriority`.
    pub fn selected_z_priority(&self) -> Result<MKAnnotationViewZPriority, MapKitError> {
        Ok(self.state()?.selected_z_priority)
    }

    /// Wraps `MKAnnotationView.selectedZPriority`.
    pub fn set_selected_z_priority(
        &self,
        selected_z_priority: MKAnnotationViewZPriority,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            selected_z_priority: Some(selected_z_priority),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.collisionMode`.
    pub fn collision_mode(&self) -> Result<MKAnnotationViewCollisionMode, MapKitError> {
        Ok(self.state()?.collision_mode)
    }

    /// Wraps `MKAnnotationView.collisionMode`.
    pub fn set_collision_mode(
        &self,
        collision_mode: MKAnnotationViewCollisionMode,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            collision_mode: Some(collision_mode),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKAnnotationView.prepareForReuse`.
    pub fn prepare_for_reuse(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_annotation_view_prepare_for_reuse(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to prepare MKAnnotationView for reuse") }
    }

    /// Wraps `MKAnnotationView.prepareForDisplay`.
    pub fn prepare_for_display(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_annotation_view_prepare_for_display(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to prepare MKAnnotationView for display") }
    }
}

impl Drop for MKAnnotationView {
    fn drop(&mut self) {
        unsafe { ffi::mk_annotation_view_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMarkerAnnotationViewState {
    base: MKAnnotationViewState,
    title_visibility: MKFeatureVisibility,
    subtitle_visibility: MKFeatureVisibility,
    glyph_text: Option<String>,
    animates_when_added: bool,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKMarkerAnnotationViewOptions {
    title_visibility: Option<MKFeatureVisibility>,
    subtitle_visibility: Option<MKFeatureVisibility>,
    glyph_text_present: bool,
    glyph_text: Option<String>,
    animates_when_added: Option<bool>,
}

/// Wraps `MKMarkerAnnotationView`.
#[derive(Debug)]
pub struct MKMarkerAnnotationView {
    raw: NonNull<c_void>,
}

impl MKMarkerAnnotationView {
    /// Creates a wrapper for `MKMarkerAnnotationView`.
    pub fn new<A: MKAnnotation + ?Sized>(
        annotation: Option<&A>,
        reuse_identifier: Option<&str>,
    ) -> Result<Self, MapKitError> {
        let reuse_identifier = reuse_identifier
            .map(|value| {
                crate::private::cstring_from_str(value, "MKMarkerAnnotationView reuseIdentifier")
            })
            .transpose()?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_marker_annotation_view_new(
                annotation.map_or(ptr::null_mut(), MKAnnotation::as_raw_annotation),
                reuse_identifier
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        let raw = owned_handle(raw, error, "failed to create MKMarkerAnnotationView")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKMarkerAnnotationViewState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_marker_annotation_view_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKMarkerAnnotationView state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKMarkerAnnotationView state") }
        }
    }

    fn apply_options(&self, options: &MKMarkerAnnotationViewOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKMarkerAnnotationView options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_marker_annotation_view_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKMarkerAnnotationView") }
    }

    /// Wraps `MKMarkerAnnotationView.annotationTitle`.
    pub fn annotation_title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.base.annotation_title)
    }

    /// Wraps `MKMarkerAnnotationView.titleVisibility`.
    pub fn title_visibility(&self) -> Result<MKFeatureVisibility, MapKitError> {
        Ok(self.state()?.title_visibility)
    }

    /// Wraps `MKMarkerAnnotationView.titleVisibility`.
    pub fn set_title_visibility(
        &self,
        title_visibility: MKFeatureVisibility,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            title_visibility: Some(title_visibility),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKMarkerAnnotationView.subtitleVisibility`.
    pub fn subtitle_visibility(&self) -> Result<MKFeatureVisibility, MapKitError> {
        Ok(self.state()?.subtitle_visibility)
    }

    /// Wraps `MKMarkerAnnotationView.subtitleVisibility`.
    pub fn set_subtitle_visibility(
        &self,
        subtitle_visibility: MKFeatureVisibility,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            subtitle_visibility: Some(subtitle_visibility),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKMarkerAnnotationView.glyphText`.
    pub fn glyph_text(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.glyph_text)
    }

    /// Wraps `MKMarkerAnnotationView.glyphText`.
    pub fn set_glyph_text(&self, glyph_text: Option<&str>) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            glyph_text_present: true,
            glyph_text: glyph_text.map(ToOwned::to_owned),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKMarkerAnnotationView.animatesWhenAdded`.
    pub fn animates_when_added(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.animates_when_added)
    }

    /// Wraps `MKMarkerAnnotationView.animatesWhenAdded`.
    pub fn set_animates_when_added(&self, animates_when_added: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            animates_when_added: Some(animates_when_added),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }
}

impl Drop for MKMarkerAnnotationView {
    fn drop(&mut self) {
        unsafe { ffi::mk_marker_annotation_view_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKPinAnnotationColor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKPinAnnotationColor {
    Red,
    Green,
    Purple,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKPinAnnotationViewState {
    base: MKAnnotationViewState,
    animates_drop: bool,
    pin_color: MKPinAnnotationColor,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKPinAnnotationViewOptions {
    animates_drop: Option<bool>,
    pin_color: Option<MKPinAnnotationColor>,
}

/// Wraps `MKPinAnnotationView`.
#[derive(Debug)]
pub struct MKPinAnnotationView {
    raw: NonNull<c_void>,
}

impl MKPinAnnotationView {
    /// Creates a wrapper for `MKPinAnnotationView`.
    pub fn new<A: MKAnnotation + ?Sized>(
        annotation: Option<&A>,
        reuse_identifier: Option<&str>,
    ) -> Result<Self, MapKitError> {
        let reuse_identifier = reuse_identifier
            .map(|value| {
                crate::private::cstring_from_str(value, "MKPinAnnotationView reuseIdentifier")
            })
            .transpose()?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_pin_annotation_view_new(
                annotation.map_or(ptr::null_mut(), MKAnnotation::as_raw_annotation),
                reuse_identifier
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        let raw = owned_handle(raw, error, "failed to create MKPinAnnotationView")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKPinAnnotationViewState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_pin_annotation_view_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKPinAnnotationView state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKPinAnnotationView state") }
        }
    }

    fn apply_options(&self, options: &MKPinAnnotationViewOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKPinAnnotationView options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_pin_annotation_view_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKPinAnnotationView") }
    }

    /// Wraps `MKPinAnnotationView.reuseIdentifier`.
    pub fn reuse_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.base.reuse_identifier)
    }

    /// Wraps `MKPinAnnotationView.annotationTitle`.
    pub fn annotation_title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.base.annotation_title)
    }

    /// Wraps `MKPinAnnotationView.annotationSubtitle`.
    pub fn annotation_subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.base.annotation_subtitle)
    }

    /// Wraps `MKPinAnnotationView.animatesDrop`.
    pub fn animates_drop(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.animates_drop)
    }

    /// Wraps `MKPinAnnotationView.animatesDrop`.
    pub fn set_animates_drop(&self, animates_drop: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKPinAnnotationViewOptions {
            animates_drop: Some(animates_drop),
            ..MKPinAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKPinAnnotationView.pinColor`.
    pub fn pin_color(&self) -> Result<MKPinAnnotationColor, MapKitError> {
        Ok(self.state()?.pin_color)
    }

    /// Wraps `MKPinAnnotationView.pinColor`.
    pub fn set_pin_color(&self, pin_color: MKPinAnnotationColor) -> Result<(), MapKitError> {
        self.apply_options(&MKPinAnnotationViewOptions {
            pin_color: Some(pin_color),
            ..MKPinAnnotationViewOptions::default()
        })
    }
}

impl Drop for MKPinAnnotationView {
    fn drop(&mut self) {
        unsafe { ffi::mk_pin_annotation_view_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKUserLocationView`.
#[derive(Debug)]
pub struct MKUserLocationView {
    raw: NonNull<c_void>,
}

impl MKUserLocationView {
    /// Creates a wrapper for `MKUserLocationView`.
    pub fn new<A: MKAnnotation + ?Sized>(
        annotation: Option<&A>,
        reuse_identifier: Option<&str>,
    ) -> Result<Self, MapKitError> {
        let reuse_identifier = reuse_identifier
            .map(|value| {
                crate::private::cstring_from_str(value, "MKUserLocationView reuseIdentifier")
            })
            .transpose()?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_user_location_view_new(
                annotation.map_or(ptr::null_mut(), MKAnnotation::as_raw_annotation),
                reuse_identifier
                    .as_ref()
                    .map_or(ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        let raw = owned_handle(raw, error, "failed to create MKUserLocationView")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKAnnotationViewState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_annotation_view_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKUserLocationView state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKUserLocationView state") }
        }
    }

    fn apply_options(&self, options: &MKAnnotationViewOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKUserLocationView options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_annotation_view_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKUserLocationView") }
    }

    /// Wraps `MKUserLocationView.reuseIdentifier`.
    pub fn reuse_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.reuse_identifier)
    }

    /// Wraps `MKUserLocationView.annotationTitle`.
    pub fn annotation_title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.annotation_title)
    }

    /// Wraps `MKUserLocationView.annotationSubtitle`.
    pub fn annotation_subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.annotation_subtitle)
    }

    /// Wraps `MKUserLocationView.centerOffset`.
    pub fn center_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.center_offset)
    }

    /// Wraps `MKUserLocationView.centerOffset`.
    pub fn set_center_offset(&self, center_offset: MKScreenPoint) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            center_offset: Some(center_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKUserLocationView.canShowCallout`.
    pub fn can_show_callout(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_show_callout)
    }

    /// Wraps `MKUserLocationView.canShowCallout`.
    pub fn set_can_show_callout(&self, can_show_callout: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            can_show_callout: Some(can_show_callout),
            ..MKAnnotationViewOptions::default()
        })
    }

    /// Wraps `MKUserLocationView.prepareForReuse`.
    pub fn prepare_for_reuse(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_annotation_view_prepare_for_reuse(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to prepare MKUserLocationView for reuse") }
    }

    /// Wraps `MKUserLocationView.prepareForDisplay`.
    pub fn prepare_for_display(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_annotation_view_prepare_for_display(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to prepare MKUserLocationView for display") }
    }
}

impl Drop for MKUserLocationView {
    fn drop(&mut self) {
        unsafe { ffi::mk_user_location_view_release(self.raw.as_ptr()) };
    }
}
