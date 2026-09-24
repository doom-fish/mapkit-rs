use core::ffi::c_void;
use std::ptr::{self, NonNull};

use serde::{Deserialize, Serialize};

use crate::error::MapKitError;
use crate::ffi;
use crate::geometry::MKMapRect;
use crate::overlay::{
    MKCircle, MKMultiPolygon, MKMultiPolyline, MKOverlay, MKPolygon, MKPolyline, MKTileOverlay,
};
use crate::private::{json_cstring, owned_handle, parse_json_ptr, unit_result, Sealed};

/// Wraps `MKZoomScale`.
pub type MKZoomScale = f64;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MKColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl MKColor {
    pub const fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    fn validated(self, name: &str) -> Result<Self, MapKitError> {
        for component in [self.red, self.green, self.blue, self.alpha] {
            unit_interval(component, name)?;
        }
        Ok(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKLineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MKLineJoin {
    Miter,
    Round,
    Bevel,
}

fn unit_interval(value: f64, name: &str) -> Result<f64, MapKitError> {
    if value.is_finite() && (0.0..=1.0).contains(&value) {
        Ok(value)
    } else {
        Err(MapKitError::InvalidArgument(format!(
            "{name} must be between 0 and 1, got {value}"
        )))
    }
}

fn non_negative(value: f64, name: &str) -> Result<f64, MapKitError> {
    if value.is_finite() && value >= 0.0 {
        Ok(value)
    } else {
        Err(MapKitError::InvalidArgument(format!(
            "{name} must be finite and non-negative, got {value}"
        )))
    }
}

fn validated_dash_pattern(pattern: Option<Vec<f64>>) -> Result<Option<Vec<f64>>, MapKitError> {
    if let Some(lengths) = &pattern {
        for &length in lengths {
            non_negative(length, "line dash pattern length")?;
        }
        if !lengths.is_empty() && lengths.iter().all(|&length| length <= 0.0) {
            return Err(MapKitError::InvalidArgument(
                "line dash pattern needs at least one non-zero length".to_owned(),
            ));
        }
    }
    Ok(pattern)
}

fn validated_color(color: Option<MKColor>, name: &str) -> Result<Option<MKColor>, MapKitError> {
    color.map(|color| color.validated(name)).transpose()
}

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
    stroke_color: Option<MKColor>,
    fill_color: Option<MKColor>,
    line_width: f64,
    line_cap: MKLineCap,
    line_join: MKLineJoin,
    miter_limit: f64,
    line_dash_phase: f64,
    line_dash_pattern: Option<Vec<f64>>,
    should_rasterize: bool,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKOverlayPathRendererOptions {
    stroke_color_present: bool,
    stroke_color: Option<MKColor>,
    fill_color_present: bool,
    fill_color: Option<MKColor>,
    line_width: Option<f64>,
    line_cap: Option<MKLineCap>,
    line_join: Option<MKLineJoin>,
    miter_limit: Option<f64>,
    line_dash_phase: Option<f64>,
    line_dash_pattern_present: bool,
    line_dash_pattern: Option<Vec<f64>>,
    should_rasterize: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MKStrokeRendererState {
    stroke_start: f64,
    stroke_end: f64,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKStrokeRendererOptions {
    stroke_start: Option<f64>,
    stroke_end: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MKGradientPolylineColors {
    colors: Vec<MKColor>,
    locations: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct MKMapRectZoomScalePayload {
    map_rect: MKMapRect,
    zoom_scale: MKZoomScale,
}

fn set_renderer_alpha(raw: NonNull<c_void>, alpha: f64, label: &str) -> Result<(), MapKitError> {
    let options = MKOverlayRendererOptions {
        alpha: Some(unit_interval(alpha, "alpha")?),
    };
    let options = json_cstring(&options, &format!("{label} options"))?;
    let mut error = ptr::null_mut();
    unsafe {
        ffi::mk_overlay_renderer_apply_options_json(raw.as_ptr(), options.as_ptr(), &raw mut error);
    };
    unsafe { unit_result(error, &format!("failed to update {label}")) }
}

fn path_state(
    raw: NonNull<c_void>,
    label: &str,
) -> Result<MKOverlayPathRendererState, MapKitError> {
    let mut error = ptr::null_mut();
    let payload = unsafe { ffi::mk_overlay_path_renderer_state_json(raw.as_ptr(), &raw mut error) };
    if payload.is_null() {
        Err(unsafe { MapKitError::from_error_ptr(error, &format!("failed to read {label} state")) })
    } else {
        unsafe { parse_json_ptr(payload, &format!("{label} state")) }
    }
}

fn apply_path_options(
    raw: NonNull<c_void>,
    options: &MKOverlayPathRendererOptions,
    label: &str,
) -> Result<(), MapKitError> {
    let options = json_cstring(options, &format!("{label} options"))?;
    let mut error = ptr::null_mut();
    unsafe {
        ffi::mk_overlay_path_renderer_apply_options_json(
            raw.as_ptr(),
            options.as_ptr(),
            &raw mut error,
        );
    };
    unsafe { unit_result(error, &format!("failed to update {label}")) }
}

/// Wraps `MKOverlayRenderer`.
#[derive(Debug)]
pub struct MKOverlayRenderer {
    raw: NonNull<c_void>,
}

impl MKOverlayRenderer {
    /// Creates a wrapper for `MKOverlayRenderer`.
    pub fn new<O: MKOverlay + ?Sized>(overlay: &O) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_overlay_renderer_new(overlay.as_raw_overlay(), &raw mut error) };
        let raw = owned_handle(raw, error, "failed to create MKOverlayRenderer")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKOverlayRendererState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_overlay_renderer_state_json(self.raw.as_ptr(), &raw mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKOverlayRenderer state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKOverlayRenderer state") }
        }
    }

    /// Wraps `MKOverlayRenderer.alpha`.
    pub fn alpha(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.alpha)
    }

    /// Wraps `MKOverlayRenderer.alpha`.
    pub fn set_alpha(&self, alpha: f64) -> Result<(), MapKitError> {
        set_renderer_alpha(self.raw, alpha, "MKOverlayRenderer")
    }

    /// Wraps `MKOverlayRenderer.contentScaleFactor`.
    pub fn content_scale_factor(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.content_scale_factor)
    }

    /// Wraps `MKOverlayRenderer.canDrawMapRect`.
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
                &raw mut error,
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

    /// Wraps `MKOverlayRenderer.needsDisplay`.
    pub fn set_needs_display(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_overlay_renderer_set_needs_display(self.raw.as_ptr(), &raw mut error) };
        unsafe { unit_result(error, "failed to invalidate MKOverlayRenderer") }
    }

    /// Wraps `MKOverlayRenderer.needsDisplayInMapRect`.
    pub fn set_needs_display_in_map_rect(&self, map_rect: MKMapRect) -> Result<(), MapKitError> {
        let payload = json_cstring(&map_rect, "MKMapRect")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_overlay_renderer_set_needs_display_in_map_rect_json(
                self.raw.as_ptr(),
                payload.as_ptr(),
                &raw mut error,
            );
        };
        unsafe { unit_result(error, "failed to invalidate MKOverlayRenderer map rect") }
    }

    /// Wraps `MKOverlayRenderer.needsDisplayInMapRectWithZoomScale`.
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
                &raw mut error,
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

macro_rules! path_style_methods {
    ($label:literal) => {
        /// Wraps `MKOverlayPathRenderer.alpha`.
        pub fn alpha(&self) -> Result<f64, MapKitError> {
            Ok(path_state(self.raw, $label)?.base.alpha)
        }

        pub fn set_alpha(&self, alpha: f64) -> Result<(), MapKitError> {
            set_renderer_alpha(self.raw, alpha, $label)
        }

        /// Wraps `MKOverlayPathRenderer.contentScaleFactor`.
        pub fn content_scale_factor(&self) -> Result<f64, MapKitError> {
            Ok(path_state(self.raw, $label)?.base.content_scale_factor)
        }

        pub fn stroke_color(&self) -> Result<Option<MKColor>, MapKitError> {
            Ok(path_state(self.raw, $label)?.stroke_color)
        }

        pub fn set_stroke_color(&self, stroke_color: Option<MKColor>) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    stroke_color_present: true,
                    stroke_color: validated_color(stroke_color, "stroke color")?,
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        /// Wraps `MKOverlayPathRenderer.lineWidth`.
        pub fn line_width(&self) -> Result<f64, MapKitError> {
            Ok(path_state(self.raw, $label)?.line_width)
        }

        /// Wraps `MKOverlayPathRenderer.lineWidth`.
        pub fn set_line_width(&self, line_width: f64) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    line_width: Some(non_negative(line_width, "line width")?),
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        pub fn line_cap(&self) -> Result<MKLineCap, MapKitError> {
            Ok(path_state(self.raw, $label)?.line_cap)
        }

        pub fn set_line_cap(&self, line_cap: MKLineCap) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    line_cap: Some(line_cap),
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        pub fn line_join(&self) -> Result<MKLineJoin, MapKitError> {
            Ok(path_state(self.raw, $label)?.line_join)
        }

        pub fn set_line_join(&self, line_join: MKLineJoin) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    line_join: Some(line_join),
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        pub fn miter_limit(&self) -> Result<f64, MapKitError> {
            Ok(path_state(self.raw, $label)?.miter_limit)
        }

        pub fn set_miter_limit(&self, miter_limit: f64) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    miter_limit: Some(non_negative(miter_limit, "miter limit")?),
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        /// Wraps `MKOverlayPathRenderer.lineDashPhase`.
        pub fn line_dash_phase(&self) -> Result<f64, MapKitError> {
            Ok(path_state(self.raw, $label)?.line_dash_phase)
        }

        /// Wraps `MKOverlayPathRenderer.lineDashPhase`.
        pub fn set_line_dash_phase(&self, line_dash_phase: f64) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    line_dash_phase: Some(non_negative(line_dash_phase, "line dash phase")?),
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        /// Wraps `MKOverlayPathRenderer.lineDashPattern`.
        pub fn line_dash_pattern(&self) -> Result<Option<Vec<f64>>, MapKitError> {
            Ok(path_state(self.raw, $label)?.line_dash_pattern)
        }

        /// Wraps `MKOverlayPathRenderer.lineDashPattern`.
        pub fn set_line_dash_pattern(
            &self,
            line_dash_pattern: Option<Vec<f64>>,
        ) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    line_dash_pattern_present: true,
                    line_dash_pattern: validated_dash_pattern(line_dash_pattern)?,
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }

        /// Wraps `MKOverlayPathRenderer.shouldRasterize`.
        pub fn should_rasterize(&self) -> Result<bool, MapKitError> {
            Ok(path_state(self.raw, $label)?.should_rasterize)
        }

        /// Wraps `MKOverlayPathRenderer.shouldRasterize`.
        pub fn set_should_rasterize(&self, should_rasterize: bool) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    should_rasterize: Some(should_rasterize),
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }
    };
}

macro_rules! path_fill_methods {
    ($label:literal) => {
        pub fn fill_color(&self) -> Result<Option<MKColor>, MapKitError> {
            Ok(path_state(self.raw, $label)?.fill_color)
        }

        pub fn set_fill_color(&self, fill_color: Option<MKColor>) -> Result<(), MapKitError> {
            apply_path_options(
                self.raw,
                &MKOverlayPathRendererOptions {
                    fill_color_present: true,
                    fill_color: validated_color(fill_color, "fill color")?,
                    ..MKOverlayPathRendererOptions::default()
                },
                $label,
            )
        }
    };
}

/// Wraps `MKOverlayPathRenderer`.
#[derive(Debug)]
pub struct MKOverlayPathRenderer {
    raw: NonNull<c_void>,
}

impl MKOverlayPathRenderer {
    /// Creates a wrapper for `MKOverlayPathRenderer`.
    pub fn new<O: MKOverlay + ?Sized>(overlay: &O) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw =
            unsafe { ffi::mk_overlay_path_renderer_new(overlay.as_raw_overlay(), &raw mut error) };
        let raw = owned_handle(raw, error, "failed to create MKOverlayPathRenderer")?;
        Ok(Self { raw })
    }

    path_style_methods!("MKOverlayPathRenderer");
    path_fill_methods!("MKOverlayPathRenderer");
}

impl Drop for MKOverlayPathRenderer {
    fn drop(&mut self) {
        unsafe { ffi::mk_overlay_path_renderer_release(self.raw.as_ptr()) };
    }
}

/// Wraps `MKCircleRenderer`.
#[derive(Debug)]
pub struct MKCircleRenderer {
    raw: NonNull<c_void>,
}

macro_rules! stroke_renderer_impl {
    ($name:ident, $new_fn:ident, $state_fn:ident, $apply_fn:ident, $release_fn:ident, $overlay_ty:ty, $label:literal) => {
        impl $name {
            /// Wraps `new`.
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

            path_style_methods!($label);

            /// Wraps `stroke_start`.
            pub fn stroke_start(&self) -> Result<f64, MapKitError> {
                Ok(self.state()?.stroke_start)
            }

            /// Wraps `set_stroke_start`.
            pub fn set_stroke_start(&self, stroke_start: f64) -> Result<(), MapKitError> {
                self.apply_options(&MKStrokeRendererOptions {
                    stroke_start: Some(unit_interval(stroke_start, "stroke start")?),
                    ..MKStrokeRendererOptions::default()
                })
            }

            /// Wraps `stroke_end`.
            pub fn stroke_end(&self) -> Result<f64, MapKitError> {
                Ok(self.state()?.stroke_end)
            }

            /// Wraps `set_stroke_end`.
            pub fn set_stroke_end(&self, stroke_end: f64) -> Result<(), MapKitError> {
                self.apply_options(&MKStrokeRendererOptions {
                    stroke_end: Some(unit_interval(stroke_end, "stroke end")?),
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

impl MKCircleRenderer {
    path_fill_methods!("MKCircleRenderer");
}

/// Wraps `MKPolylineRenderer`.
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

/// Wraps `MKGradientPolylineRenderer`.
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

impl MKGradientPolylineRenderer {
    fn gradient(&self) -> Result<MKGradientPolylineColors, MapKitError> {
        let mut error = ptr::null_mut();
        let payload = unsafe {
            ffi::mk_gradient_polyline_renderer_colors_json(self.raw.as_ptr(), &raw mut error)
        };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(
                    error,
                    "failed to read MKGradientPolylineRenderer colors",
                )
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKGradientPolylineRenderer colors") }
        }
    }

    pub fn colors(&self) -> Result<Vec<MKColor>, MapKitError> {
        Ok(self.gradient()?.colors)
    }

    pub fn locations(&self) -> Result<Vec<f64>, MapKitError> {
        Ok(self.gradient()?.locations)
    }

    pub fn set_colors(&self, colors: &[MKColor], locations: &[f64]) -> Result<(), MapKitError> {
        if colors.len() != locations.len() {
            return Err(MapKitError::InvalidArgument(format!(
                "MKGradientPolylineRenderer needs one location per color, got {} colors and {} locations",
                colors.len(),
                locations.len()
            )));
        }
        let payload = MKGradientPolylineColors {
            colors: colors
                .iter()
                .map(|color| color.validated("gradient color"))
                .collect::<Result<_, _>>()?,
            locations: locations
                .iter()
                .map(|&location| unit_interval(location, "gradient location"))
                .collect::<Result<_, _>>()?,
        };
        let payload = json_cstring(&payload, "MKGradientPolylineRenderer colors")?;
        let mut error = ptr::null_mut();
        unsafe {
            ffi::mk_gradient_polyline_renderer_set_colors_json(
                self.raw.as_ptr(),
                payload.as_ptr(),
                &raw mut error,
            );
        };
        unsafe { unit_result(error, "failed to set MKGradientPolylineRenderer colors") }
    }
}

/// Wraps `MKPolygonRenderer`.
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

impl MKPolygonRenderer {
    path_fill_methods!("MKPolygonRenderer");
}

/// Wraps `MKTileOverlayRenderer`.
#[derive(Debug)]
pub struct MKTileOverlayRenderer {
    raw: NonNull<c_void>,
}

impl MKTileOverlayRenderer {
    /// Creates a wrapper for `MKTileOverlayRenderer`.
    pub fn new(overlay: &MKTileOverlay) -> Result<Self, MapKitError> {
        let mut error = ptr::null_mut();
        let raw = unsafe { ffi::mk_tile_overlay_renderer_new(overlay.as_raw(), &raw mut error) };
        let raw = owned_handle(raw, error, "failed to create MKTileOverlayRenderer")?;
        Ok(Self { raw })
    }

    fn state(&self) -> Result<MKOverlayRendererState, MapKitError> {
        let mut error = ptr::null_mut();
        let payload =
            unsafe { ffi::mk_tile_overlay_renderer_state_json(self.raw.as_ptr(), &raw mut error) };
        if payload.is_null() {
            Err(unsafe {
                MapKitError::from_error_ptr(error, "failed to read MKTileOverlayRenderer state")
            })
        } else {
            unsafe { parse_json_ptr(payload, "MKTileOverlayRenderer state") }
        }
    }

    /// Wraps `MKTileOverlayRenderer.alpha`.
    pub fn alpha(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.alpha)
    }

    pub fn set_alpha(&self, alpha: f64) -> Result<(), MapKitError> {
        set_renderer_alpha(self.raw, alpha, "MKTileOverlayRenderer")
    }

    /// Wraps `MKTileOverlayRenderer.contentScaleFactor`.
    pub fn content_scale_factor(&self) -> Result<f64, MapKitError> {
        Ok(self.state()?.content_scale_factor)
    }

    /// Wraps `MKTileOverlayRenderer.reloadData`.
    pub fn reload_data(&self) -> Result<(), MapKitError> {
        let mut error = ptr::null_mut();
        unsafe { ffi::mk_tile_overlay_renderer_reload_data(self.raw.as_ptr(), &raw mut error) };
        unsafe { unit_result(error, "failed to reload MKTileOverlayRenderer") }
    }
}

impl Drop for MKTileOverlayRenderer {
    fn drop(&mut self) {
        unsafe { ffi::mk_tile_overlay_renderer_release(self.raw.as_ptr()) };
    }
}

macro_rules! path_renderer_impl {
    ($name:ident, $new_fn:ident, $overlay_ty:ty, $label:literal) => {
        #[derive(Debug)]
        pub struct $name {
            raw: NonNull<c_void>,
        }

        impl $name {
            /// Wraps `new`.
            pub fn new(overlay: &$overlay_ty) -> Result<Self, MapKitError> {
                let mut error = ptr::null_mut();
                let raw = unsafe { ffi::$new_fn(overlay.as_raw(), &mut error) };
                let raw = owned_handle(raw, error, concat!("failed to create ", $label))?;
                Ok(Self { raw })
            }

            path_style_methods!($label);
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { ffi::mk_overlay_path_renderer_release(self.raw.as_ptr()) };
            }
        }
    };
}

path_renderer_impl!(
    MKMultiPolylineRenderer,
    mk_multi_polyline_renderer_new,
    MKMultiPolyline,
    "MKMultiPolylineRenderer"
);

path_renderer_impl!(
    MKMultiPolygonRenderer,
    mk_multi_polygon_renderer_new,
    MKMultiPolygon,
    "MKMultiPolygonRenderer"
);

impl MKMultiPolygonRenderer {
    path_fill_methods!("MKMultiPolygonRenderer");
}

pub trait MKOverlayRendering: Sealed {
    #[doc(hidden)]
    fn as_raw_renderer(&self) -> *mut c_void;
}

macro_rules! overlay_rendering_impl {
    ($($name:ident),+ $(,)?) => {
        $(
            impl Sealed for $name {}

            impl MKOverlayRendering for $name {
                fn as_raw_renderer(&self) -> *mut c_void {
                    self.raw.as_ptr()
                }
            }
        )+
    };
}

overlay_rendering_impl!(
    MKOverlayRenderer,
    MKOverlayPathRenderer,
    MKCircleRenderer,
    MKPolylineRenderer,
    MKGradientPolylineRenderer,
    MKPolygonRenderer,
    MKTileOverlayRenderer,
    MKMultiPolylineRenderer,
    MKMultiPolygonRenderer,
);

/// Wraps `MKRoadWidthAtZoomScale`.
pub fn mk_road_width_at_zoom_scale(zoom_scale: MKZoomScale) -> f64 {
    unsafe { ffi::mk_road_width_at_zoom_scale(zoom_scale) }
}
