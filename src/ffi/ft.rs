//! Bindings to `hb-ft.h`.

use freetype::ffi::FT_Face;
use std::os::raw::*;

use crate::ffi::{
    common::*,
    face::*,
    font::*,
};

extern "C" {
    /// This one creates a new hb-face for given ft-face. When the returned
    /// hb-face is destroyed, the destroy callback is called (if not NULL), with
    /// the ft-face passed to it.
    ///
    /// The client is responsible to make sure that ft-face is destroyed after
    /// hb-face is destroyed.
    ///
    /// Most often you don't want this function. You should use either
    /// [`hb_ft_face_create_cached()`], or [`hb_ft_face_create_referenced()`].
    /// In particular, if you are going to pass NULL as destroy, you probably
    /// should use (the more recent) [`hb_ft_face_create_referenced()`] instead.
    pub fn hb_ft_face_create(
        ft_face: FT_Face,
        destroy: hb_destroy_func_t,
    ) -> *mut hb_face_t;

    /// This version is like [`hb_ft_face_create()`], except that it caches
    /// the hb-face using the generic pointer of the ft-face. This means
    /// that subsequent calls to this function with the same ft-face will
    /// return the same hb-face (correctly referenced).
    ///
    /// Client is still responsible for making sure that ft-face is destroyed
    /// after hb-face is.
    pub fn hb_ft_face_create_cached(ft_face: FT_Face) -> *mut hb_face_t;

    /// This version is like [`hb_ft_face_create()`], except that it calls
    /// [`FT_Reference_Face()`] on ft-face, as such keeping ft-face alive
    /// as long as the hb-face is.
    ///
    /// This is the most convenient version to use. Use it unless you have
    /// very good reasons not to.
    pub fn hb_ft_face_create_referenced(ft_face: FT_Face) -> *mut hb_face_t;

    /// hb-font from ft-face.
    ////

    /// Note:
    ///
    /// Set face size on ft-face before creating hb-font from it.
    /// Otherwise hb-ft would NOT pick up the font size correctly.
    ////

    /// See notes on [`hb_ft_face_create()`]. Same issues re
    /// lifecycle-management apply here. Use [`hb_ft_font_create_referenced()`]
    /// if you can.
    pub fn hb_ft_font_create(
        ft_face: FT_Face,
        destroy: hb_destroy_func_t,
    ) -> *mut hb_font_t;

    /// See notes on [`hb_ft_face_create_referenced()`] re lifecycle-management
    /// issues.
    pub fn hb_ft_font_create_referenced(ft_face: FT_Face) -> *mut hb_font_t;

    pub fn hb_ft_font_get_face(font: *mut hb_font_t) -> FT_Face;

    pub fn hb_ft_font_set_load_flags(font: *mut hb_font_t, load_flags: c_int);

    pub fn hb_ft_font_get_load_flags(font: *mut hb_font_t) -> c_int;

    /// Call when size or variations settings on underlying [`FT_Face`] change.
    pub fn hb_ft_font_changed(font: *mut hb_font_t);

    /// Makes an [`hb_font_t`] use FreeType internally to implement font
    /// functions.
    ///
    /// Note: this internally creates an FT_Face. Use it when you create your
    /// [`hb_face_t`] using [`hb_face_create()`].
    pub fn hb_ft_font_set_funcs(font: *mut hb_font_t);
}
