//! Bindings to `hb-unicode.h`.

#![allow(non_camel_case_types)]

use std::os::raw::*;

use crate::ffi::common::*;

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum hb_unicode_general_category_t {
    CONTROL,
    FORMAT,
    UNASSIGNED,
    PRIVATE_USE,
    SURROGATE,
    LOWERCASE_LETTER,
    MODIFIER_LETTER,
    OTHER_LETTER,
    TITLECASE_LETTER,
    UPPERCASE_LETTER,
    SPACING_MARK,
    ENCLOSING_MARK,
    NON_SPACING_MARK,
    DECIMAL_NUMBER,
    LETTER_NUMBER,
    OTHER_NUMBER,
    CONNECT_PUNCTUATION,
    DASH_PUNCTUATION,
    CLOSE_PUNCTUATION,
    FINAL_PUNCTUATION,
    INITIAL_PUNCTUATION,
    OTHER_PUNCTUATION,
    OPEN_PUNCTUATION,
    CURRENCY_SYMBOL,
    MODIFIER_SYMBOL,
    MATH_SYMBOL,
    OTHER_SYMBOL,
    LINE_SEPARATOR,
    PARAGRAPH_SEPARATOR,
    SPACE_SEPARATOR,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum hb_unicode_combining_class_t {
    NOT_REORDERED = 0,
    OVERLAY = 1,
    NUKTA = 7,
    KANA_VOICING = 8,
    VIRAMA = 9,
    CCC10 =  10,
    CCC11 =  11,
    CCC12 =  12,
    CCC13 =  13,
    CCC14 =  14,
    CCC15 =  15,
    CCC16 =  16,
    CCC17 =  17,
    CCC18 =  18,
    CCC19 =  19,
    CCC20 =  20,
    CCC21 =  21,
    CCC22 =  22,
    CCC23 =  23,
    CCC24 =  24,
    CCC25 =  25,
    CCC26 =  26,
    CCC27 =  27,
    CCC28 =  28,
    CCC29 =  29,
    CCC30 =  30,
    CCC31 =  31,
    CCC32 =  32,
    CCC33 =  33,
    CCC34 =  34,
    CCC35 =  35,
    CCC36 =  36,
    CCC84 =  84,
    CCC91 =  91,
    CCC103 = 103,
    CCC107 = 107,
    CCC118 = 118,
    CCC122 = 122,
    CCC129 = 129,
    CCC130 = 130,
    CCC133 = 132,
    ATTACHED_BELOW_LEFT = 200,
    ATTACHED_BELOW = 202,
    ATTACHED_ABOVE = 214,
    ATTACHED_ABOVE_RIGHT = 216,
    BELOW_LEFT = 218,
    BELOW = 220,
    BELOW_RIGHT = 222,
    LEFT = 224,
    RIGHT = 226,
    ABOVE_LEFT = 228,
    ABOVE = 230,
    ABOVE_RIGHT = 232,
    DOUBLE_BELOW = 233,
    DOUBLE_ABOVE = 234,
    IOTA_SUBSCRIPT = 240,
    INVALID = 255,
}

#[repr(C)]
pub struct hb_unicode_funcs_t(cffi::Private);

extern "C" {
    pub fn hb_unicode_funcs_get_default() -> *mut hb_unicode_funcs_t;
    pub fn hb_unicode_funcs_create(parent: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
    pub fn hb_unicode_funcs_get_empty() -> *mut hb_unicode_funcs_t;
    pub fn hb_unicode_funcs_reference(funcs: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
    pub fn hb_unicode_funcs_destroy(funcs: *mut hb_unicode_funcs_t);
    pub fn hb_unicode_funcs_set_user_data(
        funcs: *mut hb_unicode_funcs_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;
    pub fn hb_unicode_funcs_get_user_data(
        funcs: *mut hb_unicode_funcs_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;
    pub fn hb_unicode_funcs_make_immutable(funcs: *mut hb_unicode_funcs_t);
    pub fn hb_unicode_funcs_is_immutable(funcs: *mut hb_unicode_funcs_t) -> hb_bool_t;
    pub fn hb_unicode_funcs_get_parent(funcs: *mut hb_unicode_funcs_t) -> *mut hb_unicode_funcs_t;
}

pub type hb_unicode_combining_class_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    codepoint: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_unicode_combining_class_t;
pub type hb_unicode_eastasian_width_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    codepoint: hb_codepoint_t,
    user_data: *mut c_void,
) -> c_uint;
pub type hb_unicode_general_category_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    codepoint: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_unicode_general_category_t;
pub type hb_unicode_mirroring_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    codepoint: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_codepoint_t;
pub type hb_unicode_script_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    codepoint: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_script_t;
pub type hb_unicode_compose_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    a: hb_codepoint_t,
    b: hb_codepoint_t,
    ab: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t;
pub type hb_unicode_decompose_func_t = extern "C" fn(
    funcs: *mut hb_unicode_funcs_t,
    ab: hb_codepoint_t,
    a: *mut hb_codepoint_t,
    b: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t;

/// Fully decompose `u` to its Unicode compatibility decomposition.
/// The codepoints of the decomposition will be written to `decomposed`.
/// The complete length of the decomposition will be returned.
///
/// If `u` has no compatibility decomposition, zero should be returned.
///
/// The Unicode standard guarantees that a buffer of length
/// [`HB_UNICODE_MAX_DECOMPOSITION_LEN`] codepoints will always be sufficient
/// for any compatibility decomposition plus an terminating value of 0.
/// Consequently, `decompose` must be allocated by the caller to be at least
/// this length.  Implementations of this function type must ensure that they
/// do not write past the provided array.
///
/// Return value: number of codepoints in the full compatibility decomposition
/// of `u`, or 0 if no decomposition available.
///
/// ## Arguments
///
/// - `ufuncs`: a Unicode function structure
/// - `u`: codepoint to decompose
/// - `decomposed`: address of codepoint array (of length
///   [`HB_UNICODE_MAX_DECOMPOSITION_LEN`]) to write decomposition into
/// - `user_data`: user data pointer as passed
///   to [`hb_unicode_funcs_set_decompose_compatibility_func()`]
pub type hb_unicode_decompose_compatibility_func_t = extern "C" fn(
    funcs: &mut hb_unicode_funcs_t,
    u: hb_codepoint_t,
    decomposed: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> c_uint;

pub const HB_UNICODE_MAX_DECOMPOSITION_LEN: usize = 19;

extern "C" {
    pub fn hb_unicode_funcs_set_combining_class_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_combining_class_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_eastasian_width_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_eastasian_width_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_general_category_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_general_category_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_mirroring_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_mirroring_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_script_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_script_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_compose_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_compose_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_decompose_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_decompose_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_funcs_set_decompose_compatibility_func(
        funcs: *mut hb_unicode_funcs_t,
        func: hb_unicode_decompose_compatibility_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
    pub fn hb_unicode_combining_class(
        funcs: *mut hb_unicode_funcs_t,
        codepoint: hb_codepoint_t,
    ) -> hb_unicode_combining_class_t;
    pub fn hb_unicode_eastasian_width(
        funcs: *mut hb_unicode_funcs_t,
        codepoint: hb_codepoint_t,
    ) -> c_uint;
    pub fn hb_unicode_general_category(
        funcs: *mut hb_unicode_funcs_t,
        codepoint: hb_codepoint_t,
    ) -> hb_unicode_general_category_t;
    pub fn hb_unicode_mirroring(
        funcs: *mut hb_unicode_funcs_t,
        codepoint: hb_codepoint_t,
    ) -> hb_codepoint_t;
    pub fn hb_unicode_script(
        funcs: *mut hb_unicode_funcs_t,
        codepoint: hb_codepoint_t,
    ) -> hb_script_t;
    pub fn hb_unicode_compose(
        funcs: *mut hb_unicode_funcs_t,
        a: hb_codepoint_t,
        b: hb_codepoint_t,
        ab: *mut hb_codepoint_t,
    ) -> hb_bool_t;
    pub fn hb_unicode_decompose(
        funcs: *mut hb_unicode_funcs_t,
        ab: hb_codepoint_t,
        a: *mut hb_codepoint_t,
        b: *mut hb_codepoint_t,
    ) -> hb_bool_t;
    pub fn hb_unicode_decompose_compatibility(
        funcs: *mut hb_unicode_funcs_t,
        u: hb_codepoint_t,
        decomposed: *mut hb_codepoint_t
    ) -> c_uint;
}
