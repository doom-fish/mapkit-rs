#![allow(clippy::missing_errors_doc)]

use core::ffi::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::ptr::NonNull;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::MapKitError;
use crate::ffi;

pub fn cstring_from_str(value: &str, context: &str) -> Result<CString, MapKitError> {
    CString::new(value).map_err(|error| {
        MapKitError::InvalidArgument(format!("{context} contains NUL byte: {error}"))
    })
}

pub fn json_cstring<T: Serialize + ?Sized>(
    value: &T,
    context: &str,
) -> Result<CString, MapKitError> {
    let json = serde_json::to_string(value).map_err(|error| {
        MapKitError::InvalidArgument(format!("failed to encode {context} as JSON: {error}"))
    })?;
    cstring_from_str(&json, context)
}

pub fn owned_handle(
    raw: *mut c_void,
    error: *mut c_char,
    fallback: &str,
) -> Result<NonNull<c_void>, MapKitError> {
    NonNull::new(raw).ok_or_else(|| unsafe { MapKitError::from_error_ptr(error, fallback) })
}

pub unsafe fn take_string(ptr: *mut c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let string = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    ffi::mk_string_free(ptr);
    Some(string)
}

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

pub unsafe fn unit_result(
    error_ptr: *mut c_char,
    fallback: &str,
) -> Result<(), MapKitError> {
    if error_ptr.is_null() {
        Ok(())
    } else {
        Err(MapKitError::from_error_ptr(error_ptr, fallback))
    }
}
