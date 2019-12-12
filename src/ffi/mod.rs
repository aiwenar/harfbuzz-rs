//! Bindings to HarfBuzz.
//!
//! Most documentation in this crate comes directly from HarfBuzz's source code.

mod blob;
mod buffer;
mod common;
mod face;
mod font;
mod set;
mod shape;
mod unicode;

#[cfg(feature = "freetype")]
mod ft;

pub use self::blob::*;
pub use self::buffer::*;
pub use self::common::*;
pub use self::face::*;
pub use self::font::*;
pub use self::set::*;
pub use self::shape::*;
pub use self::unicode::*;

#[cfg(feature = "freetype")]
pub use self::ft::*;
