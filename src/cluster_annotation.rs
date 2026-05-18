use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::annotation::MKPointAnnotation;
use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinate;
use crate::private::{cstring_from_str, json_cstring, owned_handle, parse_json_ptr, unit_result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKClusterAnnotationState {
    coordinate: MKCoordinate,
    title: Option<String>,
    subtitle: Option<String>,
    member_count: usize,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKClusterAnnotationOptions {
    title_present: bool,
    title: Option<String>,
    subtitle_present: bool,
    subtitle: Option<String>,
}

/// Wraps `MKClusterAnnotation`.
#[derive(Debug)]
pub struct MKClusterAnnotation {
    raw: NonNull<c_void>,
}

impl MKClusterAnnotation {
    /// Creates a wrapper for `MKClusterAnnotation`.
    pub fn new(member_annotations: &[MKPointAnnotation]) -> Result<Self, MapKitError> {
        let raw_members: Vec<*mut c_void> = member_annotations
            .iter()
            .map(MKPointAnnotation::as_raw)
            .collect();
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_cluster_annotation_new(raw_members.as_ptr(), raw_members.len(), &mut error)
        };
        let raw = owned_handle(raw, error, "failed to create MKClusterAnnotation")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKClusterAnnotationState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_cluster_annotation_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKClusterAnnotation state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKClusterAnnotation state") }
        }
    }

    fn apply(&self, options: &MKClusterAnnotationOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKClusterAnnotation options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_cluster_annotation_apply_json(self.raw.as_ptr(), options.as_ptr(), &mut error)
        };
        unsafe { unit_result(error, "failed to update MKClusterAnnotation") }
    }

    /// Wraps `MKClusterAnnotation.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKClusterAnnotation.title`.
    pub fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.title)
    }

    /// Wraps `MKClusterAnnotation.subtitle`.
    pub fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.subtitle)
    }

    /// Wraps `MKClusterAnnotation.memberCount`.
    pub fn member_count(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.member_count)
    }

    /// Wraps `MKClusterAnnotation.title`.
    pub fn set_title(&self, title: Option<&str>) -> Result<(), MapKitError> {
        if let Some(title) = title {
            let _ = cstring_from_str(title, "MKClusterAnnotation title")?;
        }
        self.apply(&MKClusterAnnotationOptions {
            title_present: true,
            title: title.map(ToOwned::to_owned),
            ..MKClusterAnnotationOptions::default()
        })
    }

    /// Wraps `MKClusterAnnotation.subtitle`.
    pub fn set_subtitle(&self, subtitle: Option<&str>) -> Result<(), MapKitError> {
        if let Some(subtitle) = subtitle {
            let _ = cstring_from_str(subtitle, "MKClusterAnnotation subtitle")?;
        }
        self.apply(&MKClusterAnnotationOptions {
            subtitle_present: true,
            subtitle: subtitle.map(ToOwned::to_owned),
            ..MKClusterAnnotationOptions::default()
        })
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKClusterAnnotation {
    fn drop(&mut self) {
        unsafe { ffi::mk_cluster_annotation_release(self.raw.as_ptr()) };
    }
}
