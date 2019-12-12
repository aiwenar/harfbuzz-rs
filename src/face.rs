use cffi::Ptr;
use std::{slice, os::raw};

use crate::*;

#[repr(transparent)]
pub struct Face(Ptr<ffi::hb_face_t>);
impl_ptr!(Face, ffi::hb_face_t);

impl cffi::Alloc for ffi::hb_face_t {
    fn free(this: *mut Self) {
        unsafe { ffi::hb_face_destroy(this) }
    }
}

/// Get number of faces on the blob
pub fn number_in_blob(blob: Blob) -> usize {
    unsafe { ffi::hb_face_count(blob.into_raw()) as usize }
}

// unimplemented:
// - hb_face_set_user_data
// - hb_face_get_user_data
impl Face {
    pub fn create(blob: Blob, index: u32) -> Face {
        unsafe {
            let ptr = ffi::hb_face_create(blob.into_raw(), index);
            Face(Ptr::from_raw(ptr))
        }
    }

    pub fn empty() -> Face {
        unsafe {
            let ptr = ffi::hb_face_get_empty();
            Face(Ptr::from_raw(ptr))
        }
    }

    pub fn into_raw(self) -> *mut ffi::hb_face_t {
        Ptr::into_raw(self.0)
    }

    pub fn as_raw(&mut self) -> *mut ffi::hb_face_t {
        Ptr::as_raw(&mut self.0)
    }

    pub fn as_ptr(&self) -> *const ffi::hb_face_t {
        Ptr::as_ptr(&self.0)
    }
}

impl ffi::hb_face_t {
    pub fn is_immutable(&self) -> bool {
        unsafe { ffi::hb_face_is_immutable(self) != 0 }
    }

    pub fn make_immutable(&mut self) {
        unsafe { ffi::hb_face_make_immutable(self) }
    }

    pub fn index(&self) -> u32 {
        unsafe { ffi::hb_face_get_index(self) }
    }

    pub fn set_index(&mut self, index: u32) {
        unsafe { ffi::hb_face_set_index(self, index) }
    }

    pub fn upem(&self) -> u32 {
        unsafe { ffi::hb_face_get_upem(self) }
    }

    pub fn set_upem(&mut self, upem: u32) {
        unsafe { ffi::hb_face_set_upem(self, upem) }
    }

    pub fn glyph_count(&self) -> u32 {
        unsafe { ffi::hb_face_get_glyph_count(self) }
    }

    pub fn set_glyph_count(&mut self, count: u32) {
        unsafe { ffi::hb_face_set_glyph_count(self, count) }
    }

    pub fn table_tags<'buf>(&self, buf: &'buf mut [Tag]) -> &'buf mut [Tag] {
        let mut len = buf.len() as raw::c_uint;
        let count = unsafe {
            ffi::hb_face_get_table_tags(self, 0, &mut len, buf.as_mut_ptr())
        };
        &mut buf[..count as usize]
    }

    /// Populate `out` with all Unicode codepoints covered by this face.
    pub fn collect_unicodes(&mut self, out: &mut ffi::hb_set_t) {
        unsafe { ffi::hb_face_collect_unicodes(self, out) }
    }

    /// Populate `out` with variation selector codepoints covered by this face.
    pub fn variant_selectors(&mut self, out: &mut ffi::hb_set_t) {
        unsafe { ffi::hb_face_collect_variation_selectors(self, out) }
    }

    pub fn variation_unicodes(&mut self, selector: Codepoint, out: &mut ffi::hb_set_t) {
        unsafe { ffi::hb_face_collect_variation_unicodes(self, selector, out) }
    }
}

#[cfg(feature = "freetype")]
use freetype;

#[cfg(feature = "freetype")]
impl Face {
    pub fn from_ft_face(face: &mut freetype::Face) -> Face {
        unsafe {
            freetype::ffi::FT_Reference_Face(face.raw_mut());
            let ptr = ffi::hb_ft_face_create_referenced(face.raw_mut());
            Face(Ptr::from_raw(ptr))
        }
    }
}

#[cfg(feature = "freetype")]
impl ffi::hb_font_t {
    pub fn ft_face(&mut self) -> freetype::ffi::FT_Face {
        unsafe { ffi::hb_ft_font_get_face(self) }
    }
}
