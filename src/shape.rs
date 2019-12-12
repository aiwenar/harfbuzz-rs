use std::{mem, os::raw};

use crate::*;

/// Shapes `buffer` using `font` turning its Unicode characters content to
/// positioned glyphs. If `features` is not empty, it will be used to control
/// the features applied during shaping.
pub fn shape(
    font: &ffi::hb_font_t,
    buffer: &mut ffi::hb_buffer_t,
    features: &[Feature],
) {
    unsafe {
        ffi::hb_shape(
            mem::transmute(font),
            buffer,
            features.as_ptr(),
            features.len() as raw::c_uint,
        )
    }
}
