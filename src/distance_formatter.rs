use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::private::{cstring_from_str, owned_handle, take_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKDistanceFormatterUnits {
    #[default]
    Default,
    Metric,
    Imperial,
    ImperialWithYards,
}

impl MKDistanceFormatterUnits {
    pub(crate) const fn as_raw(self) -> u64 {
        match self {
            Self::Default => 0,
            Self::Metric => 1,
            Self::Imperial => 2,
            Self::ImperialWithYards => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MKDistanceFormatterUnitStyle {
    #[default]
    Default,
    Abbreviated,
    Full,
}

impl MKDistanceFormatterUnitStyle {
    pub(crate) const fn as_raw(self) -> u64 {
        match self {
            Self::Default => 0,
            Self::Abbreviated => 1,
            Self::Full => 2,
        }
    }
}

#[derive(Debug)]
pub struct MKDistanceFormatter {
    raw: NonNull<c_void>,
}

impl MKDistanceFormatter {
    pub fn new() -> Result<Self, MapKitError> {
        let raw = unsafe { ffi::mk_distance_formatter_new() };
        let raw = owned_handle(raw, ptr::null_mut(), "failed to create MKDistanceFormatter")?;
        Ok(Self { raw })
    }

    pub fn with_units(mut self, units: MKDistanceFormatterUnits) -> Self {
        self.set_units(units);
        self
    }

    pub fn with_unit_style(mut self, unit_style: MKDistanceFormatterUnitStyle) -> Self {
        self.set_unit_style(unit_style);
        self
    }

    pub fn set_units(&mut self, units: MKDistanceFormatterUnits) {
        unsafe { ffi::mk_distance_formatter_set_units(self.raw.as_ptr(), units.as_raw()) };
    }

    pub fn set_unit_style(&mut self, unit_style: MKDistanceFormatterUnitStyle) {
        unsafe { ffi::mk_distance_formatter_set_unit_style(self.raw.as_ptr(), unit_style.as_raw()) };
    }

    pub fn string_from_distance(&self, distance: f64) -> Result<String, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_distance_formatter_string_from_distance(
                self.raw.as_ptr(),
                distance,
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKDistanceFormatter string failed")
            })
        } else {
            unsafe { take_string(payload) }.ok_or_else(|| {
                MapKitError::OperationFailed(
                    "missing distance formatter string payload".to_owned(),
                )
            })
        }
    }

    pub fn distance_from_string(
        &self,
        distance_string: &str,
    ) -> Result<f64, MapKitError> {
        let distance_string = cstring_from_str(distance_string, "distance string")?;
        let mut error = ptr::null_mut();
        let distance = unsafe {
            ffi::mk_distance_formatter_distance_from_string(
                self.raw.as_ptr(),
                distance_string.as_ptr(),
                &mut error,
            )
        };
        if error.is_null() {
            Ok(distance)
        } else {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKDistanceFormatter distance failed")
            })
        }
    }
}

impl Drop for MKDistanceFormatter {
    fn drop(&mut self) {
        unsafe { ffi::mk_distance_formatter_release(self.raw.as_ptr()) };
    }
}
