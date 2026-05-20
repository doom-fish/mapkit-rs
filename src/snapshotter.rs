use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::{MKCoordinate, MKCoordinateRegion, MKMapRect, MKScreenPoint, MKScreenSize};
use crate::map_view::MKMapType;
use crate::point_of_interest::MKPointOfInterestFilter;
use crate::private::{json_cstring, owned_handle, parse_json_ptr};

/// Wraps `MKMapSnapshotOptions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MKMapSnapshotOptions {
    /// Wraps `MKMapSnapshotOptions.region`.
    pub region: Option<MKCoordinateRegion>,
    /// Wraps `MKMapSnapshotOptions.mapRect`.
    pub map_rect: Option<MKMapRect>,
    /// Wraps `MKMapSnapshotOptions.mapType`.
    pub map_type: Option<MKMapType>,
    /// Wraps `MKMapSnapshotOptions.pointOfInterestFilter`.
    pub point_of_interest_filter: Option<MKPointOfInterestFilter>,
    /// Wraps `MKMapSnapshotOptions.showsPointsOfInterest`.
    #[serde(default)]
    pub shows_points_of_interest: bool,
    /// Wraps `MKMapSnapshotOptions.showsBuildings`.
    #[serde(default)]
    pub shows_buildings: bool,
    /// Wraps `MKMapSnapshotOptions.size`.
    pub size: MKScreenSize,
}

impl MKMapSnapshotOptions {
    /// Creates a wrapper for `MKMapSnapshotOptions`.
    pub const fn new(size: MKScreenSize) -> Self {
        Self {
            region: None,
            map_rect: None,
            map_type: None,
            point_of_interest_filter: None,
            shows_points_of_interest: false,
            shows_buildings: false,
            size,
        }
    }

    /// Wraps `MKMapSnapshotOptions.region`.
    pub fn with_region(mut self, region: MKCoordinateRegion) -> Self {
        self.region = Some(region);
        self
    }

    /// Wraps `MKMapSnapshotOptions.mapRect`.
    pub fn with_map_rect(mut self, map_rect: MKMapRect) -> Self {
        self.map_rect = Some(map_rect);
        self
    }

    /// Wraps `MKMapSnapshotOptions.mapType`.
    pub fn with_map_type(mut self, map_type: MKMapType) -> Self {
        self.map_type = Some(map_type);
        self
    }

    /// Wraps `MKMapSnapshotOptions.pointOfInterestFilter`.
    pub fn with_point_of_interest_filter(
        mut self,
        point_of_interest_filter: MKPointOfInterestFilter,
    ) -> Self {
        self.point_of_interest_filter = Some(point_of_interest_filter);
        self
    }

    /// Wraps `MKMapSnapshotOptions.showsPointsOfInterest`.
    pub fn with_shows_points_of_interest(mut self, shows_points_of_interest: bool) -> Self {
        self.shows_points_of_interest = shows_points_of_interest;
        self
    }

    /// Wraps `MKMapSnapshotOptions.showsBuildings`.
    pub fn with_shows_buildings(mut self, shows_buildings: bool) -> Self {
        self.shows_buildings = shows_buildings;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKMapSnapshotState {
    image_byte_len: usize,
    size: MKScreenSize,
}

/// Wraps `MKMapSnapshotter`.
#[derive(Debug)]
pub struct MKMapSnapshotter {
    raw: NonNull<c_void>,
}

impl MKMapSnapshotter {
    /// Creates a wrapper for `MKMapSnapshotter`.
    pub fn new(options: &MKMapSnapshotOptions) -> Result<Self, MapKitError> {
        let options = json_cstring(options, "MKMapSnapshotOptions")?;
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_map_snapshotter_new(options.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKMapSnapshotter")?;
        Ok(Self { raw })
    }

    /// Wraps `MKMapSnapshotter.start`.
    pub fn start(&self) -> Result<MKMapSnapshot, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_map_snapshotter_start(self.raw.as_ptr(), &mut error) };
        let raw = owned_handle(raw, error, "MKMapSnapshotter start failed")?;
        Ok(MKMapSnapshot { raw })
    }

    /// Wraps `MKMapSnapshotter.isLoading`.
    pub fn is_loading(&self) -> bool {
        unsafe { ffi::mk_map_snapshotter_is_loading(self.raw.as_ptr()) }
    }

    /// Wraps `MKMapSnapshotter.cancel`.
    pub fn cancel(&self) {
        unsafe { ffi::mk_map_snapshotter_cancel(self.raw.as_ptr()) };
    }

    #[cfg(feature = "async")]
    pub(crate) fn into_raw(self) -> *mut c_void {
        let raw = self.raw.as_ptr();
        std::mem::forget(self);
        raw
    }
}

impl Drop for MKMapSnapshotter {
    fn drop(&mut self) {
        unsafe { ffi::mk_map_snapshotter_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKMapSnapshot`.
#[derive(Debug)]
pub struct MKMapSnapshot {
    raw: NonNull<c_void>,
}

impl MKMapSnapshot {
    /// Wrap a retained `MKMapSnapshot` handle produced by the Swift bridge.
    ///
    /// # Safety
    ///
    /// `ptr` must be either null or a valid, retained `MKMapSnapshot` handle.
    /// Ownership is transferred to the returned `MKMapSnapshot`, which will
    /// release it on drop.
    #[cfg(feature = "async")]
    pub(crate) unsafe fn from_raw_ptr(ptr: *mut c_void) -> Option<Self> {
        NonNull::new(ptr).map(|raw| Self { raw })
    }

    fn state(&self) -> Result<MKMapSnapshotState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_map_snapshot_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe { MapKitError::from_error_ptr(error, "failed to read MKMapSnapshot state") })
        } else {
            unsafe { parse_json_ptr(payload, "MKMapSnapshot state") }
        }
    }

    /// Wraps `MKMapSnapshot.imageByteLen`.
    pub fn image_byte_len(&self) -> Result<usize, MapKitError> {
        Ok(self.state()?.image_byte_len)
    }

    /// Wraps `MKMapSnapshot.size`.
    pub fn size(&self) -> Result<MKScreenSize, MapKitError> {
        Ok(self.state()?.size)
    }

    /// Wraps `MKMapSnapshot.pointForCoordinate`.
    pub fn point_for_coordinate(
        &self,
        coordinate: MKCoordinate,
    ) -> Result<MKScreenPoint, MapKitError> {
        let coordinate = json_cstring(&coordinate, "MKCoordinate")?;
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_map_snapshot_point_for_coordinate_json(
                self.raw.as_ptr(),
                coordinate.as_ptr(),
                &mut error,
            )
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKMapSnapshot pointForCoordinate failed")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKScreenPoint") }
        }
    }
}

impl Drop for MKMapSnapshot {
    fn drop(&mut self) {
        unsafe { ffi::mk_map_snapshot_release(self.raw.as_ptr()) };
    }
}
