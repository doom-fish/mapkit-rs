use core::fmt;
use std::ffi::CStr;

use serde::{Deserialize, Serialize};

use crate::ffi;

/// Wraps `MKErrorCode`.
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
    /// Wraps `MKErrorCode.fromRaw`.
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

    /// Wraps `MKErrorCode.asRaw`.
    pub const fn as_raw(self) -> i64 {
        self as i64
    }
}

/// Wraps `MKErrorDomain`.
pub const fn mk_error_domain() -> &'static str {
    "MKErrorDomain"
}

/// Wraps `NSErrorInfo`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NSErrorInfo {
    /// Wraps `NSErrorInfo.domain`.
    pub domain: String,
    /// Wraps `NSErrorInfo.code`.
    pub code: i64,
    /// Wraps `NSErrorInfo.message`.
    pub message: String,
}

impl NSErrorInfo {
    /// Wraps `NSErrorInfo.isMapkitDomain`.
    pub fn is_mapkit_domain(&self) -> bool {
        self.domain == mk_error_domain()
    }

    /// Wraps `NSErrorInfo.mapkitErrorCode`.
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

/// Wraps `MapKitError`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MapKitError {
    InvalidArgument(String),
    Framework(NSErrorInfo),
    OperationFailed(String),
    MainThreadRequired(String),
}

const BRIDGE_ERROR_DOMAIN: &str = "mapkit-rs";
const MAIN_THREAD_REQUIRED_CODE: i64 = -2;

impl fmt::Display for MapKitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Framework(error) => write!(f, "MapKit.framework error: {error}"),
            Self::OperationFailed(message) => write!(f, "mapkit operation failed: {message}"),
            Self::MainThreadRequired(message) => write!(f, "main thread required: {message}"),
        }
    }
}

impl std::error::Error for MapKitError {}

impl MapKitError {
    /// Wraps `MapKitError.mapkitErrorCode`.
    pub fn mapkit_error_code(&self) -> Option<MKErrorCode> {
        match self {
            Self::Framework(error) => error.mapkit_error_code(),
            Self::InvalidArgument(_) | Self::OperationFailed(_) | Self::MainThreadRequired(_) => {
                None
            }
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
    pub(crate) unsafe fn from_error_ptr(error_ptr: *mut core::ffi::c_char, fallback: &str) -> Self {
        if error_ptr.is_null() {
            return Self::OperationFailed(fallback.to_owned());
        }

        let message = CStr::from_ptr(error_ptr).to_string_lossy().into_owned();
        ffi::mk_string_free(error_ptr);
        Self::from_bridge_message(message)
    }

    fn from_bridge_message(message: String) -> Self {
        match serde_json::from_str::<NSErrorInfo>(&message) {
            Ok(info)
                if info.domain == BRIDGE_ERROR_DOMAIN && info.code == MAIN_THREAD_REQUIRED_CODE =>
            {
                Self::MainThreadRequired(info.message)
            }
            Ok(info) => Self::Framework(info),
            Err(_) => Self::OperationFailed(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn framework_info() -> NSErrorInfo {
        NSErrorInfo {
            domain: mk_error_domain().to_owned(),
            code: MKErrorCode::DirectionsNotFound.as_raw(),
            message: "directions missing".to_owned(),
        }
    }

    #[test]
    fn error_code_round_trips_through_raw_values() {
        for error_code in [
            MKErrorCode::Unknown,
            MKErrorCode::ServerFailure,
            MKErrorCode::LoadingThrottled,
            MKErrorCode::PlacemarkNotFound,
            MKErrorCode::DirectionsNotFound,
            MKErrorCode::DecodingFailed,
        ] {
            assert_eq!(MKErrorCode::from_raw(error_code.as_raw()), Some(error_code));
        }
    }

    #[test]
    fn error_code_rejects_unknown_raw_values() {
        assert_eq!(MKErrorCode::from_raw(0), None);
        assert_eq!(MKErrorCode::from_raw(7), None);
    }

    #[test]
    fn ns_error_info_detects_mapkit_domain() {
        assert!(framework_info().is_mapkit_domain());
    }

    #[test]
    fn ns_error_info_maps_known_mapkit_codes() {
        assert_eq!(
            framework_info().mapkit_error_code(),
            Some(MKErrorCode::DirectionsNotFound)
        );
    }

    #[test]
    fn ns_error_info_ignores_foreign_domains() {
        let info = NSErrorInfo {
            domain: "NSCocoaErrorDomain".to_owned(),
            code: MKErrorCode::DirectionsNotFound.as_raw(),
            message: "not mapkit".to_owned(),
        };

        assert!(!info.is_mapkit_domain());
        assert_eq!(info.mapkit_error_code(), None);
    }

    #[test]
    fn ns_error_info_display_includes_message_code_and_domain() {
        assert_eq!(
            framework_info().to_string(),
            "directions missing (5) [MKErrorDomain]"
        );
    }

    #[test]
    fn mapkit_error_display_formats_each_variant() {
        assert_eq!(
            MapKitError::InvalidArgument("bad input".to_owned()).to_string(),
            "invalid argument: bad input"
        );
        assert_eq!(
            MapKitError::Framework(framework_info()).to_string(),
            "MapKit.framework error: directions missing (5) [MKErrorDomain]"
        );
        assert_eq!(
            MapKitError::OperationFailed("bridge failed".to_owned()).to_string(),
            "mapkit operation failed: bridge failed"
        );
        assert_eq!(
            MapKitError::MainThreadRequired("off main".to_owned()).to_string(),
            "main thread required: off main"
        );
    }

    #[test]
    fn bridge_main_thread_errors_map_to_main_thread_required() {
        let message = "MKLocalSearchCompleter must be created and used on the main thread";
        assert_eq!(
            MapKitError::from_bridge_message(format!(
                r#"{{"domain":"mapkit-rs","code":-2,"message":"{message}"}}"#
            )),
            MapKitError::MainThreadRequired(message.to_owned())
        );
        assert!(matches!(
            MapKitError::from_bridge_message(
                r#"{"domain":"mapkit-rs","code":-1,"message":"other"}"#.to_owned()
            ),
            MapKitError::Framework(_)
        ));
        assert!(matches!(
            MapKitError::from_bridge_message(
                r#"{"domain":"NSCocoaErrorDomain","code":-2,"message":"foreign"}"#.to_owned()
            ),
            MapKitError::Framework(_)
        ));
        assert_eq!(
            MapKitError::from_bridge_message("plain text".to_owned()),
            MapKitError::OperationFailed("plain text".to_owned())
        );
    }

    #[test]
    fn framework_variant_exposes_mapkit_error_code() {
        let error = MapKitError::Framework(framework_info());

        assert_eq!(error.mapkit_error_code(), Some(MKErrorCode::DirectionsNotFound));
    }

    #[test]
    fn non_framework_variants_do_not_expose_mapkit_error_codes() {
        assert_eq!(
            MapKitError::InvalidArgument("bad input".to_owned()).mapkit_error_code(),
            None
        );
        assert_eq!(
            MapKitError::OperationFailed("bridge failed".to_owned()).mapkit_error_code(),
            None
        );
        assert_eq!(
            MapKitError::MainThreadRequired("off main".to_owned()).mapkit_error_code(),
            None
        );
    }
}
