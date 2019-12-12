//! Bindings to `hb-face.h`.

#![allow(non_camel_case_types)]

use std::os::raw::*;

use crate::ffi::{
    blob::*,
    common::*,
    set::*,
};

#[repr(C)]
pub struct hb_face_t(cffi::Private);

pub type hb_reference_table_func_t = extern "C" fn(
    face: *mut hb_face_t,
    tag: hb_tag_t,
    user_data: *mut c_void,
);

extern "C" {
    /// Get number of faces on the blob.
    pub fn hb_face_count(blob: *mut hb_blob_t) -> c_uint;

    pub fn hb_face_create(blob: *mut hb_blob_t, index: c_uint) -> *mut hb_face_t;

    pub fn hb_face_create_for_tables(
        reference_table_func: hb_reference_table_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    ) -> *mut hb_face_t;

    pub fn hb_face_get_empty() -> *mut hb_face_t;

    pub fn hb_face_reference(face: *mut hb_face_t) -> *mut hb_face_t;
    pub fn hb_face_destroy(face: *mut hb_face_t);
    pub fn hb_face_set_user_data(
        face: *mut hb_face_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;
    pub fn hb_face_get_user_data(
        face: *mut hb_face_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;
    pub fn hb_face_make_immutable(face: *mut hb_face_t);
    pub fn hb_face_is_immutable(face: *const hb_face_t) -> hb_bool_t;
    pub fn hb_face_reference_table(face: *mut hb_face_t, tag: hb_tag_t) -> *mut hb_blob_t;
    pub fn hb_face_reference_blob(face: *mut hb_face_t) -> *mut hb_blob_t;
    pub fn hb_face_set_index(face: *mut hb_face_t, index: c_uint);
    pub fn hb_face_get_index(face: *const hb_face_t) -> c_uint;
    pub fn hb_face_set_upem(face: *mut hb_face_t, upem: c_uint);
    pub fn hb_face_get_upem(face: *const hb_face_t) -> c_uint;
    pub fn hb_face_set_glyph_count(face: *mut hb_face_t, count: c_uint);
    pub fn hb_face_get_glyph_count(face: *const hb_face_t) -> c_uint;

    /// Retrieves table tags for a face, if possible.
    /* Since: 1.6.0 */
    pub fn hb_face_get_table_tags(
        face: *const hb_face_t,
        start_offset: c_uint,
        table_count: *mut c_uint,
        table_tag: *mut hb_tag_t,
    ) -> c_uint;

    pub fn hb_face_collect_unicodes(face: *mut hb_face_t, out: *mut hb_set_t);

    pub fn hb_face_collect_variation_selectors(face: *mut hb_face_t, out: *mut hb_set_t);
    pub fn hb_face_collect_variation_unicodes(
        face: *mut hb_face_t,
        variation_selector: hb_codepoint_t,
        out: *mut hb_set_t,
    );

    /// Creates a `hb_face_t` that can be used with
    /// [`hb_face_builder_add_table()`]. After tables are added to the face,
    /// it can be compiled to a binary font file by calling
    /// [`hb_face_reference_blob()`].
    pub fn hb_face_builder_create() -> *mut hb_face_t;

    /// Add table for `tag` with data provided by `blob` to the face.
    /// `face` must be created using [`hb_face_builder_create()`].
    pub fn hb_face_builder_add_table(
        face: *mut hb_face_t,
        tag: hb_tag_t,
        blob: *mut hb_blob_t,
    );
}
