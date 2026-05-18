use serde::Deserialize;

use crate::error::MapKitError;
use crate::ffi;
use crate::map_view::{MKMapView, MKUserTrackingMode};
use crate::private::parse_json_ptr;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKUserTrackingButtonState {
    visible: bool,
    tracking_mode: MKUserTrackingMode,
}

/// Wraps `MKUserTrackingButton`.
#[derive(Debug)]
pub struct MKUserTrackingButton<'a> {
    map_view: &'a MKMapView,
}

impl<'a> MKUserTrackingButton<'a> {
    /// Wraps `new`.
    pub const fn new(map_view: &'a MKMapView) -> Self {
        Self { map_view }
    }

    fn state(&self) -> Result<MKUserTrackingButtonState, MapKitError> {
        let mut error = std::ptr::null_mut();
        let payload =
            unsafe { ffi::mk_user_tracking_button_state_json(self.map_view.as_raw(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKUserTrackingButton state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKUserTrackingButton state") }
        }
    }

    /// Wraps `is_visible`.
    pub fn is_visible(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.visible)
    }

    /// Wraps `tracking_mode`.
    pub fn tracking_mode(&self) -> Result<MKUserTrackingMode, MapKitError> {
        Ok(self.state()?.tracking_mode)
    }

    /// Wraps `set_visible`.
    pub fn set_visible(&self, visible: bool) -> Result<(), MapKitError> {
        let mut error = std::ptr::null_mut();
        unsafe {
            ffi::mk_user_tracking_button_set_visible(self.map_view.as_raw(), visible, &mut error)
        };
        unsafe {
            crate::private::unit_result(error, "failed to set MKUserTrackingButton visibility")
        }
    }

    /// Wraps `set_tracking_mode`.
    pub fn set_tracking_mode(
        &self,
        tracking_mode: MKUserTrackingMode,
        animated: bool,
    ) -> Result<(), MapKitError> {
        let payload = crate::private::json_cstring(&tracking_mode, "MKUserTrackingMode")?;
        let mut error = std::ptr::null_mut();
        unsafe {
            ffi::mk_user_tracking_button_set_tracking_mode(
                self.map_view.as_raw(),
                payload.as_ptr(),
                animated,
                &mut error,
            );
        };
        unsafe { crate::private::unit_result(error, "failed to set MKUserTrackingButton mode") }
    }
}
