#![allow(missing_docs)]

use core::ffi::c_char;

mod annotation;
mod annotation_view;
mod cluster_annotation;
mod configuration;
mod directions;
mod distance_formatter;
mod geocoder;
mod geojson;
mod geometry;
mod local_search;
mod local_search_completer;
mod look_around;
mod map_item;
mod map_view;
mod overlay;
mod overlay_renderer;
mod point_of_interest;
mod snapshotter;
mod user_tracking_button;

pub use annotation::*;
pub use annotation_view::*;
pub use cluster_annotation::*;
pub use configuration::*;
pub use directions::*;
pub use distance_formatter::*;
pub use geocoder::*;
pub use geojson::*;
pub use geometry::*;
pub use local_search::*;
pub use local_search_completer::*;
pub use look_around::*;
pub use map_item::*;
pub use map_view::*;
pub use overlay::*;
pub use overlay_renderer::*;
pub use point_of_interest::*;
pub use snapshotter::*;
pub use user_tracking_button::*;

extern "C" {
    pub fn mk_string_free(string: *mut c_char);
}
