use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKScreenSize};
use crate::map_item::MKMapItem;
use crate::point_of_interest::MKPointOfInterestFilter;
use crate::private::{json_cstring, owned_handle, parse_json_ptr};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKLookAroundSceneRequestState {
    coordinate: MKCoordinate,
    has_map_item: bool,
    cancelled: bool,
    loading: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKLookAroundSnapshotState {
    image_byte_len: usize,
    size: MKScreenSize,
}

/// Wraps `MKLookAroundSnapshotOptions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKLookAroundSnapshotOptions {
    /// Wraps `MKLookAroundSnapshotOptions.size`.
    pub size: MKScreenSize,
    /// Wraps `MKLookAroundSnapshotOptions.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
}

impl MKLookAroundSnapshotOptions {
    /// Creates a wrapper for `MKLookAroundSnapshotOptions`.
    pub const fn new(size: MKScreenSize) -> Self {
        Self {
            size,
            point_of_interest_filter: None,
        }
    }

    /// Wraps `MKLookAroundSnapshotOptions.pointOfInterestFilter`.
    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }
}

/// Wraps `MKLookAroundSceneRequest`.
#[derive(Debug)]
pub struct MKLookAroundSceneRequest {
    raw: NonNull<c_void>,
}

impl MKLookAroundSceneRequest {
    /// Creates a wrapper for `MKLookAroundSceneRequest`.
    pub fn new(coordinate: MKCoordinate) -> Result<Self, MapKitError> {
        let coordinate = json_cstring(&coordinate, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_look_around_scene_request_new_coordinate_json(coordinate.as_ptr(), &mut error)
        };
        let raw = owned_handle(raw, error, "failed to create MKLookAroundSceneRequest")?;
        Ok(Self { raw })
    }

    /// Wraps `MKLookAroundSceneRequest.fromMapItem`.
    pub fn from_map_item(map_item: &MKMapItem) -> Result<Self, MapKitError> {
        let map_item = json_cstring(map_item, "MKMapItem")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_look_around_scene_request_new_map_item_json(map_item.as_ptr(), &mut error)
        };
        let raw = owned_handle(
            raw,
            error,
            "failed to create MKLookAroundSceneRequest from MKMapItem",
        )?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKLookAroundSceneRequestState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_look_around_scene_request_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKLookAroundSceneRequest state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKLookAroundSceneRequest state") }
        }
    }

    /// Wraps `MKLookAroundSceneRequest.coordinate`.
    pub fn coordinate(&self) -> Result<MKCoordinate, MapKitError> {
        Ok(self.state()?.coordinate)
    }

    /// Wraps `MKLookAroundSceneRequest.hasMapItem`.
    pub fn has_map_item(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.has_map_item)
    }

    /// Wraps `MKLookAroundSceneRequest.isCancelled`.
    pub fn is_cancelled(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.cancelled)
    }

    /// Wraps `MKLookAroundSceneRequest.isLoading`.
    pub fn is_loading(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.loading)
    }

    /// Wraps `MKLookAroundSceneRequest.scene`.
    pub fn scene(&self) -> Result<MKLookAroundScene, MapKitError> {
        let mut error = ptr::null_mut();
        let raw =
            unsafe { ffi::mk_look_around_scene_request_get_scene(self.raw.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "MKLookAroundSceneRequest getScene failed")?;
        Ok(MKLookAroundScene { raw })
    }

    /// Wraps `MKLookAroundSceneRequest.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_look_around_scene_request_cancel(self.raw.as_ptr()) };
    }
}

impl Drop for MKLookAroundSceneRequest {
    fn drop(&mut self) {
        unsafe { ffi::mk_look_around_scene_request_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKLookAroundScene`.
#[derive(Debug)]
pub struct MKLookAroundScene {
    raw: NonNull<c_void>,
}

impl MKLookAroundScene {
    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

impl Drop for MKLookAroundScene {
    fn drop(&mut self) {
        unsafe { ffi::mk_look_around_scene_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKLookAroundSnapshotter`.
#[derive(Debug)]
pub struct MKLookAroundSnapshotter {
    raw: NonNull<c_void>,
}

impl MKLookAroundSnapshotter {
    /// Creates a wrapper for `MKLookAroundSnapshotter`.
    pub fn new(
        scene: &MKLookAroundScene,
        options: &MKLookAroundSnapshotOptions,
    ) -> Result<Self, MapKitError> {
        let options = json_cstring(options, "MKLookAroundSnapshotOptions")?;
        let mut error = ptr::null_mut();
        let raw = unsafe {
            ffi::mk_look_around_snapshotter_new(scene.as_raw(), options.as_ptr(), &mut error)
        };
        let raw = owned_handle(raw, error, "failed to create MKLookAroundSnapshotter")?;
        Ok(Self { raw })
    }

    /// Wraps `MKLookAroundSnapshotter.snapshot`.
    pub fn snapshot(&self) -> Result<MKLookAroundSnapshot, MapKitError> {
        let mut error = ptr::null_mut();
        let raw =
            unsafe { ffi::mk_look_around_snapshotter_get_snapshot(self.raw.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "MKLookAroundSnapshotter getSnapshot failed")?;
        Ok(MKLookAroundSnapshot { raw })
    }

    /// Wraps `MKLookAroundSnapshotter.isLoading`.
    pub fn is_loading(&self) -> bool {
        unsafe { ffi::mk_look_around_snapshotter_is_loading(self.raw.as_ptr()) }
    }

    /// Wraps `MKLookAroundSnapshotter.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_look_around_snapshotter_cancel(self.raw.as_ptr()) };
    }
}

impl Drop for MKLookAroundSnapshotter {
    fn drop(&mut self) {
        unsafe { ffi::mk_look_around_snapshotter_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKLookAroundSnapshot`.
#[derive(Debug)]
pub struct MKLookAroundSnapshot {
    raw: NonNull<c_void>,
}

impl MKLookAroundSnapshot {
    fn state(&self) -> Result<MKLookAroundSnapshotState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_look_around_snapshot_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKLookAroundSnapshot state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKLookAroundSnapshot state") }
        }
    }

    /// Wraps `MKLookAroundSnapshot.imageByteLen`.
    pub fn image_byte_len(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.image_byte_len)
    }

    /// Wraps `MKLookAroundSnapshot.size`.
    pub fn size(&self) -> Result<MKScreenSize, MapKitError> {
        Ok(self.state()?.size)
    }
}

impl Drop for MKLookAroundSnapshot {
    fn drop(&mut self) {
        unsafe { ffi::mk_look_around_snapshot_release(self.raw.as_ptr()) };
    }
}
