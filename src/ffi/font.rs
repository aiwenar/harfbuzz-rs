//! Bindings to `hb-font.h`.

#![allow(non_camel_case_types)]

use std::os::raw::*;

use crate::ffi::{
    common::*,
    face::*,
};

#[repr(C)]
pub struct hb_font_t(cffi::Private);

#[repr(C)]
pub struct hb_font_funcs_t(cffi::Private);

/// Note that typically ascender is positive and descender negative
/// in coordinate systems that grow up.
#[repr(C)]
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct hb_font_extents_t {
    /// Typographic ascender.
    pub ascender: hb_position_t,
    /// Typographic descender.
    pub descender: hb_position_t,
    /// suggested line spacing gap.
    pub line_gap: hb_position_t,
    _reserved9: hb_position_t,
    _reserved8: hb_position_t,
    _reserved7: hb_position_t,
    _reserved6: hb_position_t,
    _reserved5: hb_position_t,
    _reserved4: hb_position_t,
    _reserved3: hb_position_t,
    _reserved2: hb_position_t,
    _reserved1: hb_position_t,
}

/// Note that height is negative in coordinate systems that grow up.
#[repr(C)]
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct hb_glyph_extents_t
{
  /// Left side of glyph from origin.
  pub x_bearing: hb_position_t,
  /// Top side of glyph from origin.
  pub y_bearing: hb_position_t,
  /// Distance from left to right side.
  pub width: hb_position_t,
  /// Distance from top to bottom side.
  pub height: hb_position_t,
}

pub type hb_font_get_font_extents_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    metrics: *mut hb_font_extents_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_font_h_extents_func_t = hb_font_get_font_extents_func_t;
pub type hb_font_get_font_v_extents_func_t = hb_font_get_font_extents_func_t;
pub type hb_font_get_nominal_glyph_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    unicode: hb_codepoint_t,
    glyph: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_variation_glyph_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    unicode: hb_codepoint_t,
    variation_selector: hb_codepoint_t,
    glyph: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_nominal_glyphs_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    count: c_uint,
    first_unicode: *const hb_codepoint_t,
    unicode_stride: c_uint,
    first_glyph: *mut hb_codepoint_t,
    glyph_stride: c_uint,
    user_data: *mut c_void,
) -> c_uint;
pub type hb_font_get_glyph_advance_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_position_t;
pub type hb_font_get_glyph_h_advance_func_t = hb_font_get_glyph_advance_func_t;
pub type hb_font_get_glyph_v_advance_func_t = hb_font_get_glyph_advance_func_t;
pub type hb_font_get_glyph_advances_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    count: c_uint,
    first_glyph: *mut hb_codepoint_t,
    glyph_stride: c_uint,
    first_advance: *mut hb_position_t,
    advance_stride: c_uint,
    user_data: *mut c_void,
);
pub type hb_font_get_glyph_h_advances_func_t = hb_font_get_glyph_advances_func_t;
pub type hb_font_get_glyph_v_advances_func_t = hb_font_get_glyph_advances_func_t;
pub type hb_font_get_glyph_origin_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_glyph_h_origin_func_t = hb_font_get_glyph_origin_func_t;
pub type hb_font_get_glyph_v_origin_func_t = hb_font_get_glyph_origin_func_t;
pub type hb_font_get_glyph_extents_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    extents: *mut hb_glyph_extents_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_glyph_contour_point_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    point_index: c_uint,
    x: *mut hb_position_t,
    y: *mut hb_position_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_glyph_name_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    glyph: hb_codepoint_t,
    name: *mut c_char,
    size: c_uint,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_font_get_glyph_from_name_func_t = extern "C" fn(
    font: *mut hb_font_t,
    font_data: *mut c_void,
    name: *const c_char,
    len: c_int,
    glyph: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t;

extern "C" {
    pub fn hb_font_funcs_create() -> *mut hb_font_funcs_t;

    pub fn hb_font_funcs_get_empty() -> *mut hb_font_funcs_t;

    pub fn hb_font_funcs_reference(funcs: *mut hb_font_funcs_t) -> *mut hb_font_funcs_t;

    pub fn hb_font_funcs_destroy(funcs: *mut hb_font_funcs_t);

    pub fn hb_font_funcs_set_user_data(
        funcs: *mut hb_font_funcs_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;

    pub fn hb_font_funcs_get_user_data(
        funcs: *mut hb_font_funcs_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;

    pub fn hb_font_funcs_make_immutable(funcs: *mut hb_font_funcs_t);

    pub fn hb_font_funcs_is_immutable(funcs: *mut hb_font_funcs_t) -> hb_bool_t;

    pub fn hb_font_funcs_set_font_h_extents_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_font_h_extents_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_font_v_extents_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_font_v_extents_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_nominal_glyph_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_nominal_glyph_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_variation_glyph_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_variation_glyph_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_h_advance_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_advance_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_v_advance_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_advance_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_h_advances_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_advances_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_v_advances_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_advances_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_h_origin_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_h_origin_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_v_origin_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_v_origin_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_extents_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_extents_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_font_funcs_set_glyph_contour_point_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_contour_point_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );

    pub fn hb_font_funcs_set_glyph_name_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_name_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );

    pub fn hb_font_funcs_set_glyph_from_name_func(
        funcs: *mut hb_font_funcs_t,
        func: hb_font_get_glyph_from_name_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );

    pub fn hb_font_get_h_extents(
        funcs: *mut hb_font_t,
        extents: *mut hb_font_extents_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_v_extents(
        funcs: *mut hb_font_t,
        extents: *mut hb_font_extents_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_nominal_glyph(
        funcs: *mut hb_font_t,
        unicode: hb_codepoint_t,
        glyph: *mut hb_codepoint_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_variation_glyph(
        funcs: *mut hb_font_t,
        unicode: hb_codepoint_t,
        variation_selector: hb_codepoint_t,
        glyph: *mut hb_codepoint_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_h_advance(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
    ) -> hb_position_t;

    pub fn hb_font_get_glyph_v_advance(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
    ) -> hb_position_t;

    pub fn hb_font_get_glyph_h_advances(
        funcs: *mut hb_font_t,
        count: c_uint,
        first_glyph: *mut hb_codepoint_t,
        glyph_stride: c_uint,
        first_advance: *mut hb_codepoint_t,
        advance_stride: c_uint
    );

    pub fn hb_font_get_glyph_v_advances(
        funcs: *mut hb_font_t,
        count: c_uint,
        first_glyph: *mut hb_codepoint_t,
        glyph_stride: c_uint,
        first_advance: *mut hb_codepoint_t,
        advance_stride: c_uint
    );

    pub fn hb_font_get_glyph_h_origin(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_v_origin(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_extents(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
        extents: *mut hb_glyph_extents_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_contour_point(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
        point_index: c_uint,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_name(
        funcs: *mut hb_font_t,
        glyph: hb_codepoint_t,
        name: *mut c_char,
        size: c_uint,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_from_name(
        funcs: *mut hb_font_t,
        name: *const c_char,
        len: c_int,
        glyph: *mut hb_codepoint_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph(
        font: *mut hb_font_t,
        unicode: hb_codepoint_t,
        variation_selector: hb_codepoint_t,
        glyph: *mut hb_codepoint_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_extents_for_direction(
        font: *mut hb_font_t,
        direction: hb_direction_t,
        extents: *mut hb_glyph_extents_t,
    );

    pub fn hb_font_get_glyph_advance_for_direction(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    );

    pub fn hb_font_get_glyph_advances_for_direction(
        font: *mut hb_font_t,
        direction: hb_direction_t,
        count: c_uint,
        first_glyph: *mut hb_codepoint_t,
        glyph_stride: c_uint,
        first_advance: *mut hb_position_t,
        advance_stride: c_uint,
    );

    pub fn hb_font_get_glyph_origin_for_direction(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    );

    pub fn hb_font_add_glyph_origin_for_direction(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    );

    pub fn hb_font_subtract_glyph_origin_for_direction(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    );

    pub fn hb_font_get_glyph_extents_for_origin(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        direction: hb_direction_t,
        extents: *mut hb_glyph_extents_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_glyph_contour_point_for_origin(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        point_index: c_uint,
        direction: hb_direction_t,
        x: *mut hb_position_t,
        y: *mut hb_position_t,
    ) -> hb_bool_t;

    pub fn hb_font_glyph_to_string(
        font: *mut hb_font_t,
        glyph: hb_codepoint_t,
        s: *mut c_char,
        size: c_uint,
    );

    pub fn hb_font_glyph_from_string(
        font: *mut hb_font_t,
        s: *const c_char,
        len: c_int,
        glyph: *mut hb_codepoint_t,
    );

    pub fn hb_font_create(face: *mut hb_face_t) -> *mut hb_font_t;

    pub fn hb_font_create_sub_font(parent: *mut hb_font_t) -> *mut hb_font_t;

    pub fn hb_font_get_empty() -> *mut hb_font_t;

    pub fn hb_font_reference(font: *mut hb_font_t) -> *mut hb_font_t;

    pub fn hb_font_destroy(font: *mut hb_font_t);

    pub fn hb_font_set_user_data(
        font: *mut hb_font_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;

    pub fn hb_font_get_user_data(
        font: *mut hb_font_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;

    pub fn hb_font_make_immutable(font: *mut hb_font_t);

    pub fn hb_font_is_immutable(font: *mut hb_font_t) -> hb_bool_t;

    /// Sets parent font of `font`.
    pub fn hb_font_set_parent(font: *mut hb_font_t, parent: *mut hb_font_t);

    pub fn hb_font_get_parent(font: *mut hb_font_t) -> *mut hb_font_t;

    /// Sets font-face of `font`.
    pub fn hb_font_set_face(font: *mut hb_font_t, face: *mut hb_face_t);

    pub fn hb_font_get_face(font: *mut hb_font_t) -> *mut hb_face_t;

    pub fn hb_font_set_funcs(
        font: *mut hb_font_t,
        klass: *mut hb_font_funcs_t,
        font_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );

    pub fn hb_font_set_funcs_data(
        font: *mut hb_font_t,
        font_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );

    pub fn hb_font_set_scale(
        font: *mut hb_font_t,
        x_scale: c_int,
        y_scale: c_int,
    );

    pub fn hb_font_get_scale(
        font: *mut hb_font_t,
        x_scale: *mut c_int,
        y_scale: *mut c_int,
    );

    pub fn hb_font_set_ppem(
        font: *mut hb_font_t,
        x_ppem: c_uint,
        y_ppem: c_uint,
    );

    pub fn hb_font_get_ppem(
        font: *mut hb_font_t,
        x_ppem: *mut c_uint,
        y_ppem: *mut c_uint,
    );

    /// Sets "point size" of the font.
    pub fn hb_font_set_ptem(font: *mut hb_font_t, ptem: c_float);

    /// Gets the "point size" of the font. A value of 0 means unset.
    pub fn hb_font_get_ptem(font: *mut hb_font_t) -> c_float;

    pub fn hb_font_set_variations(
        font: *mut hb_font_t,
        variations: *const hb_variation_t,
        variations_length: c_uint,
    );
    pub fn hb_font_set_var_coords_design(
        font: *mut hb_font_t,
        coords: *const c_float,
        coords_length: c_uint,
    );

    pub fn hb_font_set_var_coords_normalized(
        font: *mut hb_font_t,
        coords: *const c_int,
        coords_length: c_uint,
    );

    /// Return value is valid as long as variation coordinates of the font are
    /// not modified.
    pub fn hb_font_get_var_coords_normalized(
        font: *mut hb_font_t,
        length: *mut c_uint,
    ) -> *const c_int;
}
