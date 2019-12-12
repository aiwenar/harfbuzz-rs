use std::{
    mem,
    os::raw,
    slice,
};
use cffi::Ptr;

use crate::*;

pub struct Font(Ptr<ffi::hb_font_t>);
impl_ptr!(Font, ffi::hb_font_t);

pub type FontFuncs = Ptr<ffi::hb_font_funcs_t>;

impl cffi::Alloc for ffi::hb_font_t {
    fn free(this: *mut Self) {
        unsafe { ffi::hb_font_destroy(this) }
    }
}

impl cffi::Alloc for ffi::hb_font_funcs_t {
    fn free(this: *mut Self) {
        unsafe { ffi::hb_font_funcs_destroy(this) }
    }
}

impl Font {
    pub fn create(face: Face) -> Font {
        unsafe {
            let ptr = ffi::hb_font_create(face.into_raw());
            Font(Ptr::from_raw(ptr))
        }
    }

    pub fn into_raw(self) -> *mut ffi::hb_font_t {
        Ptr::into_raw(self.0)
    }

    pub fn as_raw(&mut self) -> *mut ffi::hb_font_t {
        Ptr::as_raw(&mut self.0)
    }

    pub fn as_ptr(&self) -> *const ffi::hb_font_t {
        Ptr::as_ptr(&self.0)
    }
}

// unimplemented:
// - hb_font_get_glyph_h_advances
// - hb_font_get_glyph_v_advances
// - hb_font_get_glyph_name
// - hb_font_get_glyph_advances_for_direction
// - hb_font_glyph_to_string
// - hb_font_get_user_data
// - hb_font_set_user_data
// - hb_font_set_funcs
// - hb_font_set_funcs_data
impl ffi::hb_font_t {
    pub fn h_extents(&mut self) -> Option<FontExtents> {
        let mut extents = Default::default();
        let r = unsafe {
            ffi::hb_font_get_h_extents(self, &mut extents)
        };
        if r != 0 { Some(extents) } else { None }
    }

    pub fn v_extents(&mut self) -> Option<FontExtents> {
        let mut extents = Default::default();
        let r = unsafe {
            ffi::hb_font_get_v_extents(self, &mut extents)
        };
        if r != 0 { Some(extents) } else { None }
    }

    pub fn glyph(
        &mut self,
        unicode: Codepoint,
        variation_selector: Codepoint,
    ) -> Option<Codepoint> {
        let mut cp = 0;
        let r = unsafe {
            ffi::hb_font_get_glyph(self, unicode, variation_selector, &mut cp)
        };
        if r != 0 { Some(cp) } else { None }
    }

    pub fn nominal_glyph(
        &mut self,
        unicode: Codepoint,
    ) -> Option<Codepoint> {
        let mut cp = 0;
        let r = unsafe {
            ffi::hb_font_get_nominal_glyph(self, unicode, &mut cp)
        };
        if r != 0 { Some(cp) } else { None }
    }

    pub fn variation_glyph(
        &mut self,
        unicode: Codepoint,
        variation_selector: Codepoint,
    ) -> Option<Codepoint> {
        let mut cp = 0;
        let r = unsafe {
            ffi::hb_font_get_variation_glyph(self, unicode, variation_selector, &mut cp)
        };
        if r != 0 { Some(cp) } else { None }
    }

    pub fn glyph_h_advance(&mut self, cp: Codepoint) -> Position {
        unsafe { ffi::hb_font_get_glyph_h_advance(self, cp) }
    }

    pub fn glyph_v_advance(&mut self, top: Codepoint) -> Position {
        unsafe { ffi::hb_font_get_glyph_v_advance(self, top) }
    }

    pub fn glyph_h_origin(&mut self, glyph: Codepoint)
    -> Option<(Position, Position)> {
        let mut x = 0;
        let mut y = 0;
        let r = unsafe {
            ffi::hb_font_get_glyph_h_origin(self, glyph, &mut x, &mut y)
        };
        if r != 0 { Some((x, y)) } else { None }
    }

    pub fn glyph_v_origin(&mut self, glyph: Codepoint)
    -> Option<(Position, Position)> {
        let mut x = 0;
        let mut y = 0;
        let r = unsafe {
            ffi::hb_font_get_glyph_v_origin(self, glyph, &mut x, &mut y)
        };
        if r != 0 { Some((x, y)) } else { None }
    }

    pub fn glyph_extents(&mut self, glyph: Codepoint) -> Option<GlyphExtents> {
        let mut extents = Default::default();
        let r = unsafe {
            ffi::hb_font_get_glyph_extents(self, glyph, &mut extents)
        };
        if r != 0 { Some(extents) } else { None }
    }

    pub fn glyph_contour_point(&mut self, glyph: Codepoint, index: u32)
    -> Option<(Position, Position)> {
        let mut x = 0;
        let mut y = 0;
        let r = unsafe {
            ffi::hb_font_get_glyph_contour_point(self, glyph, index, &mut x, &mut y)
        };
        if r != 0 { Some((x, y)) } else { None }
    }

    pub fn glyph_from_name(&mut self, name: &str) -> Option<Codepoint> {
        let bytes = name.as_bytes();
        let mut cp = 0;
        let r = unsafe {
            ffi::hb_font_get_glyph_from_name(
                self,
                bytes.as_ptr() as *const raw::c_char,
                bytes.len() as raw::c_int,
                &mut cp,
            )
        };
        if r != 0 { Some(cp) } else { None }
    }

    pub fn extents_for_direction(&mut self, direction: Direction) -> GlyphExtents {
        let mut extents = Default::default();
        unsafe {
            ffi::hb_font_get_extents_for_direction(self, direction, &mut extents);
        }
        extents
    }

    pub fn glyph_advance_for_direction(
        &mut self,
        glyph: Codepoint,
        direction: Direction,
    ) -> (Position, Position) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            ffi::hb_font_get_glyph_advance_for_direction(
                self, glyph, direction, &mut x, &mut y);
        }
        (x, y)
    }

    pub fn get_glyph_origin_for_direction(
        &mut self,
        glyph: Codepoint,
        direction: Direction,
    ) -> (Position, Position) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            ffi::hb_font_get_glyph_origin_for_direction(
                self, glyph, direction, &mut x, &mut y);
        }
        (x, y)
    }

    pub fn add_glyph_origin_for_direction(
        &mut self,
        glyph: Codepoint,
        direction: Direction,
    ) -> (Position, Position) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            ffi::hb_font_add_glyph_origin_for_direction(
                self, glyph, direction, &mut x, &mut y);
        }
        (x, y)
    }

    pub fn subtract_glyph_origin_for_direction(
        &mut self,
        glyph: Codepoint,
        direction: Direction,
    ) -> (Position, Position) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            ffi::hb_font_subtract_glyph_origin_for_direction(
                self, glyph, direction, &mut x, &mut y);
        }
        (x, y)
    }

    pub fn glyph_extents_for_origin(
        &mut self,
        glyph: Codepoint,
        direction: Direction,
    ) -> Option<GlyphExtents> {
        let mut extents = Default::default();
        let r = unsafe {
            ffi::hb_font_get_glyph_extents_for_origin(
                self, glyph, direction, &mut extents)
        };
        if r != 0 { Some(extents) } else { None }
    }

    pub fn glyph_contour_point_for_origin(
        &mut self,
        glyph: Codepoint,
        index: u32,
        direction: Direction,
    ) -> Option<(Position, Position)> {
        let mut x = 0;
        let mut y = 0;
        let r = unsafe {
            ffi::hb_font_get_glyph_contour_point_for_origin(
                self, glyph, index, direction, &mut x, &mut y)
        };
        if r != 0 { Some((x, y)) } else { None }
    }

    pub fn glyph_from_string(&mut self, s: &str) -> Codepoint {
        let bytes = s.as_bytes();
        let mut cp = 0;
        unsafe {
            ffi::hb_font_glyph_from_string(
                self,
                bytes.as_ptr() as *const raw::c_char,
                bytes.len() as raw::c_int,
                &mut cp,
            );
        }
        cp
    }

    // TODO: does this need to be mutable?
    pub fn create_sub_font(&mut self) -> Font {
        unsafe {
            let ptr = ffi::hb_font_create_sub_font(self);
            Font(Ptr::from_raw(ptr))
        }
    }

    pub fn is_immutable(&self) -> bool {
        // XXX: As of 1.8.8 hb_font_is_immutable is constant, so this is safe.
        unsafe { ffi::hb_font_is_immutable(mem::transmute(self)) != 0 }
    }

    pub fn make_immutable(&mut self) {
        unsafe { ffi::hb_font_make_immutable(self) }
    }

    pub fn parent(&self) -> &ffi::hb_font_t {
        // XXX: since we're returning a non-mutable reference there should
        // be no problems with aliasing.
        unsafe { mem::transmute(ffi::hb_font_get_parent(mem::transmute(self))) }
    }

    /// Sets parent font of `font`.
    pub fn set_parent(&mut self, parent: Font) {
        unsafe { ffi::hb_font_set_parent(self, parent.into_raw()) }
    }

    pub fn face(&self) -> &ffi::hb_face_t {
        // XXX: since we're returning a non-mutable reference there should
        // be no problems with aliasing.
        unsafe { mem::transmute(ffi::hb_font_get_face(mem::transmute(self))) }
    }

    /// Sets font-face of `font`.
    pub fn set_face(&mut self, face: Face) {
        unsafe { ffi::hb_font_set_face(self, face.into_raw()) }
    }

    pub fn scale(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            // XXX: As of 1.8.8 hb_font_get_scale is constant, so this is safe.
            ffi::hb_font_get_scale(mem::transmute(self), &mut x, &mut y);
        }
        (x, y)
    }

    pub fn set_scale(&mut self, (x, y): (i32, i32)) {
        unsafe { ffi::hb_font_set_scale(self, x, y) }
    }

    pub fn ppem(&self) -> (u32, u32) {
        let mut x = 0;
        let mut y = 0;
        unsafe {
            // XXX: As of 1.8.8 hb_font_get_ppem is constant, so this is safe.
            ffi::hb_font_get_ppem(mem::transmute(self), &mut x, &mut y);
        }
        (x, y)
    }

    pub fn set_ppem(&mut self, (x, y): (u32, u32)) {
        unsafe { ffi::hb_font_set_ppem(self, x, y) }
    }

    /// Sets “point size” of the font.
    pub fn set_ptem(&mut self, ptem: f32) {
        unsafe { ffi::hb_font_set_ptem(self, ptem) }
    }

    /// Gets the “point size” of the font. A value of 0 means unset.
    pub fn ptem(&self) -> f32 {
        // XXX: As of 1.8.8 hb_font_get_ptem is constant, so this is safe.
        unsafe { ffi::hb_font_get_ptem(mem::transmute(self)) }
    }

    pub fn set_variations(&mut self, variations: &[Variation]) {
        unsafe {
            ffi::hb_font_set_variations(
                self,
                variations.as_ptr(),
                variations.len() as raw::c_uint,
            )
        }
    }

    pub fn set_var_coords_normalized(&mut self, coords: &[i32]) {
        unsafe {
            ffi::hb_font_set_var_coords_normalized(
                self,
                coords.as_ptr(),
                coords.len() as raw::c_uint,
            )
        }
    }

    pub fn var_coords_normalized(&self) -> &[i32] {
        let mut len = 0;
        unsafe {
            let buf = ffi::hb_font_get_var_coords_normalized(
                // XXX: Since as of 1.8.8 hb_font_get_var_coords_normalized
                // is constant, and we're returning an immutable reference,
                // this should be safe.
                mem::transmute(self),
                &mut len,
            );
            slice::from_raw_parts(buf, len as usize)
        }
    }
}

#[cfg(feature = "freetype")]
use freetype;

#[cfg(feature = "freetype")]
impl Font {
    pub fn from_ft_face(mut face: freetype::Face) -> Font {
        face.reference();
        unsafe {
            let ptr = ffi::hb_ft_font_create_referenced(face.raw_mut());
            Font(Ptr::from_raw(ptr))
        }
    }
}

#[cfg(feature = "freetype")]
impl ffi::hb_font_t {
    /// Call when size or variations settings on underlying [`FT_Face`] change.
    pub fn ft_font_changed(&mut self) {
        unsafe { ffi::hb_ft_font_changed(self) }
    }

    /// Makes a [`Font`] use FreeType internally to implement font functions.
    ///
    /// Note: this internally creates an [`FT_Face`]. Use it when you create
    /// your [`Face`] using [`Face::create()`].
    pub fn ft_set_funcs(&mut self) {
        unsafe { ffi::hb_ft_font_set_funcs(self) }
    }
}

// unimplemented:
// - hb_font_funcs_set_user_data
// - hb_font_funcs_get_user_data
impl ffi::hb_font_funcs_t {
    pub fn new() -> FontFuncs {
        unsafe {
            let ptr = ffi::hb_font_funcs_create();
            Ptr::from_raw(ptr)
        }
    }

    pub fn empty() -> FontFuncs {
        unsafe {
            let ptr = ffi::hb_font_funcs_get_empty();
            Ptr::from_raw(ptr)
        }
    }

    pub fn is_immutable(&self) -> bool {
        // XXX: As of 1.8.8 hb_font_funcs_is_immutable is constant, so this
        // is safe.
        unsafe { ffi::hb_font_funcs_is_immutable(mem::transmute(self)) != 0 }
    }

    pub fn make_immutable(&mut self) {
        unsafe { ffi::hb_font_funcs_make_immutable(self) }
    }
}
