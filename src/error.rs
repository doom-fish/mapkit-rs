use core::fmt;
use std::ffi::CStr;

use serde::{Deserialize, Serialize};

use crate::ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKErrorCode {
    Unknown = 1,
    ServerFailure = 2,
    LoadingThrottled = 3,
    PlacemarkNotFound = 4,
    DirectionsNotFound = 5,
    DecodingFailed = 6,
}

impl MKErrorCode {
    pub const fn from_raw(value: i64) -> Option<Self> {
        match value {
            1 => Some(Self::Unknown),
            2 => Some(Self::ServerFailure),
            3 => Some(Self::LoadingThrottled),
            4 => Some(Self::PlacemarkNotFound),
            5 => Some(Self::DirectionsNotFound),
            6 => Some(Self::DecodingFailed),
            _ => None,
        }
    }

    pub const fn as_raw(self) -> i64 {
        self as i64
    }
}

pub const fn mk_error_domain() -> &'static str {
    "MKErrorDomain"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NSErrorInfo {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

impl NSErrorInfo {
    pub fn is_mapkit_domain(&self) -> bool {
        self.domain == mk_error_domain()
    }

    pub fn mapkit_error_code(&self) -> Option<MKErrorCode> {
        self.is_mapkit_domain()
            .then(|| MKErrorCode::from_raw(self.code))
            .flatten()
    }
}

impl fmt::Display for NSErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) [{}]", self.message, self.code, self.domain)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MapKitError {
    InvalidArgument(String),
    Framework(NSErrorInfo),
    OperationFailed(String),
}

impl fmt::Display for MapKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Framework(error) => write!(f, "MapKit.framework error: {error}"),
            Self::OperationFailed(message) => write!(f, "mapkit operation failed: {message}"),
        }
    }
}

impl std::error::Error for MapKitError {}

impl MapKitError {
    pub fn mapkit_error_code(&self) -> Option<MKErrorCode> {
        match self {
            Self::Framework(error) => error.mapkit_error_code(),
            Self::InvalidArgument(_) | Self::OperationFailed(_) => None,
        }
    }

    /// Build a [`MapKitError`] from a nullable C string error pointer produced
    /// by a Swift bridge thunk and free the string.
    ///
    /// # Safety
    ///
    /// `error_ptr` must be either null or a valid, non-aliased, nul-terminated
    /// C string allocated by the Swift bridge.  The string is freed after this
    /// call; the caller must not use `error_ptr` again.
    pub(crate) unsafe fn from_error_ptr(
        error_ptr: *mut core::ffi::c_char,
        fallback: &str,
    ) -> Self {
        if error_ptr.is_null() {
            return Self::OperationFailed(fallback.to_owned());
        }

        let message = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
        ffi::mk_string_free(error_ptr);

        if let Ok(payload) = serde_json::from_str::<NSErrorInfo>(&message) {
            Self::Framework(payload)
        } else {
            Self::OperationFailed(message)
        }
    }
}
