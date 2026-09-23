use core::ffi::{c_char, c_void};
use std::ptr::{self, NonNull};
use std::sync::{Mutex, MutexGuard, TryLockError};

use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::panic_safe::catch_user_panic_result;

use crate::annotation::MKUserLocation;
use crate::annotation_view::{MKAnnotationView, MKAnnotationViewDragState};
use crate::error::MapKitError;
use crate::ffi;
use crate::map_view::{MKMapView, MKUserTrackingMode};
use crate::overlay::{
    MKCircle, MKGeodesicPolyline, MKMultiPolygon, MKMultiPolyline, MKOverlay, MKPolygon,
    MKPolyline, MKTileOverlay,
};
use crate::overlay_renderer::MKOverlayRendering;
use crate::private::owned_handle;

/// Wraps `MKMapViewDelegate`.
pub trait MKMapViewDelegate: Send + 'static {
    fn map_view_region_will_change_animated(&mut self, _map_view: &MKMapView, _animated: bool) {}

    fn map_view_region_did_change_animated(&mut self, _map_view: &MKMapView, _animated: bool) {}

    fn map_view_did_change_visible_region(&mut self, _map_view: &MKMapView) {}

    fn map_view_will_start_loading_map(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_finish_loading_map(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_fail_loading_map(&mut self, _map_view: &MKMapView, _error: &MapKitError) {}

    fn map_view_will_start_rendering_map(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_finish_rendering_map(&mut self, _map_view: &MKMapView, _fully_rendered: bool) {}

    fn map_view_did_select_annotation_view(
        &mut self,
        _map_view: &MKMapView,
        _view: &MKAnnotationView,
    ) {
    }

    fn map_view_did_deselect_annotation_view(
        &mut self,
        _map_view: &MKMapView,
        _view: &MKAnnotationView,
    ) {
    }

    fn map_view_will_start_locating_user(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_stop_locating_user(&mut self, _map_view: &MKMapView) {}

    fn map_view_did_update_user_location(
        &mut self,
        _map_view: &MKMapView,
        _user_location: &MKUserLocation,
    ) {
    }

    fn map_view_did_fail_to_locate_user_with_error(
        &mut self,
        _map_view: &MKMapView,
        _error: &MapKitError,
    ) {
    }

    fn map_view_annotation_view_did_change_drag_state(
        &mut self,
        _map_view: &MKMapView,
        _view: &MKAnnotationView,
        _new_state: MKAnnotationViewDragState,
        _old_state: MKAnnotationViewDragState,
    ) {
    }

    fn map_view_did_change_user_tracking_mode(
        &mut self,
        _map_view: &MKMapView,
        _mode: MKUserTrackingMode,
        _animated: bool,
    ) {
    }

    fn map_view_renderer_for_overlay(
        &mut self,
        _map_view: &MKMapView,
        _overlay: &MKAnyOverlay,
    ) -> Option<Box<dyn MKOverlayRendering>> {
        None
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum MKAnyOverlay {
    Circle(MKCircle),
    Polyline(MKPolyline),
    GeodesicPolyline(MKGeodesicPolyline),
    Polygon(MKPolygon),
    MultiPolyline(MKMultiPolyline),
    MultiPolygon(MKMultiPolygon),
    Tile(MKTileOverlay),
}

impl MKAnyOverlay {
    pub fn as_overlay(&self) -> &dyn MKOverlay {
        match self {
            Self::Circle(overlay) => overlay,
            Self::Polyline(overlay) => overlay,
            Self::GeodesicPolyline(overlay) => overlay,
            Self::Polygon(overlay) => overlay,
            Self::MultiPolyline(overlay) => overlay,
            Self::MultiPolygon(overlay) => overlay,
            Self::Tile(overlay) => overlay,
        }
    }

    fn from_owned(kind: i32, raw: NonNull<c_void>) -> Option<Self> {
        Some(match kind {
            0 => Self::Circle(MKCircle::from_owned(raw)),
            1 => Self::Polyline(MKPolyline::from_owned(raw)),
            2 => Self::GeodesicPolyline(MKGeodesicPolyline::from_owned(raw)),
            3 => Self::Polygon(MKPolygon::from_owned(raw)),
            4 => Self::MultiPolyline(MKMultiPolyline::from_owned(raw)),
            5 => Self::MultiPolygon(MKMultiPolygon::from_owned(raw)),
            6 => Self::Tile(MKTileOverlay::from_owned(raw)),
            _ => {
                unsafe { ffi::mk_object_release(raw.as_ptr()) };
                return None;
            }
        })
    }
}

type DelegateState = Mutex<Box<dyn MKMapViewDelegate>>;

#[must_use = "dropping the registration removes the delegate"]
#[derive(Debug)]
pub struct MKMapViewDelegateRegistration {
    map_view: NonNull<c_void>,
    proxy: NonNull<c_void>,
    context: CallbackContext<DelegateState>,
}

impl Drop for MKMapViewDelegateRegistration {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe {
            ffi::mk_map_view_clear_delegate(self.map_view.as_ptr(), self.proxy.as_ptr());
            ffi::mk_object_release(self.proxy.as_ptr());
            ffi::mk_object_release(self.map_view.as_ptr());
        }
    }
}

impl MKMapView {
    pub fn set_delegate<D: MKMapViewDelegate>(
        &self,
        delegate: D,
    ) -> Result<MKMapViewDelegateRegistration, MapKitError> {
        let delegate: Box<dyn MKMapViewDelegate> = Box::new(delegate);
        let context = CallbackContext::new(Mutex::new(delegate));
        let map_view = NonNull::new(unsafe { ffi::mk_object_retain(self.as_raw()) })
            .ok_or_else(|| MapKitError::OperationFailed("failed to retain MKMapView".into()))?;
        let mut error = ptr::null_mut();
        let proxy = unsafe {
            ffi::mk_map_view_set_delegate(
                map_view.as_ptr(),
                context.as_ptr(),
                CallbackContext::<DelegateState>::RETAIN,
                CallbackContext::<DelegateState>::RELEASE,
                event_trampoline,
                renderer_trampoline,
                &raw mut error,
            )
        };
        match owned_handle(proxy, error, "failed to set the MKMapView delegate") {
            Ok(proxy) => Ok(MKMapViewDelegateRegistration {
                map_view,
                proxy,
                context,
            }),
            Err(error) => {
                unsafe { ffi::mk_object_release(map_view.as_ptr()) };
                Err(error)
            }
        }
    }
}

const REGION_WILL_CHANGE: i32 = 0;
const REGION_DID_CHANGE: i32 = 1;
const DID_CHANGE_VISIBLE_REGION: i32 = 2;
const WILL_START_LOADING_MAP: i32 = 3;
const DID_FINISH_LOADING_MAP: i32 = 4;
const DID_FAIL_LOADING_MAP: i32 = 5;
const WILL_START_RENDERING_MAP: i32 = 6;
const DID_FINISH_RENDERING_MAP: i32 = 7;
const DID_SELECT_ANNOTATION_VIEW: i32 = 8;
const DID_DESELECT_ANNOTATION_VIEW: i32 = 9;
const WILL_START_LOCATING_USER: i32 = 10;
const DID_STOP_LOCATING_USER: i32 = 11;
const DID_UPDATE_USER_LOCATION: i32 = 12;
const DID_FAIL_TO_LOCATE_USER: i32 = 13;
const ANNOTATION_VIEW_DRAG_STATE: i32 = 14;
const DID_CHANGE_USER_TRACKING_MODE: i32 = 15;

const fn drag_state(code: i64) -> Option<MKAnnotationViewDragState> {
    match code {
        0 => Some(MKAnnotationViewDragState::None),
        1 => Some(MKAnnotationViewDragState::Starting),
        2 => Some(MKAnnotationViewDragState::Dragging),
        3 => Some(MKAnnotationViewDragState::Canceling),
        4 => Some(MKAnnotationViewDragState::Ending),
        _ => None,
    }
}

const fn tracking_mode(code: i64) -> Option<MKUserTrackingMode> {
    match code {
        0 => Some(MKUserTrackingMode::None),
        1 => Some(MKUserTrackingMode::Follow),
        2 => Some(MKUserTrackingMode::FollowWithHeading),
        _ => None,
    }
}

fn lock(state: &DelegateState) -> Option<MutexGuard<'_, Box<dyn MKMapViewDelegate>>> {
    match state.try_lock() {
        Ok(guard) => Some(guard),
        Err(TryLockError::Poisoned(poisoned)) => Some(poisoned.into_inner()),
        Err(TryLockError::WouldBlock) => None,
    }
}

fn dispatch(
    delegate: &mut dyn MKMapViewDelegate,
    event: i32,
    map_view: &MKMapView,
    view: Option<&MKAnnotationView>,
    user_location: Option<&MKUserLocation>,
    (first, second): (i64, i64),
    error: Option<&MapKitError>,
) {
    match (event, view, user_location, error) {
        (REGION_WILL_CHANGE, ..) => {
            delegate.map_view_region_will_change_animated(map_view, first != 0);
        }
        (REGION_DID_CHANGE, ..) => {
            delegate.map_view_region_did_change_animated(map_view, first != 0);
        }
        (DID_CHANGE_VISIBLE_REGION, ..) => delegate.map_view_did_change_visible_region(map_view),
        (WILL_START_LOADING_MAP, ..) => delegate.map_view_will_start_loading_map(map_view),
        (DID_FINISH_LOADING_MAP, ..) => delegate.map_view_did_finish_loading_map(map_view),
        (DID_FAIL_LOADING_MAP, _, _, Some(error)) => {
            delegate.map_view_did_fail_loading_map(map_view, error);
        }
        (WILL_START_RENDERING_MAP, ..) => delegate.map_view_will_start_rendering_map(map_view),
        (DID_FINISH_RENDERING_MAP, ..) => {
            delegate.map_view_did_finish_rendering_map(map_view, first != 0);
        }
        (DID_SELECT_ANNOTATION_VIEW, Some(view), ..) => {
            delegate.map_view_did_select_annotation_view(map_view, view);
        }
        (DID_DESELECT_ANNOTATION_VIEW, Some(view), ..) => {
            delegate.map_view_did_deselect_annotation_view(map_view, view);
        }
        (WILL_START_LOCATING_USER, ..) => delegate.map_view_will_start_locating_user(map_view),
        (DID_STOP_LOCATING_USER, ..) => delegate.map_view_did_stop_locating_user(map_view),
        (DID_UPDATE_USER_LOCATION, _, Some(user_location), _) => {
            delegate.map_view_did_update_user_location(map_view, user_location);
        }
        (DID_FAIL_TO_LOCATE_USER, _, _, Some(error)) => {
            delegate.map_view_did_fail_to_locate_user_with_error(map_view, error);
        }
        (ANNOTATION_VIEW_DRAG_STATE, Some(view), ..) => {
            if let (Some(new_state), Some(old_state)) = (drag_state(first), drag_state(second)) {
                delegate.map_view_annotation_view_did_change_drag_state(
                    map_view, view, new_state, old_state,
                );
            }
        }
        (DID_CHANGE_USER_TRACKING_MODE, ..) => {
            if let Some(mode) = tracking_mode(first) {
                delegate.map_view_did_change_user_tracking_mode(map_view, mode, second != 0);
            }
        }
        _ => {}
    }
}

unsafe extern "C" fn event_trampoline(
    context: *mut c_void,
    event: i32,
    map_view: *mut c_void,
    object: *mut c_void,
    first: i64,
    second: i64,
    error: *mut c_char,
) {
    let _ = catch_user_panic_result("MKMapViewDelegate event", || {
        let error = (!error.is_null())
            .then(|| unsafe { MapKitError::from_error_ptr(error, "MapKit reported an error") });
        let object = NonNull::new(object);
        let takes_view = matches!(
            event,
            DID_SELECT_ANNOTATION_VIEW | DID_DESELECT_ANNOTATION_VIEW | ANNOTATION_VIEW_DRAG_STATE
        );
        let view = object
            .filter(|_| takes_view)
            .map(MKAnnotationView::from_owned);
        let user_location = object
            .filter(|_| event == DID_UPDATE_USER_LOCATION)
            .and_then(|raw| MKUserLocation::from_owned_raw(raw.as_ptr()).ok());
        if let (Some(raw), None, None) = (object, &view, &user_location) {
            unsafe { ffi::mk_object_release(raw.as_ptr()) };
        }
        let Some(map_view) = NonNull::new(map_view).map(MKMapView::from_owned) else {
            return;
        };
        unsafe {
            CallbackContext::<DelegateState>::with(context, "MKMapViewDelegate event", |state| {
                if let Some(mut delegate) = lock(state) {
                    dispatch(
                        &mut **delegate,
                        event,
                        &map_view,
                        view.as_ref(),
                        user_location.as_ref(),
                        (first, second),
                        error.as_ref(),
                    );
                }
            })
        };
    });
}

unsafe extern "C" fn renderer_trampoline(
    context: *mut c_void,
    map_view: *mut c_void,
    overlay: *mut c_void,
    kind: i32,
) -> *mut c_void {
    catch_user_panic_result("MKMapViewDelegate renderer", || {
        let overlay = NonNull::new(overlay).and_then(|raw| MKAnyOverlay::from_owned(kind, raw));
        let map_view = NonNull::new(map_view).map(MKMapView::from_owned);
        let (Some(map_view), Some(overlay)) = (map_view, overlay) else {
            return ptr::null_mut();
        };
        unsafe {
            CallbackContext::<DelegateState>::with(context, "MKMapViewDelegate renderer", |state| {
                let renderer = lock(state)?.map_view_renderer_for_overlay(&map_view, &overlay)?;
                Some(ffi::mk_object_retain(renderer.as_raw_renderer()))
            })
        }
        .flatten()
        .unwrap_or(ptr::null_mut())
    })
    .unwrap_or(ptr::null_mut())
}
