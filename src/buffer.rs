use cffi::Ptr;
use std::{
    mem,
    ops::Range,
    os::raw,
};

use crate::*;

#[repr(transparent)]
pub struct Buffer(Ptr<ffi::hb_buffer_t>);
impl_ptr!(Buffer, ffi::hb_buffer_t);

pub type GlyphInfo = ffi::hb_glyph_info_t;
pub type GlyphPosition = ffi::hb_glyph_position_t;

impl cffi::Alloc for ffi::hb_buffer_t {
    fn free(ptr: *mut ffi::hb_buffer_t) {
        unsafe {
            ffi::hb_buffer_destroy(ptr)
        }
    }
}

impl Buffer {
    /// Create a new buffer with all properties set to defaults.
    pub fn new() -> Buffer {
        unsafe {
            let raw = ffi::hb_buffer_create();
            if 0 == ffi::hb_buffer_allocation_successful(raw) {
                panic!("Cannot allocate hb_buffer_t: out of memory");
            }
            Buffer(Ptr::from_raw(raw))
        }
    }

    /// Create a new buffer with all properties set to defaults and specified
    /// capacity.
    pub fn with_capacity(capacity: usize) -> Buffer {
        let mut buf = Self::new();
        buf.reserve(capacity);
        buf
    }
}

impl ffi::hb_buffer_t {
    /// Reserve space for at least `capacity` items.
    ///
    /// Note that unlike [`Vec`] `capacity` is the _total_ capacity, not space
    /// for additional elements.
    pub fn reserve(&mut self, capacity: usize) {
        unsafe {
            ffi::hb_buffer_pre_allocate(self, capacity as raw::c_uint);
            if 0 == ffi::hb_buffer_allocation_successful(self) {
                panic!("Cannot reserve space for hb_buffer_t: out of memory");
            }
        }
    }

    /// Resets the buffer to its initial status, as if it was just newly created
    /// with [`new()`].
    pub fn reset(&mut self) {
        unsafe { ffi::hb_buffer_reset(self) }
    }

    /// Similar to [`hb_buffer_reset()`], but does not clear the Unicode
    /// functions and the replacement code point.
    pub fn clear_contents(&mut self) {
        unsafe { ffi::hb_buffer_clear_contents(self) }
    }

    /// Return the number of items in the buffer.
    pub fn len(&self) -> usize {
        // XXX: In 1.8.8 hb_buffer_get_length is constant.
        unsafe { ffi::hb_buffer_get_length(mem::transmute(self)) as usize }
    }

    /// Set the text flow direction of the buffer. No shaping can happen without
    /// setting buffer direction, and it controls the visual direction for the
    /// output glyphs; for RTL direction the glyphs will be reversed. Many layout
    /// features depend on the proper setting of the direction, for example,
    /// reversing RTL text before shaping, then shaping with LTR direction is not
    /// the same as keeping the text in logical order and shaping with RTL
    /// direction.
    pub fn set_direction(&mut self, dir: Direction) {
        unsafe { ffi::hb_buffer_set_direction(self, dir) }
    }

    pub fn direction(&self) -> Direction {
        // XXX: In 1.8.8 hb_buffer_get_direction is constant.
        #[allow(mutable_transmutes)]
        unsafe { ffi::hb_buffer_get_direction(mem::transmute(self)) }
    }

    /// Sets unset buffer segment properties based on buffer Unicode contents.
    /// If buffer is not empty, it must have content type [`UNICODE`].
    ///
    /// If buffer script is not set (ie. is [`SCRIPT_INVALID`]), it will be set
    /// to the Unicode script of the first character in the buffer that has
    /// a script other than [`SCRIPT_COMMON`], [`SCRIPT_INHERITED`],
    /// and [`SCRIPT_UNKNOWN`].
    ///
    /// Next, if buffer direction is not set (ie. is [`DIRECTION_INVALID`]),
    /// it will be set to the natural horizontal direction of the buffer script
    /// as returned by [`hb_script_get_horizontal_direction()`].
    /// If [`hb_script_get_horizontal_direction()`] returns [`DIRECTION_INVALID`],
    /// then [`DIRECTION_LTR`] is used.
    ///
    /// Finally, if buffer language is not set (ie. is [`LANGUAGE_INVALID`]),
    /// it will be set to the process's default language as returned by
    /// [`hb_language_get_default()`]. This may change in the future by taking
    /// buffer script into consideration when choosing a language.
    pub fn guess_segment_properties(&mut self) {
        unsafe { ffi::hb_buffer_guess_segment_properties(self) }
    }

    /// See [`add_codepoints()`].
    ///
    /// Replaces invalid UTF-8 characters with the `buffer` replacement code
    /// point, see [`set_replacement_codepoint()`].
    pub fn add_utf8(&mut self, codepoints: &str, slice: Range<usize>) {
        let bytes = codepoints.as_bytes();
        unsafe {
            ffi::hb_buffer_add_utf8(
                self,
                bytes.as_ptr() as *const raw::c_char,
                bytes.len() as raw::c_int,
                slice.start as raw::c_uint,
                slice.end as raw::c_int,
            )
        }
    }

    /// See [`add_codepoints()`].
    ///
    /// Replaces invalid UTF-16 characters with the `buffer` replacement code
    /// point, see [`set_replacement_codepoint()`].
    pub fn add_utf16(&mut self, codepoints: &[u16], slice: Range<usize>) {
        unsafe {
            ffi::hb_buffer_add_utf16(
                self,
                codepoints.as_ptr(),
                codepoints.len() as raw::c_int,
                slice.start as raw::c_uint,
                slice.end as raw::c_int,
            )
        }
    }

    /// See [`add_codepoints()`].
    ///
    /// Replaces invalid UTF-32 characters with the `buffer` replacement code
    /// point, see [`set_replacement_codepoint()`].
    pub fn add_utf32(&mut self, codepoints: &[char], slice: Range<usize>) {
        unsafe {
            ffi::hb_buffer_add_utf32(
                self,
                codepoints.as_ptr() as *const u32,
                codepoints.len() as raw::c_int,
                slice.start as raw::c_uint,
                slice.end as raw::c_int,
            )
        }
    }

    /// Similar to [`add_codepoints()`], but allows only access
    /// to first 256 Unicode code points that can fit in 8-bit strings.
    ///
    /// Note: Has nothing to do with non-Unicode Latin-1 encoding.
    pub fn add_latin1(&mut self, codepoints: &[u8], slice: Range<usize>) {
        unsafe {
            ffi::hb_buffer_add_latin1(
                self,
                codepoints.as_ptr(),
                codepoints.len() as raw::c_int,
                slice.start as raw::c_uint,
                slice.end as raw::c_int,
            )
        }
    }

    /// Appends characters from `text` array to `buffer`. The `item_offset`
    /// is the position of the first character from `text` that will
    /// be appended, and `item_length` is the number of character. When shaping
    /// part of a larger text (e.g. a run of text from a paragraph), instead
    /// of passing just the substring corresponding to the run, it is preferable
    /// to pass the whole paragraph and specify the run start and length
    /// as `item_offset` and `item_length`, respectively, to give HarfBuzz
    /// the full context to be able, for example, to do cross-run Arabic shaping
    /// or properly handle combining marks at stat of run.
    ///
    /// This function does not check the validity of `text`, it is up to
    /// the caller to ensure it contains a valid Unicode code points.
    pub fn add_codepoints(&mut self, codepoints: &[Codepoint], slice: Range<usize>) {
        unsafe {
            ffi::hb_buffer_add_codepoints(
                self,
                codepoints.as_ptr(),
                codepoints.len() as raw::c_int,
                slice.start as raw::c_uint,
                slice.end as raw::c_int,
            )
        }
    }

    /// Append (part of) contents of another buffer to this buffer.
    pub fn append(&mut self, other: &ffi::hb_buffer_t, slice: Range<usize>) {
        unsafe {
            ffi::hb_buffer_append(
                self,
                // In 1.8.8 `other` is never touched, so this is safe.
                mem::transmute(other),
                slice.start as raw::c_uint,
                slice.end as raw::c_uint,
            )
        }
    }

    /// Reorders a glyph buffer to have canonical in-cluster glyph
    /// order / position. The resulting clusters should behave identical
    /// to pre-reordering clusters.
    ///
    /// Note: This has nothing to do with Unicode normalization.
    pub fn normalize_glyphs(&mut self) {
        unsafe { ffi::hb_buffer_normalize_glyphs(self) }
    }

    /// Get glyph informations.
    pub fn infos(&self) -> &[GlyphInfo] {
        let mut len = 0;
        unsafe {
            // XXX: In 1.8.8 this function is constant.
            let buf = ffi::hb_buffer_get_glyph_infos(mem::transmute(self), &mut len);
            ::std::slice::from_raw_parts(buf, len as usize)
        }
    }

    /// Get glyph positions.
    pub fn positions(&self) -> &[GlyphPosition] {
        let mut len = 0;
        unsafe {
            // XXX: While this function is not technically constant, in 1.8.8
            // all it does is ensure that the returned array is correct (which
            // might mutate internal state). This is similar to how interior
            // mutability works in Rust.
            let buf = ffi::hb_buffer_get_glyph_positions(mem::transmute(self), &mut len);
            ::std::slice::from_raw_parts(buf, len as usize)
        }
    }

    /// Get iterator over [`infos()`] and [`positions()`].
    pub fn iter(&self) -> impl Iterator<Item = (&GlyphInfo, &GlyphPosition)> {
        self.infos().iter().zip(self.positions())
    }

    /// Reverse buffer contents.
    pub fn reverse(&mut self) {
        unsafe { ffi::hb_buffer_reverse(self) }
    }

    /// Reverse buffer contents between start to end.
    pub fn reverse_range(&mut self, range: Range<usize>) {
        unsafe {
            ffi::hb_buffer_reverse_range(
                self, range.start as raw::c_uint, range.end as raw::c_uint)
        }
    }

    /// Reverse buffer clusters. That is, the buffer contents are reversed,
    /// then each cluster (consecutive items having the same cluster number) are
    /// reversed again.
    pub fn reverse_clusters(&mut self) {
        unsafe { ffi::hb_buffer_reverse_clusters(self) }
    }
}
