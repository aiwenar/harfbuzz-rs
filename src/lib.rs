#[macro_use] extern crate bitflags;
#[macro_use] extern crate cffi;

pub mod ffi;

mod blob;
mod buffer;
mod face;
mod font;
mod shape;

pub use self::blob::*;
pub use self::buffer::*;
pub use self::face::*;
pub use self::font::*;
pub use self::shape::*;

pub type Codepoint = ffi::hb_codepoint_t;
pub type Direction = ffi::hb_direction_t;
pub type Variation = ffi::hb_variation_t;
pub type Position = ffi::hb_position_t;
pub type GlyphExtents = ffi::hb_glyph_extents_t;
pub type FontExtents = ffi::hb_font_extents_t;
pub type Tag = ffi::hb_tag_t;
pub type Feature = ffi::hb_feature_t;
