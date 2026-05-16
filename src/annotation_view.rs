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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAnnotationViewCollisionMode {
    Rectangle,
    Circle,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKAnnotationViewDragState {
    None,
    Starting,
    Dragging,
    Canceling,
    Ending,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKAnnotationViewZPriority(pub f32);

impl MKAnnotationViewZPriority {
    pub const MAX: Self = Self(1000.0);
    pub const DEFAULT_SELECTED: Self = Self(1000.0);
    pub const DEFAULT_UNSELECTED: Self = Self(500.0);
    pub const MIN: Self = Self(0.0);
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MKFeatureDisplayPriority(pub f32);

impl MKFeatureDisplayPriority {
    pub const REQUIRED: Self = Self(1000.0);
    pub const DEFAULT_HIGH: Self = Self(750.0);
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

#[derive(Debug)]
pub struct MKAnnotationView {
    raw: NonNull<c_void>,
}

impl MKAnnotationView {
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

    pub fn reuse_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.reuse_identifier)
    }

    pub fn annotation_title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.annotation_title)
    }

    pub fn annotation_subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.annotation_subtitle)
    }

    pub fn center_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.center_offset)
    }

    pub fn set_center_offset(&self, center_offset: MKScreenPoint) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            center_offset: Some(center_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn callout_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.callout_offset)
    }

    pub fn set_callout_offset(&self, callout_offset: MKScreenPoint) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            callout_offset: Some(callout_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn left_callout_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.left_callout_offset)
    }

    pub fn set_left_callout_offset(
        &self,
        left_callout_offset: MKScreenPoint,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            left_callout_offset: Some(left_callout_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn right_callout_offset(&self) -> Result<MKScreenPoint, MapKitError> {
        Ok(self.state()?.right_callout_offset)
    }

    pub fn set_right_callout_offset(
        &self,
        right_callout_offset: MKScreenPoint,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            right_callout_offset: Some(right_callout_offset),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn is_enabled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.enabled)
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            enabled: Some(enabled),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn is_highlighted(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.highlighted)
    }

    pub fn set_highlighted(&self, highlighted: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            highlighted: Some(highlighted),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn is_selected(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.selected)
    }

    pub fn set_selected(&self, selected: bool, animated: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            selected: Some(selected),
            selected_animated: Some(animated),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn can_show_callout(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.can_show_callout)
    }

    pub fn set_can_show_callout(&self, can_show_callout: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            can_show_callout: Some(can_show_callout),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn is_draggable(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.draggable)
    }

    pub fn set_draggable(&self, draggable: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            draggable: Some(draggable),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn drag_state(&self) -> Result<MKAnnotationViewDragState, MapKitError> {
        Ok(self.state()?.drag_state)
    }

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

    pub fn clustering_identifier(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.clustering_identifier)
    }

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

    pub fn display_priority(&self) -> Result<MKFeatureDisplayPriority, MapKitError> {
        Ok(self.state()?.display_priority)
    }

    pub fn set_display_priority(
        &self,
        display_priority: MKFeatureDisplayPriority,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            display_priority: Some(display_priority),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn z_priority(&self) -> Result<MKAnnotationViewZPriority, MapKitError> {
        Ok(self.state()?.z_priority)
    }

    pub fn set_z_priority(&self, z_priority: MKAnnotationViewZPriority) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            z_priority: Some(z_priority),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn selected_z_priority(&self) -> Result<MKAnnotationViewZPriority, MapKitError> {
        Ok(self.state()?.selected_z_priority)
    }

    pub fn set_selected_z_priority(
        &self,
        selected_z_priority: MKAnnotationViewZPriority,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            selected_z_priority: Some(selected_z_priority),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn collision_mode(&self) -> Result<MKAnnotationViewCollisionMode, MapKitError> {
        Ok(self.state()?.collision_mode)
    }

    pub fn set_collision_mode(
        &self,
        collision_mode: MKAnnotationViewCollisionMode,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKAnnotationViewOptions {
            collision_mode: Some(collision_mode),
            ..MKAnnotationViewOptions::default()
        })
    }

    pub fn prepare_for_reuse(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_annotation_view_prepare_for_reuse(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to prepare MKAnnotationView for reuse") }
    }

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

#[derive(Debug)]
pub struct MKMarkerAnnotationView {
    raw: NonNull<c_void>,
}

impl MKMarkerAnnotationView {
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

    pub fn annotation_title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.base.annotation_title)
    }

    pub fn title_visibility(&self) -> Result<MKFeatureVisibility, MapKitError> {
        Ok(self.state()?.title_visibility)
    }

    pub fn set_title_visibility(
        &self,
        title_visibility: MKFeatureVisibility,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            title_visibility: Some(title_visibility),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }

    pub fn subtitle_visibility(&self) -> Result<MKFeatureVisibility, MapKitError> {
        Ok(self.state()?.subtitle_visibility)
    }

    pub fn set_subtitle_visibility(
        &self,
        subtitle_visibility: MKFeatureVisibility,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            subtitle_visibility: Some(subtitle_visibility),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }

    pub fn glyph_text(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.glyph_text)
    }

    pub fn set_glyph_text(&self, glyph_text: Option<&str>) -> Result<(), MapKitError> {
        self.apply_options(&MKMarkerAnnotationViewOptions {
            glyph_text_present: true,
            glyph_text: glyph_text.map(ToOwned::to_owned),
            ..MKMarkerAnnotationViewOptions::default()
        })
    }

    pub fn animates_when_added(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.animates_when_added)
    }

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
