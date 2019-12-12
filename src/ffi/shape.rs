//! Conversion of text strings into positioned glyphs.
//!
//! This module contains raw bindings to `hb-shape.h`.
//!
//! Shaping is the central operation of HarfBuzz. Shaping operates on buffers,
//! which are sequences of Unicode characters that use the same font and have
//! the same text direction, script and language. After shaping the buffer
//! contains the output glyphs and their positions.

use std::os::raw::*;

use crate::ffi::{
    buffer::*,
    common::*,
    font::*,
};

extern "C" {
    /// Shapes `buffer` using `font` turning its Unicode characters content to
    /// positioned glyphs. If `features` is not null, it will be used to control
    /// the features applied during shaping.
    ///
    /// ## Arguments
    ///
    /// - `font`: an [`hb_font_t`] to use for shaping
    /// - `buffer`: an [`hb_buffer_t`] to shape
    /// - `features`: an array of user specified [`hb_feature_t`] or null
    /// - `num_features`: the length of `features` array
    pub fn hb_shape(
        font: *mut hb_font_t,
        buffer: *mut hb_buffer_t,
        features: *const hb_feature_t,
        num_features: c_uint,
    );

    /// See [`hb_shape()`] for details. If `shaper_list` is not null,
    /// the specified shapers will be used in the given order, otherwise
    /// the default shapers list will be used.
    ///
    /// Return false if all shapers failed, true otherwise.
    ///
    /// ## Arguments
    ///
    /// - `font`: an [`hb_font_t`] to use for shaping
    /// - `buffer`: an [`hb_buffer_t`] to shape
    /// - `features`: an array of user specified [`hb_feature_t`] or null
    /// - `num_features`: the length of `features` array
    /// - `shaper_list`: a null-terminated array of shapers to use or null
    pub fn hb_shape_full(
        font: *mut hb_font_t,
        buffer: *mut hb_buffer_t,
        features: *const hb_feature_t,
        num_features: c_uint,
        shaper_list: *const *const c_char,
    ) -> hb_bool_t;

    /// Retrieves the list of shapers supported by HarfBuzz.
    pub fn hb_shape_list_shapers() -> *mut *const c_char;
}
