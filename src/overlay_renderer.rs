use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKMapRect;
use crate::overlay::{MKCircle, MKOverlay, MKPolygon, MKPolyline, MKTileOverlay};
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result};

pub type MKZoomScale = f64;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKOverlayRendererState {
    alpha: f64,
    content_scale_factor: f64,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKOverlayRendererOptions {
    alpha: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKOverlayPathRendererState {
    base: MKOverlayRendererState,
    line_width: f64,
    line_dash_phase: f64,
    line_dash_pattern: Option<Vec<f64>>,
    should_rasterize: bool,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKOverlayPathRendererOptions {
    line_width: Option<f64>,
    line_dash_phase: Option<f64>,
    line_dash_pattern_present: bool,
    line_dash_pattern: Option<Vec<f64>>,
    should_rasterize: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKStrokeRendererState {
    base: MKOverlayPathRendererState,
    stroke_start: f64,
    stroke_end: f64,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKStrokeRendererOptions {
    stroke_start: Option<f64>,
    stroke_end: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKMapRectZoomScalePayload {
    map_rect: MKMapRect,
    zoom_scale: MKZoomScale,
}

#[derive(Debug)]
pub struct MKOverlayRenderer {
    raw: NonNull<c_void>,
}

impl MKOverlayRenderer {
    pub fn new<O: MKOverlay + ?Sized>(overlay: &O) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_overlay_renderer_new(overlay.as_raw_overlay(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKOverlayRenderer")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKOverlayRendererState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe { ffi::mk_overlay_renderer_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKOverlayRenderer state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKOverlayRenderer state") }
        }
    }

    fn apply_options(&self, options: &MKOverlayRendererOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKOverlayRenderer options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_overlay_renderer_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKOverlayRenderer") }
    }

    pub fn alpha(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.alpha)
    }

    pub fn set_alpha(&self, alpha: f64) -> Result<(), MapKitError> {
        self.apply_options(&MKOverlayRendererOptions { alpha: Some(alpha) })
    }

    pub fn content_scale_factor(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.content_scale_factor)
    }

    pub fn can_draw_map_rect(
        &self,
        map_rect: MKMapRect,
        zoom_scale: MKZoomScale,
    ) -> Result<bool, MapKitError> {
        let payload = json_cstring(
            &MKMapRectZoomScalePayload {
                map_rect,
                zoom_scale,
            },
            "MKOverlayRenderer canDraw payload",
        )?;
        let mut error = ptr::null_mut();
        let result = unsafe {
            ffi::mk_overlay_renderer_can_draw_map_rect_json(
                self.raw.as_ptr(),
                payload.as_ptr(),
                &mut error,
            )
        };
        if error.is_null() {
            Ok(result)
        } else {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "MKOverlayRenderer canDrawMapRect failed")
            })
        }
    }

    pub fn set_needs_display(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_overlay_renderer_set_needs_display(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to invalidate MKOverlayRenderer") }
    }

    pub fn set_needs_display_in_map_rect(&self, map_rect: MKMapRect) -> Result<(), MapKitError> {
        let payload = json_cstring(&map_rect, "MKMapRect")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_overlay_renderer_set_needs_display_in_map_rect_json(
                self.raw.as_ptr(),
                payload.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to invalidate MKOverlayRenderer map rect") }
    }

    pub fn set_needs_display_in_map_rect_with_zoom_scale(
        &self,
        map_rect: MKMapRect,
        zoom_scale: MKZoomScale,
    ) -> Result<(), MapKitError> {
        let payload = json_cstring(
            &MKMapRectZoomScalePayload {
                map_rect,
                zoom_scale,
            },
            "MKOverlayRenderer map rect payload",
        )?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_overlay_renderer_set_needs_display_in_map_rect_zoom_scale_json(
                self.raw.as_ptr(),
                payload.as_ptr(),
                &mut error,
            );
        };
        unsafe {
            unit_result(
                error,
                "failed to invalidate MKOverlayRenderer map rect with zoom scale",
            )
        }
    }
}

impl Drop for MKOverlayRenderer {
    fn drop(&mut self) {
        unsafe { ffi::mk_overlay_renderer_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKOverlayPathRenderer {
    raw: NonNull<c_void>,
}

impl MKOverlayPathRenderer {
    pub fn new<O: MKOverlay + ?Sized>(overlay: &O) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw =
            unsafe { ffi::mk_overlay_path_renderer_new(overlay.as_raw_overlay(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKOverlayPathRenderer")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKOverlayPathRendererState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_overlay_path_renderer_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKOverlayPathRenderer state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKOverlayPathRenderer state") }
        }
    }

    fn apply_options(&self, options: &MKOverlayPathRendererOptions) -> Result<(), MapKitError> {
        let options = json_cstring(options, "MKOverlayPathRenderer options")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_overlay_path_renderer_apply_options_json(
                self.raw.as_ptr(),
                options.as_ptr(),
                &mut error,
            );
        };
        unsafe { unit_result(error, "failed to update MKOverlayPathRenderer") }
    }

    pub fn alpha(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.base.alpha)
    }

    pub fn content_scale_factor(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.base.content_scale_factor)
    }

    pub fn line_width(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.line_width)
    }

    pub fn set_line_width(&self, line_width: f64) -> Result<(), MapKitError> {
        self.apply_options(&MKOverlayPathRendererOptions {
            line_width: Some(line_width),
            ..MKOverlayPathRendererOptions::default()
        })
    }

    pub fn line_dash_phase(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.line_dash_phase)
    }

    pub fn set_line_dash_phase(&self, line_dash_phase: f64) -> Result<(), MapKitError> {
        self.apply_options(&MKOverlayPathRendererOptions {
            line_dash_phase: Some(line_dash_phase),
            ..MKOverlayPathRendererOptions::default()
        })
    }

    pub fn line_dash_pattern(&self) -> Result<Option<Vec<f64>>, MapKitError> {
        Ok(self.state()?.line_dash_pattern)
    }

    pub fn set_line_dash_pattern(
        &self,
        line_dash_pattern: Option<Vec<f64>>,
    ) -> Result<(), MapKitError> {
        self.apply_options(&MKOverlayPathRendererOptions {
            line_dash_pattern_present: true,
            line_dash_pattern,
            ..MKOverlayPathRendererOptions::default()
        })
    }

    pub fn should_rasterize(&self) -> Result<bool, MapKitError> {
        Ok(self.state()?.should_rasterize)
    }

    pub fn set_should_rasterize(&self, should_rasterize: bool) -> Result<(), MapKitError> {
        self.apply_options(&MKOverlayPathRendererOptions {
            should_rasterize: Some(should_rasterize),
            ..MKOverlayPathRendererOptions::default()
        })
    }
}

impl Drop for MKOverlayPathRenderer {
    fn drop(&mut self) {
        unsafe { ffi::mk_overlay_path_renderer_release(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MKCircleRenderer {
    raw: NonNull<c_void>,
}

macro_rules! stroke_renderer_impl {
    ($name:ident, $new_fn:ident, $state_fn:ident, $apply_fn:ident, $release_fn:ident, $overlay_ty:ty, $label:literal) => {
        impl $name {
            pub fn new(overlay: &$overlay_ty) -> Result<Self, MapKitError> {
                let mut error = ptr::null_mut();
                let raw = unsafe { ffi::$new_fn(overlay.as_raw(), &mut error) };
                let raw = owned_handle(raw, error, concat!("failed to create ", $label))?;
                Ok(Self { raw })
            }

            fn state(&self) -> Result<MKStrokeRendererState, MapKitError> {
                let mut error = ptr::null_mut();
                let payload = unsafe { ffi::$state_fn(self.raw.as_ptr(), &mut error) };
                if payload.is_null() {
                    Err(unsafe {
                        MapKitError::from_error_ptr(
                            error,
                            concat!("failed to read ", $label, " state"),
                        )
                    })
                } else {
                    unsafe { parse_json_ptr(payload, concat!($label, " state")) }
                }
            }

            fn apply_options(&self, options: &MKStrokeRendererOptions) -> Result<(), MapKitError> {
                let options = json_cstring(options, concat!($label, " options"))?;
                let mut error = ptr::null_mut();
                unsafe { ffi::$apply_fn(self.raw.as_ptr(), options.as_ptr(), &mut error) };
                unsafe { unit_result(error, concat!("failed to update ", $label)) }
            }

            pub fn alpha(&self) -> Result<f64, MapKitError> {
                Ok(self.state()?.base.base.alpha)
            }

            pub fn line_width(&self) -> Result<f64, MapKitError> {
                Ok(self.state()?.base.line_width)
            }

            pub fn stroke_start(&self) -> Result<f64, MapKitError> {
                Ok(self.state()?.stroke_start)
            }

            pub fn set_stroke_start(&self, stroke_start: f64) -> Result<(), MapKitError> {
                self.apply_options(&MKStrokeRendererOptions {
                    stroke_start: Some(stroke_start),
                    ..MKStrokeRendererOptions::default()
                })
            }

            pub fn stroke_end(&self) -> Result<f64, MapKitError> {
                Ok(self.state()?.stroke_end)
            }

            pub fn set_stroke_end(&self, stroke_end: f64) -> Result<(), MapKitError> {
                self.apply_options(&MKStrokeRendererOptions {
                    stroke_end: Some(stroke_end),
                    ..MKStrokeRendererOptions::default()
                })
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { ffi::$release_fn(self.raw.as_ptr()) };
            }
        }
    };
}

stroke_renderer_impl!(
    MKCircleRenderer,
    mk_circle_renderer_new,
    mk_circle_renderer_state_json,
    mk_circle_renderer_apply_options_json,
    mk_circle_renderer_release,
    MKCircle,
    "MKCircleRenderer"
);

#[derive(Debug)]
pub struct MKPolylineRenderer {
    raw: NonNull<c_void>,
}

stroke_renderer_impl!(
    MKPolylineRenderer,
    mk_polyline_renderer_new,
    mk_polyline_renderer_state_json,
    mk_polyline_renderer_apply_options_json,
    mk_polyline_renderer_release,
    MKPolyline,
    "MKPolylineRenderer"
);

#[derive(Debug)]
pub struct MKGradientPolylineRenderer {
    raw: NonNull<c_void>,
}

stroke_renderer_impl!(
    MKGradientPolylineRenderer,
    mk_gradient_polyline_renderer_new,
    mk_gradient_polyline_renderer_state_json,
    mk_gradient_polyline_renderer_apply_options_json,
    mk_gradient_polyline_renderer_release,
    MKPolyline,
    "MKGradientPolylineRenderer"
);

#[derive(Debug)]
pub struct MKPolygonRenderer {
    raw: NonNull<c_void>,
}

stroke_renderer_impl!(
    MKPolygonRenderer,
    mk_polygon_renderer_new,
    mk_polygon_renderer_state_json,
    mk_polygon_renderer_apply_options_json,
    mk_polygon_renderer_release,
    MKPolygon,
    "MKPolygonRenderer"
);

#[derive(Debug)]
pub struct MKTileOverlayRenderer {
    raw: NonNull<c_void>,
}

impl MKTileOverlayRenderer {
    pub fn new(overlay: &MKTileOverlay) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_tile_overlay_renderer_new(overlay.as_raw(), &mut error) };
        let raw = owned_handle(raw, error, "failed to create MKTileOverlayRenderer")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKOverlayRendererState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_tile_overlay_renderer_state_json(self.raw.as_ptr(), &mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKTileOverlayRenderer state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKTileOverlayRenderer state") }
        }
    }

    pub fn alpha(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.alpha)
    }

    pub fn content_scale_factor(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.content_scale_factor)
    }

    pub fn reload_data(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_tile_overlay_renderer_reload_data(self.raw.as_ptr(), &mut error) };
        unsafe { unit_result(error, "failed to reload MKTileOverlayRenderer") }
    }
}

impl Drop for MKTileOverlayRenderer {
    fn drop(&mut self) {
        unsafe { ffi::mk_tile_overlay_renderer_release(self.raw.as_ptr()) };
    }
}

pub fn mk_road_width_at_zoom_scale(zoom_scale: MKZoomScale) -> f64 {
    unsafe { ffi::mk_road_width_at_zoom_scale(zoom_scale) }
}
