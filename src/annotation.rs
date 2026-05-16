use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKCoordinate;
use crate::private::{cstring_from_str, json_cstring, owned_handle, parse_json_ptr, unit_result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKPointAnnotationState {
    coordinate: MKCoordinate,
    title: Option<String>,
    subtitle: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKPointAnnotationOptions {
    coordinate: Option<MKCoordinate>,
    title_present: bool,
    title: Option<String>,
    subtitle_present: bool,
    subtitle: Option<String>,
}

#[derive(Debug)]
pub struct MKPointAnnotation {
    raw: NonNull<c_void>,
}

impl MKPointAnnotation {
    pub fn new(coordinate: MKCoordinate) -> Result<Self, MapKitError> {
        let payload = MKPointAnnotationState {
            coordinate,
            title: None,
            subtitle: None,
        };
        let payload = json_cstring(&payload, "MKPointAnnotation")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_point_annotation_new_json(payload.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKPointAnnotation")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKPointAnnotationState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_point_annotation_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKPointAnnotation state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKPointAnnotation state") }
        }
    }

    fn apply(&self, options: &MKPointAnnotationOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKPointAnnotation options")?;
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_point_annotation_apply_json(self.raw.as_ptr(), options.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to update MKPointAnnotation") }
    }

    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    pub fn title(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.title)
    }

    pub fn subtitle(&self) -> Result<Option<String>, MapKitError> {
        Ok(self.state()?.subtitle)
    }

    pub fn set_coordinate(&self, coordinate: MKCoordinate) -> Result<(), MapKitError> {
        self.apply(&MKPointAnnotationOptions {
            coordinate: Some(coordinate),
            ..MKPointAnnotationOptions::default()
        })
    }

    pub fn set_title(&self, title: Option<&str>) -> Result<(), MapKitError> {
        if let Some(title) = title {
            let _ = cstring_from_str(title, "MKPointAnnotation title")?;
        }
        self.apply(&MKPointAnnotationOptions {
            title_present: true,
            title: title.map(ToOwned::to_owned),
            ..MKPointAnnotationOptions::default()
        })
    }

    pub fn set_subtitle(&self, subtitle: Option<&str>) -> Result<(), MapKitError> {
        if let Some(subtitle) = subtitle {
            let _ = cstring_from_str(subtitle, "MKPointAnnotation subtitle")?;
        }
        self.apply(&MKPointAnnotationOptions {
            subtitle_present: true,
            subtitle: subtitle.map(ToOwned::to_owned),
            ..MKPointAnnotationOptions::default()
        })
    }

    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKPointAnnotation {
    fn drop(&mut self) {
        unsafe { ffi::mk_point_annotation_release(self.raw.as_ptr()) };
    }
}
