#![allow(clippy::missing_errors_doc)]

use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::MapKitError;
use crate::ffi;

/// Wraps `cstring_from_str`.
pub fn cstring_from_str(value: &str, context: &str) -> Result<CString, MapKitError> {
    CString::new(value).map_err(|error| {
        MapKitError::InvalidArgument(format!("{context} contains NUL byte: {error}"))
    })
}

/// Wraps `json_cstring`.
pub fn json_cstring<T: Serialize + ?Sized>(
    value: &T,
    context: &str,
) -> Result<CString, MapKitError> {
    let json = serde_json::to_string(value).map_err(|error| {
        MapKitError::InvalidArgument(format!("failed to encode {context} as JSON: {error}"))
    })?;
    cstring_from_str(&json, context)
}

/// Wraps `owned_handle`.
pub fn owned_handle(
    raw: *mut c_void,
    error: *mut c_char,
    fallback: &str,
) -> Result<NonNull<c_void>, MapKitError> {
    NonNull::new(raw).ok_or_else(|| unsafe { MapKitError::from_error_ptr(error, fallback) })
}

/// Take ownership of a C string produced by a Swift bridge thunk.
///
/// # Safety
///
/// `ptr` must be either null or a valid, non-aliased, nul-terminated C string
/// that was allocated by the Swift bridge (i.e. returned via
/// `mk_string_free`-compatible allocation).  The string is freed after this
/// call; the caller must not use `ptr` again.
pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::mk_string_free(ptr);
    Some(string)
}

/// Parse a JSON-encoded value from a Swift bridge C string and free it.
///
/// # Safety
///
/// Same contract as [`take_string`]: `ptr` must be either null or a valid,
/// non-aliased, nul-terminated C string allocated by the Swift bridge.  The
/// string is freed after this call; the caller must not use `ptr` again.
pub unsafe fn parse_json_ptr<T: DeserializeOwned>(
    ptr: *mut c_char,
    context: &str,
) -> Result<T, MapKitError> {
    let json = take_string(ptr).ok_or_else(|| {
        MapKitError::OperationFailed(format!("missing JSON payload for {context}"))
    })?;

    serde_json::from_str(&json).map_err(|error| {
        MapKitError::OperationFailed(format!(
            "failed to parse {context} JSON: {error}; payload={json}"
        ))
    })
}

/// Convert a nullable C string error pointer into a `Result`.
///
/// Returns `Ok(())` when `error_ptr` is null (success) or
/// `Err(MapKitError)` when it is non-null.
///
/// # Safety
///
/// `error_ptr` must be either null or a valid, non-aliased, nul-terminated C
/// string allocated by the Swift bridge.  The string is freed after this call;
/// the caller must not use `error_ptr` again.
pub unsafe fn unit_result(error_ptr: *mut c_char, fallback: &str) -> Result<(), MapKitError> {
    if error_ptr.is_null() {
        Ok(())
    } else {
        Err(MapKitError::from_error_ptr(error_ptr, fallback))
    }
}
