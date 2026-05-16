#![allow(missing_docs)]

use core::ffi::c_char;

mod annotation;
mod cluster_annotation;
mod directions;
mod distance_formatter;
mod geocoder;
mod geometry;
mod local_search;
mod look_around;
mod map_view;
mod overlay;
mod snapshotter;
mod user_tracking_button;

pub use annotation::*;
pub use cluster_annotation::*;
pub use directions::*;
pub use distance_formatter::*;
pub use geocoder::*;
pub use geometry::*;
pub use local_search::*;
pub use look_around::*;
pub use map_view::*;
pub use overlay::*;
pub use snapshotter::*;
pub use user_tracking_button::*;

extern "C" {
    pub fn mk_string_free(string: *mut c_char);
}
